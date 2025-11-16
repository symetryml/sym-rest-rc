use crate::config;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize)]
struct DataFrame {
    #[serde(rename = "attributeNames")]
    attribute_names: Vec<String>,
    data: Vec<Vec<String>>,
    #[serde(rename = "attributeTypes")]
    attribute_types: Vec<String>,
    #[serde(rename = "errorHandling")]
    error_handling: i32,
}

#[derive(Serialize)]
struct WsHeaders {
    headers: Vec<String>,
    #[serde(rename = "extraKeys")]
    extra_keys: Vec<String>,
}

pub async fn handle_learn(
    project: String,
    file: String,
    types: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Learning from file: {} for project: {} (using WebSocket)", file, project);

    // Read CSV file
    let file_handle = File::open(&file)?;
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

    // Parse attribute types
    let attribute_types: Vec<String> = types
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Validate that the number of types matches the number of columns
    if attribute_types.len() != attribute_names.len() {
        return Err(format!(
            "Number of attribute types ({}) does not match number of columns ({})",
            attribute_types.len(),
            attribute_names.len()
        ).into());
    }

    println!("Loaded {} rows with {} columns", data.len(), attribute_names.len());

    // Build DataFrame
    let dataframe = DataFrame {
        attribute_names,
        data,
        attribute_types,
        error_handling: 1,
    };

    // Convert DataFrame to JSON string (compact format)
    let dataframe_json = serde_json::to_string(&dataframe)?;

    // Build WebSocket URL
    let ws_url = format!(
        "ws://{}:{}/symetry/ws/learn",
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

    // Build STRING_TO_SIGN for WebSocket (different format than REST!)
    // Format: md5\nsecret_key\ndate\ncustomer_id\npayload\nurl\nextra_key\n
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        payload_md5,
        &config::Config::secretkey(),
        full_date,
        config::Config::user(),
        dataframe_json,
        ws_url,
        project
    );

    // Compute HMAC-SHA256 signature
    let secret_bytes = BASE64.decode(&config::Config::secretkey())?;
    let mut mac = HmacSha256::new_from_slice(&secret_bytes)?;
    mac.update(string_to_sign.as_bytes());
    let signature = mac.finalize();
    let authorization = BASE64.encode(signature.into_bytes());

    // Build headers structure
    // Format: [timestamp, md5, authorization, customer-id]
    let ws_headers = WsHeaders {
        headers: vec![
            full_date.clone(),
            payload_md5.clone(),
            authorization.clone(),
            config::Config::user().to_string(),
        ],
        extra_keys: vec![project.clone()],
    };

    // Convert headers to JSON (compact format, no escaping needed)
    let headers_json = serde_json::to_string(&ws_headers)?;

    // Build complete message: headers_json + dataframe_json
    let full_message = format!("{}{}", headers_json, dataframe_json);

    // Add length prefix (LENGTH OF HEADER JSON, not total message!)
    let header_length = headers_json.len();
    let complete_message = format!("{},{}", header_length, full_message);

    // Debug output
    //println!("Headers JSON: {}", headers_json);
    //println!("DataFrame JSON: {}", dataframe_json);
    //println!("Header length: {}", header_length);
    //println!("Full message length: {}", full_message.len());
    //println!("Complete message: {}", complete_message);
    //println!();
    //println!("Connecting to WebSocket: {}", ws_url);

    // Connect to WebSocket
    let (ws_stream, _) = connect_async(&ws_url).await?;
    //println!("WebSocket connected successfully");

    let (mut write, mut read) = ws_stream.split();

    // Send the complete message
    //println!("Sending message (header length prefix: {})", header_length);
    write.send(Message::Text(complete_message)).await?;
    //println!("Message sent successfully");

    // Read responses
    //println!("Waiting for responses...");
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);

                // Check if this is a completion/done message
                if text.contains("\"statusCode\"") || text.contains("DONE") || text.contains("FINISHED") {
                    //println!("Learn operation completed");
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                println!("Received binary data: {} bytes", data.len());
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

    //println!("WebSocket communication completed");
    Ok(())
}
