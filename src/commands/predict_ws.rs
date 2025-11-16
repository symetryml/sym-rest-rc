use crate::config;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize)]
struct DataFrame {
    #[serde(rename = "attributeNames")]
    attribute_names: Vec<String>,
    data: Vec<Vec<String>>,
    #[serde(rename = "attributeTypes")]
    attribute_types: Vec<String>,
}

#[derive(Serialize)]
struct WsHeaders {
    headers: Vec<String>,
    #[serde(rename = "extraKeys")]
    extra_keys: Vec<String>,
}

pub async fn handle_predict(
    project: String,
    model: String,
    df: Option<String>,
    file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Making prediction with project: {} and model: {} (using WebSocket)", project, model);

    // Check that either df or file is provided
    if df.is_none() && file.is_none() {
        return Err("Must specify either --df (JSON dataframe) or --file (data file)".into());
    }

    if df.is_some() && file.is_some() {
        return Err("Cannot specify both --df and --file. Choose one.".into());
    }

    // Parse the input data
    let (attribute_names, data, attribute_types) = if let Some(json_df) = df {
        println!("Using JSON dataframe: {}", json_df);
        let parsed: DataFrame = serde_json::from_str(&json_df)?;
        (parsed.attribute_names, parsed.data, parsed.attribute_types)
    } else if let Some(file_path) = file {
        println!("Reading data from file: {}", file_path);

        // Read CSV file
        let file_handle = File::open(&file_path)?;
        let reader = BufReader::new(file_handle);
        let mut lines = reader.lines();

        // Read header (first line)
        let header_line = lines.next()
            .ok_or("Empty CSV file")??;
        let attribute_names: Vec<String> = header_line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Read data rows
        let mut data: Vec<Vec<String>> = Vec::new();
        for line in lines {
            let line = line?;
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }
            let row: Vec<String> = line
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            data.push(row);
        }

        println!("Loaded {} rows with {} columns", data.len(), attribute_names.len());

        // For predict, we typically only have input features, so use "C" (continuous) for all
        // The user can provide types via --df if they need custom types
        let attribute_types: Vec<String> = vec!["C".to_string(); attribute_names.len()];

        (attribute_names, data, attribute_types)
    } else {
        unreachable!()
    };

    // Build DataFrame
    let dataframe = DataFrame {
        attribute_names,
        data,
        attribute_types,
    };

    // Convert DataFrame to JSON string (compact format)
    let dataframe_json = serde_json::to_string(&dataframe)?;

    // Build WebSocket URL
    let ws_url = format!(
        "ws://{}:{}/symetry/ws/predict",
        config::Config::host(),
        config::Config::port(),
    );

    // Calculate MD5 of payload
    let payload_md5_digest = md5::compute(dataframe_json.as_bytes());
    let payload_md5 = BASE64.encode(&payload_md5_digest.0);

    // Get current UTC time
    let now = chrono::Utc::now();
    let date_part = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let nanos = now.timestamp_subsec_nanos();
    let full_date = format!("{};{}", date_part, nanos);

    // Build STRING_TO_SIGN for WebSocket
    // Format: md5\nsecret_key\ndate\ncustomer_id\npayload\nurl\nextra_key1\nextra_key2\n
    // For predict, extraKeys are [project, model] (project first, then model)
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        payload_md5,
        &config::Config::secretkey(),
        full_date,
        config::Config::user(),
        dataframe_json,
        ws_url,
        project,
        model
    );

    // Compute HMAC-SHA256 signature
    let secret_bytes = BASE64.decode(&config::Config::secretkey())?;
    let mut mac = HmacSha256::new_from_slice(&secret_bytes)?;
    mac.update(string_to_sign.as_bytes());
    let signature = mac.finalize();
    let authorization = BASE64.encode(signature.into_bytes());

    // Build headers structure
    // Format: [timestamp, md5, authorization, customer-id]
    // extraKeys: [project, model] - project name first, then model id
    let ws_headers = WsHeaders {
        headers: vec![
            full_date.clone(),
            payload_md5.clone(),
            authorization.clone(),
            config::Config::user().to_string(),
        ],
        extra_keys: vec![project.clone(), model.clone()],
    };

    // Convert headers to JSON (compact format)
    let headers_json = serde_json::to_string(&ws_headers)?;

    // Build complete message: headers_json + dataframe_json
    let full_message = format!("{}{}", headers_json, dataframe_json);

    // Add length prefix (LENGTH OF HEADER JSON, not total message!)
    let header_length = headers_json.len();
    let complete_message = format!("{},{}", header_length, full_message);

    // Connect to WebSocket
    let (ws_stream, _) = connect_async(&ws_url).await?;

    let (mut write, mut read) = ws_stream.split();

    // Send the complete message
    write.send(Message::Text(complete_message)).await?;

    // Read responses
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                println!("WS Received: {}", text);

                // Check if this is a completion/done message
                if text.contains("\"statusCode\"") || text.contains("DONE") || text.contains("FINISHED") {
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                println!("WS Received binary data: {} bytes", data.len());
            }
            Ok(Message::Close(_)) => {
                println!("WebSocket closed by server");
                break;
            }
            Ok(Message::Ping(_)) => {
                println!("Received ping");
            }
            Ok(Message::Pong(_)) => {
                println!("Received pong");
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
