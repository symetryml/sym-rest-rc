use crate::config;
use crate::auth::AuthHeaders;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MLContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_attributes: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_attribute_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_attribute_names: Option<Vec<String>>,
    extra_parameters: HashMap<String, String>,
}

pub async fn handle_build(
    project: String,
    model_name: String,
    model_type: String,
    targets: Option<String>,
    inputs: Option<String>,
    target_names: Option<String>,
    input_names: Option<String>,
    params: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building model: {} (type: {}) for project: {}", model_name, model_type, project);

    // Check which form is being used
    let using_ids = targets.is_some() || inputs.is_some();
    let using_names = target_names.is_some() || input_names.is_some();

    if using_ids && using_names {
        return Err("Cannot mix ID-based and name-based parameters".into());
    }

    if !using_ids && !using_names {
        return Err("Must specify either targets/inputs (IDs) or targetNames/inputNames (names)".into());
    }

    // Parse targets (IDs)
    let targets_vec = if let Some(t) = targets {
        println!("Targets (IDs): {}", t);
        Some(parse_int_list(&t)?)
    } else {
        None
    };

    // Parse inputs (IDs)
    let inputs_vec = if let Some(i) = inputs {
        println!("Inputs (IDs): {}", i);
        Some(parse_int_list(&i)?)
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

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/projects/{}/build",
        config::Config::user(),
        project
    );

    // Build query string
    let query = format!("algo={}&modelid={}", model_type, model_name);

    // Build the full URL
    let url = format!(
        "http://{}:{}{}?{}",
        config::Config::host(),
        config::Config::port(),
        resource,
        query
    );

    // Convert body to JSON string
    let body_json = serde_json::to_string(&ml_context)?;
    println!("Request body: {}", body_json);

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
        println!("Model build request submitted successfully!");
        if let Some(job_id) = job_id {
            println!("Job ID: {}", job_id);
            println!("You can check the job status with:");
            println!("  job --id={}", job_id);
        }
        println!("Response: {}", response_text);
    } else {
        println!("Failed to build model. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}

// Helper function to parse comma-separated integers
fn parse_int_list(s: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    s.split(',')
        .map(|item| {
            item.trim()
                .parse::<i32>()
                .map_err(|e| format!("Failed to parse '{}' as integer: {}", item.trim(), e).into())
        })
        .collect()
}

// Helper function to parse comma-separated strings
fn parse_string_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_string())
        .collect()
}
