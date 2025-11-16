use crate::config;
use crate::auth::AuthHeaders;

pub async fn handle_job(
    job_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking status for job: {}", job_id);

    // Build the resource path
    let resource = format!(
        "/symetry/rest/{}/jobs/{}",
        config::Config::user(),
        job_id
    );

    // Build the full URL
    let url = format!(
        "http://{}:{}{}",
        config::Config::host(),
        config::Config::port(),
        resource
    );

    // Generate authentication headers
    let auth = AuthHeaders::generate(
        "GET",
        &resource,
        None,
        None,
        config::Config::user(),
        &config::Config::secretkey(),
    )?;

    // Create HTTP client and make the request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Content-MD5", &auth.content_md5)
        .header("Sym-date", &auth.sym_date)
        .header("Customer-ID", config::Config::user())
        .header("Authorization", &auth.authorization)
        .header("sym-version", "6.3")
        .send()
        .await?;

    // Check response status
    let status = response.status();
    let response_text = response.text().await?;

    if status.is_success() {
        println!("Job status retrieved successfully!");
        println!("Response: {}", response_text);
    } else {
        println!("Failed to get job status. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
