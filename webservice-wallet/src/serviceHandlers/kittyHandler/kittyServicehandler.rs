use serde::{Deserialize, Serialize};

use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::{Decode, Encode};
use std::path::PathBuf;
use sled::Db;
use crate::kitty;
use sp_core::H256;

use crate::cli::CreateKittyArgs;
use crate::cli::ListKittyForSaleArgs;

/// The default RPC endpoint for the wallet to connect to
const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use crate::{ keystore::SHAWN_PUB_KEY};

use crate::get_db;
use crate::get_local_keystore;
use crate::sync_and_get_db;


use axum::{http::StatusCode, response::IntoResponse, routing::{get, post, put},Json, Router};
use std::convert::Infallible;
use axum::{response::Html,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use runtime::{opaque::Block as OpaqueBlock, Block};
use anyhow::bail;


use runtime::{
    kitties::{
        DadKittyStatus, FreeKittyConstraintChecker, KittyDNA, KittyData, KittyHelpers,
        MomKittyStatus, Parent,
    },
    money::{Coin, MoneyConstraintChecker},
    tradable_kitties::{TradableKittyConstraintChecker, TradableKittyData},
    OuterVerifier, Transaction,
};

#[derive(Debug, Deserialize)]
pub struct CreateKittyRequest {
    pub name: String,
    pub owner_public_key:String,
}

#[derive(Debug, Serialize)]
pub struct CreateKittyResponse {
    pub message: String,
    pub kitty:Option<KittyData>
    // Add any additional fields as needed
}

/*
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
        kitty_name: body.name.to_string(),
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
*/

pub async fn create_kitties(body: Json<CreateKittyRequest>) -> Result<Json<CreateKittyResponse>, Infallible> {
    println!("create_kitties called ");
    let client_result = HttpClientBuilder::default().build(DEFAULT_ENDPOINT);
    let db = sync_and_get_db().await.expect("Error");

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(CreateKittyResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                kitty:None,
            }));
        }
    };

    // Convert the hexadecimal string to bytes
    let public_key_bytes = hex::decode(SHAWN_PUB_KEY).expect("Invalid hexadecimal string");
    let public_key_h256 = H256::from_slice(&public_key_bytes);

    match kitty::create_kitty(&db, &client, CreateKittyArgs {
        kitty_name: body.name.to_string(),
        owner: public_key_h256,
    }).await {
        Ok(Some(created_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = CreateKittyResponse {
                message: format!("Kitty created successfully"),
                kitty: Some(created_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        },
        Ok(None) => Ok(Json(CreateKittyResponse {
            message: format!("Kitty creation failed: No data returned"),
            kitty:None,
        })),
        Err(err) => Ok(Json(CreateKittyResponse {
            message: format!("Error creating kitty: {:?}", err),
            kitty:None,
        })),
    }
}

////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct ListKittyForSaleRequest {
    pub name: String,
    pub price: u128,
    pub owner_public_key:String,
}

#[derive(Debug, Serialize)]
pub struct ListKittyForSaleResponse {
    pub message: String,
    pub td_kitty:Option<TradableKittyData>
    // Add any additional fields as needed
}
pub async fn list_kitties_for_sale (body: Json<ListKittyForSaleRequest>) -> Result<Json<ListKittyForSaleResponse>, Infallible> {
    let client_result = HttpClientBuilder::default().build(DEFAULT_ENDPOINT);
    let db = sync_and_get_db().await.expect("Error");

    let client = match client_result {
        Ok(client) => client,
        Err(err) => {
            return Ok(Json(ListKittyForSaleResponse {
                message: format!("Error creating HTTP client: {:?}", err),
                td_kitty:None,
            }));
        }
    };

    // Convert the hexadecimal string to bytes
    let public_key_bytes = hex::decode(SHAWN_PUB_KEY).expect("Invalid hexadecimal string");
    let public_key_h256 = H256::from_slice(&public_key_bytes);
    let ks = get_local_keystore().await.expect("Error");

    match kitty::list_kitty_for_sale(&db, &client, &ks,ListKittyForSaleArgs {
        name: body.name.to_string(),
        price: body.price,
        owner: public_key_h256,
    }).await {
        Ok(Some(listed_kitty)) => {
            // Convert created_kitty to JSON and include it in the response
            let response = ListKittyForSaleResponse {
                message: format!("Kitty listed for sale successfully"),
                td_kitty: Some(listed_kitty), // Include the created kitty in the response
            };
            Ok(Json(response))
        },
        Ok(None) => Ok(Json(ListKittyForSaleResponse {
            message: format!("Kitty listing forsale  failed: No data returned"),
            td_kitty:None,
        })),
        Err(err) => Ok(Json(ListKittyForSaleResponse {
            message: format!("Error listing forsale: {:?}", err),
            td_kitty:None,
        })),
    }
}