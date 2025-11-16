use crate::config;
use crate::auth::AuthHeaders;
use std::collections::HashMap;

pub async fn handle_create(
    name: String,
    project_type: String,
    params: Option<String>,
    enable_histogram: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating project: {} (type: {})", name, project_type);

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/projects",
        config::Config::user()
    );

    // Build query string
    let query = format!(
        "pid={}&persist=true&type={}&enableHistogram={}",
        name, project_type, enable_histogram
    );

    // Build the full URL
    let url = format!(
        "http://{}:{}{}?{}",
        config::Config::host(),
        config::Config::port(),
        resource,
        query
    );

    // Build request body - start with required parameter
    let mut body_params: HashMap<String, String> = HashMap::new();
    body_params.insert("sml_project_autosave".to_string(), "true".to_string());

    // Parse and add optional parameters
    if let Some(p) = params {
        println!("Parameters: {}", p);
        for pair in p.split(',') {
            let parts: Vec<&str> = pair.trim().split('=').collect();
            if parts.len() == 2 {
                body_params.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    // Convert body to JSON string
    let body_json = serde_json::to_string(&body_params)?;

    // Generate authentication headers
    let auth = AuthHeaders::generate(
        "POST",
        &resource,
        Some(&query),
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
        println!("Project created successfully!");
        println!("Response: {}", response_text);
    } else {
        println!("Failed to create project. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
