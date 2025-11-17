use crate::config;
use crate::auth::AuthHeaders;
use crate::common::{DataFrame, MLContext, parse_int_list_as_strings, parse_string_list};
use std::collections::HashMap;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Debug)]
struct AutoSelectRequest {
    dataframe: DataFrame,
    mlcontext: MLContext,
}

pub async fn handle_autoselect(
    project: String,
    model_name: String,
    task: String,
    val_file: Option<String>,
    val_df: Option<String>,
    targets: Option<String>,
    inputs: Option<String>,
    target_names: Option<String>,
    input_names: Option<String>,
    params: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Auto-selecting algorithm for model: {} in project: {} (task: {})", model_name, project, task);

    // Check that either val_df or val_file is provided
    if val_df.is_none() && val_file.is_none() {
        return Err("Must specify either --val-df (JSON dataframe) or --val-file (data file)".into());
    }

    if val_df.is_some() && val_file.is_some() {
        return Err("Cannot specify both --val-df and --val-file. Choose one.".into());
    }

    // Parse the validation data
    let dataframe = if let Some(json_df) = val_df {
        println!("Using JSON validation dataframe");
        serde_json::from_str::<DataFrame>(&json_df)?
    } else if let Some(file_path) = val_file {
        println!("Reading validation data from file: {}", file_path);

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
                .map(|s| {
                    let trimmed = s.trim();
                    // Try to clean up decimal values that should be integers
                    if let Ok(val) = trimmed.parse::<f64>() {
                        if val.fract() == 0.0 {
                            // It's a whole number, return as integer string
                            return format!("{}", val as i64);
                        }
                    }
                    trimmed.to_string()
                })
                .collect();
            data.push(row);
        }

        println!("Loaded {} rows with {} columns", data.len(), attribute_names.len());

        // For validation data, we need to infer types or use "C" for continuous
        let attribute_types: Vec<String> = vec!["C".to_string(); attribute_names.len()];

        DataFrame {
            attribute_names,
            data,
            attribute_types,
            error_handling: Some(1),
        }
    } else {
        unreachable!()
    };

    // Parse targets (IDs) - convert to strings for server
    let targets_vec = if let Some(t) = targets {
        println!("Targets (IDs): {}", t);
        Some(parse_int_list_as_strings(&t)?)
    } else {
        None
    };

    // Parse inputs (IDs) - convert to strings for server
    let inputs_vec = if let Some(i) = inputs {
        println!("Inputs (IDs): {}", i);
        Some(parse_int_list_as_strings(&i)?)
    } else {
        None
    };

    // Parse target names
    let target_names_vec = if let Some(tn) = target_names {
        println!("Target Names: {}", tn);
        Some(parse_string_list(&tn))
    } else {
        None
    };

    // Parse input names
    let input_names_vec = if let Some(in_) = input_names {
        println!("Input Names: {}", in_);
        Some(parse_string_list(&in_))
    } else {
        None
    };

    // Parse extra parameters
    let mut extra_params: HashMap<String, String> = HashMap::new();
    if let Some(p) = params {
        println!("Parameters: {}", p);
        for pair in p.split(',') {
            let parts: Vec<&str> = pair.trim().split('=').collect();
            if parts.len() == 2 {
                extra_params.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    // Build the MLContext
    let ml_context = MLContext {
        targets: targets_vec,
        input_attributes: inputs_vec,
        input_attribute_names: input_names_vec,
        target_attribute_names: target_names_vec,
        extra_parameters: extra_params,
    };

    // Build the request body
    let request_body = AutoSelectRequest {
        dataframe,
        mlcontext: ml_context,
    };

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/projects/{}/autoSelect",
        config::Config::user(),
        project
    );

    // Build query string with task and modelid
    let query = format!("task={}&modelid={}", task, model_name);

    // Build the full URL
    let url = format!(
        "http://{}:{}{}?{}",
        config::Config::host(),
        config::Config::port(),
        resource,
        query
    );

    // Convert body to JSON string
    let body_json = serde_json::to_string(&request_body)?;
    println!("Request body length: {} bytes", body_json.len());
    println!("DEBUG - Full request body: {}", body_json);

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

    // Extract the sym-job-id header if present
    let job_id = response.headers()
        .get("sym-job-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let response_text = response.text().await?;

    if status.is_success() {
        println!("Auto-select request submitted successfully!");
        if let Some(job_id) = job_id {
            println!("Job ID: {}", job_id);
            println!("You can check the job status with:");
            println!("  job --id={}", job_id);
        }
        println!("Response: {}", response_text);
    } else {
        println!("Failed to auto-select model. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
