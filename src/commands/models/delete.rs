use crate::config;
use crate::auth::AuthHeaders;

pub async fn handle_delete(
    project: String,
    model: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Deleting model '{}' from project '{}'", model, project);

    // Build the resource path - DELETE /symetry/rest/{cid}/projects/{pid}/models/{mid}
    let resource = format!(
        "/symetry/rest/{}/projects/{}/models/{}",
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

    // Generate authentication headers (no body for DELETE)
    let auth = AuthHeaders::generate(
        "DELETE",
        &resource,
        None,
        None,
        config::Config::user(),
        &config::Config::secretkey(),
    )?;

    // Create HTTP client and make the request
    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
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
        println!("Model '{}' deleted successfully from project '{}'!", model, project);
        if !response_text.is_empty() {
            println!("Response: {}", response_text);
        }
    } else {
        println!("Failed to delete model. Status: {}", status);
        println!("Response: {}", response_text);
        return Err(format!("Request failed with status: {}", status).into());
    }

    Ok(())
}
