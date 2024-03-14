use serde::{Deserialize, Serialize};

use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::{Decode, Encode};
use runtime::OuterVerifier;
use std::path::PathBuf;
use sled::Db;
use crate::kitty;
use sp_core::H256;

use crate::cli::CreateKittyArgs;

/// The default RPC endpoint for the wallet to connect to
const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use crate::{ keystore::SHAWN_PUB_KEY};

use crate::get_db;


use axum::{http::StatusCode, response::IntoResponse, routing::{get, post},Json, Router};
use axum::{response::Html,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use runtime::{opaque::Block as OpaqueBlock, Block};
use anyhow::bail;

#[derive(Debug, Deserialize)]
pub struct CreateKittyRequest {
    pub name: String,
    pub owner_public_key:String,
}

#[derive(Debug, Serialize)]
pub struct CreateKittyResponse {
    pub message: String,
    // Add any additional fields as needed
}

pub async fn create_kitties(body: Json<CreateKittyRequest>) -> Json<CreateKittyResponse> {
    println!("create_kitties called ");
    let client_result = HttpClientBuilder::default().build(DEFAULT_ENDPOINT);
    let db = get_db().await.expect("Errior");
    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Json(CreateKittyResponse {
                message: format!("Error creating HTTP client: {:?}", err),
            });
        }
    };
    // Convert the hexadecimal string to bytes
    let public_key_bytes = hex::decode(SHAWN_PUB_KEY).expect("Invalid hexadecimal string");
    let public_key_h256 = H256::from_slice(&public_key_bytes);

    match kitty::create_kitty(&db,&client, CreateKittyArgs {
        kitty_name: "Amit".to_string(),
        owner: public_key_h256,
    }).await {
        Ok(()) => Json(CreateKittyResponse {
            message: format!("Kitty created successfully"),
        }),
        Err(err) => Json(CreateKittyResponse {
            message: format!("Error creating kitty: {:?}", err),
        }),
    }
}