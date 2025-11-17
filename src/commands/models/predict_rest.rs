use crate::config;
use crate::auth::AuthHeaders;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PredictRequest {
    attribute_names: Vec<String>,
    data: Vec<Vec<String>>,
    attribute_types: Vec<String>,
}

pub async fn handle_predict(
    project: String,
    model: String,
    df: Option<String>,
    file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Making prediction with project: {} and model: {}", project, model);

    // Check that either df or file is provided
    if df.is_none() && file.is_none() {
        return Err("Must specify either --df (JSON dataframe) or --file (data file)".into());
    }

    if df.is_some() && file.is_some() {
        return Err("Cannot specify both --df and --file. Choose one.".into());
    }

    // Parse the input data
    let predict_request = if let Some(json_df) = df {
        println!("Using JSON dataframe: {}", json_df);
        serde_json::from_str::<PredictRequest>(&json_df)?
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

        PredictRequest {
            attribute_names,
            data,
            attribute_types,
        }
    } else {
        unreachable!()
    };

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/projects/{}/predict/{}",
        config::Config::user(),
        project,
        model
    );

    // Build the full URL
    let url = format!(
        "http://{}:{}{}",
        config::Config::host(),
        config::Config::port(),
        resource
    );

    // Convert body to JSON string
    let body_json = serde_json::to_string(&predict_request)?;
    println!("Request body: {}", body_json);

    // Generate authentication headers
    let auth = AuthHeaders::generate(
        "POST",
        &resource,
        None,
        Some(&body_json),
        config::Config::user(),
        &config::Config::secretkey(),
    )?;

    // Create HTTP client and make the request
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-MD5", &auth.content_md5)
        .header("Sym-date", &auth.sym_date)
        .header("Customer-ID", config::Config::user())
        .header("Authorization", &auth.authorization)
        .header("sym-version", "6.3")
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .await?;

    // Check response status
    let status = response.status();
    let response_text = response.text().await?;

    if status.is_success() {
        println!("Prediction completed successfully!");
        println!("Response: {}", response_text);
    } else {
        println!("Failed to make prediction. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
