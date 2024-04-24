use serde::{Deserialize, Serialize};

use crate::keystore;

use axum::Json;

#[derive(Debug, Deserialize)]
pub struct GenerateKeyRequest {
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GenerateKeyResponse {
    pub message: String,
    pub public_key: Option<String>,
    pub phrase: Option<String>,
}

pub async fn debug_generate_key(body: Json<GenerateKeyRequest>) -> Json<GenerateKeyResponse> {
    match keystore::generate_key(body.password.clone()).await {
        Ok((public_key, phrase)) => Json(GenerateKeyResponse {
            message: format!("Keys generated successfully"),
            public_key: Some(public_key),
            phrase: Some(phrase),
        }),
        Err(err) => Json(GenerateKeyResponse {
            message: format!("Error generating keys: {:?}", err),
            public_key: None,
            phrase: None,
        }),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// get keys
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct GetKeyResponse {
    pub message: String,
    pub keys: Option<Vec<String>>,
}

pub async fn debug_get_keys() -> Json<GetKeyResponse> {
    match keystore::get_keys().await {
        Ok(keys_iter) => {
            // Lets collect keys into a vector of strings
            let keys: Vec<String> = keys_iter.map(|key| hex::encode(key)).collect();

            Json(GetKeyResponse {
                message: format!("Keys retrieved successfully"),
                keys: Some(keys),
            })
        }
        Err(err) => Json(GetKeyResponse {
            message: format!("Error retrieving keys: {:?}", err),
            keys: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_debug_generate_key_success() {
        let request_body = GenerateKeyRequest {
            password: Some("securepassword123".to_string()),
        };

        let response = debug_generate_key(Json(request_body)).await;
        assert_eq!(response.0.message, "Keys generated successfully");
        assert!(response.0.public_key.is_some());
        assert!(response.0.phrase.is_some());
    }

    #[tokio::test]
    async fn test_debug_get_keys_success() {
        let response = debug_get_keys().await;
        assert_eq!(response.0.message, "Keys retrieved successfully");
        assert!(response.0.keys.is_some());
    }
}
