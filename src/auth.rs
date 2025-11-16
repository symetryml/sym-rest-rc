use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::Utc;

type HmacSha256 = Hmac<Sha256>;

/// Generate authentication headers for Symetry REST API
pub struct AuthHeaders {
    pub content_md5: String,
    pub sym_date: String,
    pub authorization: String,
}

impl AuthHeaders {
    /// Generate authentication headers
    ///
    /// # Arguments
    /// * `method` - HTTP method (GET, POST, etc.)
    /// * `resource` - API resource path (e.g., "/symetry/rest/c1/projects")
    /// * `query` - Query string (e.g., "pid=test&type=cpu")
    /// * `body` - Request body as JSON string
    /// * `customer_id` - Customer ID
    /// * `secret_key` - Base64-encoded secret key
    pub fn generate(
        method: &str,
        resource: &str,
        query: Option<&str>,
        body: Option<&str>,
        customer_id: &str,
        secret_key: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate timestamp in format: yyyy-MM-dd HH:mm:ss;nanoseconds
        let now = Utc::now();
        let date_part = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let nanos = now.timestamp_subsec_nanos();
        let sym_date = format!("{};{}", date_part, nanos);

        // Compute MD5 of body if present
        let (content_md5, body_to_sign) = if let Some(b) = body {
            let digest = md5::compute(b.as_bytes());
            let md5_b64 = BASE64.encode(&digest.0);
            (md5_b64, b)
        } else {
            (String::new(), "")
        };

        // Build string to sign based on whether we have body and/or query
        let string_to_sign = match (body, query) {
            (None, None) => {
                // No body, no query
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n",
                    method, content_md5, secret_key, sym_date, customer_id, resource
                )
            }
            (None, Some(q)) => {
                // No body, with query
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    method, content_md5, secret_key, sym_date, customer_id, resource, q
                )
            }
            (Some(_), None) => {
                // With body, no query
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    method, content_md5, secret_key, sym_date, customer_id, body_to_sign, resource
                )
            }
            (Some(_), Some(q)) => {
                // With body, with query
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    method, content_md5, secret_key, sym_date, customer_id, body_to_sign, resource, q
                )
            }
        };

        // Decode the base64 secret key
        let secret_bytes = BASE64.decode(secret_key)?;

        // Compute HMAC-SHA256
        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .map_err(|e| format!("Invalid secret key length: {}", e))?;
        mac.update(string_to_sign.as_bytes());
        let signature = mac.finalize();
        let authorization = BASE64.encode(signature.into_bytes());

        Ok(AuthHeaders {
            content_md5,
            sym_date,
            authorization,
        })
    }
}
