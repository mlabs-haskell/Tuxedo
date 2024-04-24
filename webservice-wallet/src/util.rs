use crate::kitty::Gender;
use crate::service_handlers::key_handler::key_service_handler::GenerateKeyRequest;
use crate::service_handlers::kitty_handler::kitty_service_handler::{
    CreateKittyRequest, CreateKittyResponse,
};
use crate::service_handlers::money_handler::money_servicehandler::MintCoinsRequest;
use crate::{create_kitty, debug_generate_key, get_local_keystore, mint_coins};
use axum::Json;
use sled::Db;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn use_test_db() -> Arc<Mutex<Db>> {
    let db = Arc::new(Mutex::new(
        sled::Config::new().temporary(true).open().unwrap(),
    ));
    db
}

pub async fn insert_test_coins(owner_public_key: &str, amount: u128) {
    let _ = get_local_keystore().await.expect("Error");
    let request = MintCoinsRequest {
        amount,
        owner_public_key: owner_public_key.to_string(),
    };
    let json_request = Json(request);
    mint_coins(json_request).await;
}

pub async fn mint_kitty(
    owner_public_key: &str,
    name: &str,
    gender: Option<Gender>,
) -> Result<Json<CreateKittyResponse>, Infallible> {
    let request = CreateKittyRequest {
        name: name.to_string(),
        owner_public_key: owner_public_key.to_string(),
        gender,
    };
    let json_request = Json(request);
    create_kitty(json_request).await
}

pub async fn create_seller() -> String {
    let request_body = GenerateKeyRequest {
        password: Some("".to_string()),
    };

    let response = debug_generate_key(Json(request_body)).await;
    if let Some(public_key) = &response.public_key {
        insert_test_coins(&public_key, 300).await;
        let _ = get_local_keystore().await;
        return public_key.clone();
    }
    String::new()
}
