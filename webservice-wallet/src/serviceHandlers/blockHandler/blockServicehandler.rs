use serde::{Deserialize, Serialize};

use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::{Decode, Encode};
use runtime::OuterVerifier;
use std::path::PathBuf;
use sled::Db;
use crate::money;
use sp_core::H256;
use crate::rpc;

use crate::cli::MintCoinArgs;
use crate::cli::CreateKittyArgs;

/// The default RPC endpoint for the wallet to connect to
const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use crate::{ keystore::SHAWN_PUB_KEY};


use axum::{http::StatusCode, response::IntoResponse, routing::{get, post},Json, Router};
use axum::{response::Html,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use runtime::{opaque::Block as OpaqueBlock, Block};
use anyhow::bail;


#[derive(Debug, Deserialize)]
pub struct BlockRequest {
    pub number: u128,
}

#[derive(Debug, Serialize)]
pub struct BlockResponse {
    pub message: String,
}

pub async fn get_block(body: Json<BlockRequest>) -> Json<BlockResponse> {
    println!("Get block called fro block num {} ",body.number);
    match get_blocks(body.number).await {
        Ok(Some(node_genesis_block)) => Json(BlockResponse {
            message: format!("block  found {:?}",node_genesis_block),
        }),
        
        Ok(None) => Json(BlockResponse {
            message: format!("Node's block not found"),
        }),
        Err(err) => Json(BlockResponse {
            message: format!("Error getting the block: {:?}", err),
        }),
        Err(_) => Json(BlockResponse {
            message: format!("Unknown Error getting the block: "),
        }),
    }
}

async fn get_blocks(number: u128) -> anyhow::Result<Option<Block>> {
    let client = HttpClientBuilder::default().build(DEFAULT_ENDPOINT)?;
    let node_block_hash = rpc::node_get_block_hash(number.try_into().unwrap(), &client)
        .await?
        .expect("node should be able to return some genesis hash");
    println!("Get blocks node_block_hash {:?} ",node_block_hash);
    let maybe_block = rpc::node_get_block(node_block_hash, &client).await?;
    println!("BlockData {:?} ",maybe_block.clone().unwrap());
    match maybe_block {
        Some(block) => Ok(Some(block)),
        None => bail!("Block not found for hash: {:?}", node_block_hash),
    }
}