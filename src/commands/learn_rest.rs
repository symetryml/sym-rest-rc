use crate::config;
use crate::auth::AuthHeaders;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub async fn handle_learn(
    project: String,
    file: String,
    types: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Learning from file: {} for project: {}", file, project);

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

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/projects/{}/learn",
        config::Config::user(),
        project
    );

    // Build the full URL
    let url = format!(
        "http://{}:{}{}",
        config::Config::host(),
        config::Config::port(),
        resource
    );

    // Convert body to JSON string
    let body_json = serde_json::to_string(&dataframe)?;

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

    // Try to extract job ID from headers
    let job_id = response.headers()
        .get("sym-job-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let response_text = response.text().await?;

    if status.is_success() {
        println!("Learn operation started successfully!");
        if let Some(jid) = job_id {
            println!("Job ID: {}", jid);
        }
        println!("Response: {}", response_text);
    } else {
        println!("Failed to start learn operation. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
