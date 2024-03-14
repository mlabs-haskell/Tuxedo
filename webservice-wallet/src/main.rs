//! A simple CLI wallet. For now it is a toy just to start testing things out.

use clap::Parser;
use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::{Decode, Encode};
use runtime::OuterVerifier;
use std::path::PathBuf;
use sled::Db;
use crate::kitty::create_kitty;
use tuxedo_core::{types::OutputRef, verifier::*};
use sp_core::H256;

//mod amoeba;
mod TradableKitties;
mod cli;
mod req_resp;
mod keystore;
mod kitty;
mod money;
mod output_filter;
mod rpc;
mod sync;
mod timestamp;

use cli::{Cli, Command};
use crate::cli::MintCoinArgs;
use crate::cli::CreateKittyArgs;

//use moneyServicehandler::{MintCoinsRequest, MintCoinsResponse};
mod serviceHandlers {
    
    pub mod blockHandler {
        pub mod blockServicehandler;
    }

    pub mod moneyHandler {
        pub mod moneyServicehandler;
    }

    pub mod kittyHandler {
        pub mod kittyServicehandler;
    }
}

use serviceHandlers::moneyHandler::moneyServicehandler::{MintCoinsRequest, MintCoinsResponse, mint_coins};
use serviceHandlers::kittyHandler::kittyServicehandler::{CreateKittyRequest, CreateKittyResponse, create_kitties};
use serviceHandlers::blockHandler::blockServicehandler::{BlockRequest, BlockResponse, get_block};

/// The default RPC endpoint for the wallet to connect to
const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use crate::{ keystore::SHAWN_PUB_KEY};


use axum::{http::StatusCode, response::IntoResponse, routing::{get, post},Json, Router};
use axum::{response::Html,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use runtime::{opaque::Block as OpaqueBlock, Block};
use anyhow::bail;
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
       // .route("/get-block",get(|| async { get_block().await }))
       .route("/mint-coins", get(get_block)) 
       .route("/mint-coins", post(mint_coins))
        .route("/create-kitty", post(create_kitties))
     //   .route("/spend-coins", put(spend_coins))
     //  
     //   .route("/breed-kitty", post(breed_kitty))
    //    .route("/buy-kitty", put(buy_kitty))
    //    .route("/update-kitty-name", patch(update_kitty_name))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*
async fn get_block() -> Html<String> {

    println!("In the root function");
    
    match get_blocks().await {
        Ok(Some(node_genesis_block)) => {
            Html(format!("Node's Genesis block::{:?}", node_genesis_block))
        },
        Ok(None) => {
            Html("Node's Genesis block not found".to_string())
        },
        Err(e) => Html(format!("Error: {:?}", e)),
    }
}

async fn get_blocks() -> anyhow::Result<Option<Block>> {
    let client = HttpClientBuilder::default().build(DEFAULT_ENDPOINT)?;
    let mut block_num:i128 = 0;
    let node_genesis_hash = rpc::node_get_block_hash(block_num.try_into().unwrap(), &client)
        .await?
        .expect("node should be able to return some genesis hash");

    let maybe_block = rpc::node_get_block(node_genesis_hash, &client).await?;
    block_num = block_num+1;
    match maybe_block {
        Some(block) => Ok(Some(block)),
        None => bail!("Block not found for hash: {:?}", node_genesis_hash),
    }
}
*/
async fn get_db(
) -> anyhow::Result<Db> {
    let client = HttpClientBuilder::default().build(DEFAULT_ENDPOINT)?;
    let data_path = temp_dir();
    let db_path = data_path.join("wallet_database");
    // Read node's genesis block.
    let node_genesis_hash = rpc::node_get_block_hash(0, &client)
        .await?
        .expect("node should be able to return some genesis hash");
        let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
        .await?
        .expect("node should be able to return some genesis block");
        println!("Node's Genesis block::{:?}", node_genesis_hash);
    
    // Open the local database
    let db = sync::open_db(db_path, node_genesis_hash, node_genesis_block.clone())?;
    Ok(db)
}

/// Parse a string into an H256 that represents a public key
pub(crate) fn h256_from_string(s: &str) -> anyhow::Result<H256> {
    let s = strip_0x_prefix(s);

    let mut bytes: [u8; 32] = [0; 32];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H256::from(bytes))
}

/// Parse an output ref from a string
fn output_ref_from_string(s: &str) -> Result<OutputRef, clap::Error> {
    let s = strip_0x_prefix(s);
    let bytes =
        hex::decode(s).map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;

    OutputRef::decode(&mut &bytes[..])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))
}

/// Takes a string and checks for a 0x prefix. Returns a string without a 0x prefix.
fn strip_0x_prefix(s: &str) -> &str {
    if &s[..2] == "0x" {
        &s[2..]
    } else {
        s
    }
}

/// Generate a plaform-specific temporary directory for the wallet
fn temp_dir() -> PathBuf {
    // Since it is only used for testing purpose, we don't need a secure temp dir, just a unique one.
    std::env::temp_dir().join(format!(
        "tuxedo-wallet-{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
    ))
}

/// Generate the platform-specific default data path for the wallet
fn default_data_path() -> PathBuf {
    // This uses the directories crate.
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html

    // Application developers may want to put actual qualifiers or organization here
    let qualifier = "";
    let organization = "";
    let application = env!("CARGO_PKG_NAME");

    directories::ProjectDirs::from(qualifier, organization, application)
        .expect("app directories exist on all supported platforms; qed")
        .data_dir()
        .into()
}

/// Utility to pretty print an outer verifier
pub fn pretty_print_verifier(v: &OuterVerifier) {
    match v {
        OuterVerifier::Sr25519Signature(sr25519_signature) => {
            println! {"owned by {}", sr25519_signature.owner_pubkey}
        }
        OuterVerifier::UpForGrabs(_) => println!("that can be spent by anyone"),
        OuterVerifier::ThresholdMultiSignature(multi_sig) => {
            let string_sigs: Vec<_> = multi_sig
                .signatories
                .iter()
                .map(|sig| format!("0x{}", hex::encode(sig)))
                .collect();
            println!(
                "Owned by {:?}, with a threshold of {} sigs necessary",
                string_sigs, multi_sig.threshold
            );
        }
    }
}
