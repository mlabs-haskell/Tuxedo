//! A simple CLI wallet. For now it is a toy just to start testing things out.

use jsonrpsee::http_client::HttpClient;
use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::Decode;
use runtime::OuterVerifier;
use sled::Db;
use std::path::PathBuf;
//use crate::kitty::{create_kitty,list_kitty_for_sale};
use sc_keystore::LocalKeystore;
use sp_core::H256;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use tuxedo_core::types::OutputRef;
//mod amoeba;

mod keystore;
mod kitty;
mod money;
mod output_filter;
mod rpc;
mod sync;
mod timestamp;
mod util;

//use moneyServicehandler::{MintCoinsRequest, MintCoinsResponse};
mod service_handlers {

    pub mod block_handler {
        pub mod block_service_handler;
    }

    pub mod money_handler {
        pub mod money_servicehandler;
    }

    pub mod kitty_handler {
        pub mod kitty_service_handler;
    }

    pub mod key_handler {
        pub mod key_service_handler;
    }
}

use service_handlers::key_handler::key_service_handler::{debug_generate_key, debug_get_keys};

use service_handlers::money_handler::money_servicehandler::{
    get_all_coins, get_owned_coins, mint_coins,
};

use service_handlers::kitty_handler::kitty_service_handler::{
    breed_kitty, buy_kitty, create_kitty, delist_kitty_from_sale, get_all_kitty_list,
    get_all_td_kitty_list, get_kitty_by_dna, get_owned_kitty_list, get_owned_td_kitty_list,
    get_td_kitty_by_dna, get_txn_and_inpututxolist_for_breed_kitty,
    get_txn_and_inpututxolist_for_buy_kitty, get_txn_and_inpututxolist_for_delist_kitty_from_sale,
    get_txn_and_inpututxolist_for_kitty_name_update,
    get_txn_and_inpututxolist_for_list_kitty_for_sale,
    get_txn_and_inpututxolist_for_td_kitty_name_update,
    get_txn_and_inpututxolist_for_td_kitty_price_update, list_kitty_for_sale, update_kitty_name,
    update_td_kitty_name, update_td_kitty_price,
};

use service_handlers::block_handler::block_service_handler::get_block;

/// The default RPC endpoint for the wallet to connect to
//const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use axum::{
    routing::{get, patch, post, put},
    Extension, Router,
};
use std::net::SocketAddr;
use std::{fs, io};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, serde::Deserialize)]
struct Config {
    server: ServerConfig,
}

#[derive(Debug, serde::Deserialize)]
struct ServerConfig {
    block_chain_node_endpoint: String,
    socket_address: String,
}

fn read_config() -> Result<Config, io::Error> {
    let config_contents = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() {
    let config = read_config().expect("Cant read the server config");
    let block_chain_node_endpoint = config.server.block_chain_node_endpoint;
    // let socket_address = config.server.socket_address;
    let socket_address: SocketAddr = config
        .server
        .socket_address
        .parse()
        .expect("Invalid socket address");

    println!("Blockchain node Endpoint: {}", block_chain_node_endpoint);
    println!("Socket Address: {}", socket_address);
    let cors = CorsLayer::new().allow_origin(Any);
    let db = Arc::new(Mutex::new(get_db().await.expect("Failed to init db")));

    let app = Router::new()
        .route("/get-block", get(get_block))
        .route("/post-mint-coin", post(mint_coins))
        .route("/post-create-kitty", post(create_kitty))
        .route(
            "/get-txn-and-inpututxolist-for-listkitty-forsale",
            get(get_txn_and_inpututxolist_for_list_kitty_for_sale),
        )
        .route("/put-listkitty-for-sale", put(list_kitty_for_sale))
        .route(
            "/get-txn-and-inpututxolist-for-delist-kitty-from-sale",
            get(get_txn_and_inpututxolist_for_delist_kitty_from_sale),
        )
        .route("/put-delist-kitty-from-sale", put(delist_kitty_from_sale))
        .route(
            "/get-txn-and-inpututxolist-for-kitty-name-update",
            get(get_txn_and_inpututxolist_for_kitty_name_update),
        )
        .route("/patch-update-kitty-name", patch(update_kitty_name))
        .route(
            "/get-txn-and-inpututxolist-for-td-kitty-name-update",
            get(get_txn_and_inpututxolist_for_td_kitty_name_update),
        )
        .route("/patch-update-td-kitty-name", patch(update_td_kitty_name))
        .route(
            "/get-txn-and-inpututxolist-for-td-kitty-price-update",
            get(get_txn_and_inpututxolist_for_td_kitty_price_update),
        )
        .route("/patch-update-td-kitty-price", patch(update_td_kitty_price))
        .route(
            "/get-txn-and-inpututxolist-for-breed-kitty",
            get(get_txn_and_inpututxolist_for_breed_kitty),
        )
        .route("/post-breed-kitty", post(breed_kitty))
        .route(
            "/get-txn-and-inpututxolist-for-buy-kitty",
            get(get_txn_and_inpututxolist_for_buy_kitty),
        )
        .route("/patch-buy-kitty", patch(buy_kitty))
        .route("/get-kitty-by-dna", get(get_kitty_by_dna))
        .route("/get-tradable-kitty-by-dna", get(get_td_kitty_by_dna))
        .route("/get-all-kitty-list", get(get_all_kitty_list))
        .route("/get-all-tradable-kitty-list", get(get_all_td_kitty_list))
        .route("/get-owned-kitty-list", get(get_owned_kitty_list))
        .route(
            "/get-owned-tradable-kitty-list",
            get(get_owned_td_kitty_list),
        )
        .route("/get-all-coins", get(get_all_coins))
        .route("/get-owned-coins", get(get_owned_coins))
        // Below are for debug purpose only.
        .route("/debug-generate-key", post(debug_generate_key))
        .route("/debug-get-keys", get(debug_get_keys))
        .layer(cors)
        .layer(Extension(db.clone()));

    let periodic_sync_interval = Duration::from_secs(10);
    tokio::spawn(async move {
        let mut interval_timer = interval(periodic_sync_interval);
        loop {
            interval_timer.tick().await;
            match sync_and_get_db(db.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error syncing db: {:?}", e);
                }
            }
        }
    });

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Server error: {}", e)))
        .expect("Cant create the Server due to invalid address")
}

fn get_blockchain_node_endpoint() -> anyhow::Result<String> {
    let config = read_config().expect("Cant read the server config");
    Ok(config.server.block_chain_node_endpoint)
}

async fn get_db() -> anyhow::Result<Db> {
    let client = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"))?;
    let data_path = temp_dir();
    let db_path = data_path.join("wallet_database");
    let node_genesis_hash = rpc::node_get_block_hash(0, &client)
        .await?
        .expect("node should be able to return some genesis hash");
    let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
        .await?
        .expect("node should be able to return some genesis block");
    println!("Node's Genesis block::{:?}", node_genesis_hash);

    let db = sync::open_db(db_path, node_genesis_hash, node_genesis_block.clone())?;
    Ok(db)
}

async fn get_local_keystore() -> anyhow::Result<LocalKeystore> {
    let data_path = temp_dir();
    let keystore_path = data_path.join("keystore");
    println!("keystore_path: {:?}", keystore_path);
    let keystore = sc_keystore::LocalKeystore::open(keystore_path.clone(), None)?;
    keystore::insert_development_key_for_this_session(&keystore)?;
    Ok(keystore)
}

async fn sync_db<F: Fn(&OuterVerifier) -> bool>(
    db: &Db,
    client: &HttpClient,
    filter: &F,
) -> anyhow::Result<()> {
    if !sled::Db::was_recovered(&db) {
        let node_genesis_hash = rpc::node_get_block_hash(0, &client)
            .await?
            .expect("node should be able to return some genesis hash");
        let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
            .await?
            .expect("node should be able to return some genesis block");

        println!(" in sync_db !sled::Db::was_recovered(&db)");
        async {
            let _ = sync::apply_block(&db, node_genesis_block, node_genesis_hash, &filter).await;
        }
        .await;
    }
    println!(" sync::synchronize will be called!!");
    sync::synchronize(&db, &client, &filter).await?;

    log::info!(
        "Wallet database synchronized with node to height {:?}",
        sync::height(&db)?.expect("We just synced, so there is a height available")
    );
    Ok(())
}

async fn sync_and_get_db(db: Arc<Mutex<Db>>) -> anyhow::Result<()> {
    let client = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"))?;

    let keystore_filter = |_v: &OuterVerifier| -> bool { true };
    let db_guard = db.lock().await;
    sync_db(&*db_guard, &client, &keystore_filter).await?;
    Ok(())
}

/// Parse a string into an H256 that represents a public key
pub(crate) fn h256_from_string(s: &str) -> anyhow::Result<H256> {
    let s = strip_0x_prefix(s);

    let mut bytes: [u8; 32] = [0; 32];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H256::from(bytes))
}

use std::error::Error;
/// Parse an output ref from a string
pub(crate) fn convert_output_ref_from_string(s: &str) -> Result<OutputRef, Box<dyn Error>> {
    let s = strip_0x_prefix(s);
    let bytes = hex::decode(s)?;

    OutputRef::decode(&mut &bytes[..]).map_err(|_| "Failed to decode OutputRef from string".into())
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
    /*
    std::env::temp_dir().join(format!(
        "tuxedo-wallet-{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
    ))
    */
    std::env::temp_dir().join(format!("tuxedo-wallet"))
}
/*
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
*/

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

/*
async fn original_get_db() -> anyhow::Result<Db> {

    let client = HttpClientBuilder::default().build(DEFAULT_ENDPOINT)?;

    // Read node's genesis block.
    let node_genesis_hash = rpc::node_get_block_hash(0, &client)
        .await?
        .expect("node should be able to return some genesis hash");
    let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
        .await?
        .expect("node should be able to return some genesis block");
    log::debug!("Node's Genesis block::{:?}", node_genesis_hash);

    // Open the local database
    let data_path = temp_dir();
    let db_path = data_path.join("wallet_database");
    let db = sync::open_db(db_path, node_genesis_hash, node_genesis_block.clone())?;

    let num_blocks =
        sync::height(&db)?.expect("db should be initialized automatically when opening.");
    log::info!("Number of blocks in the db: {num_blocks}");

    // No filter at-all
    let keystore_filter = |_v: &OuterVerifier| -> bool {
        true
    };

    if !sled::Db::was_recovered(&db) {
        println!("!sled::Db::was_recovered(&db) called ");
        // This is a new instance, so we need to apply the genesis block to the database.
        async {
            sync::apply_block(&db, node_genesis_block, node_genesis_hash, &keystore_filter)
            .await;
        };
    }

    sync::synchronize(&db, &client, &keystore_filter).await?;

    println!(
        "Wallet database synchronized with node to height {:?}",
        sync::height(&db)?.expect("We just synced, so there is a height available")
    );

    if let Err(err) = db.flush() {
        println!("Error flushing Sled database: {}", err);
    }

    Ok(db)
}

*/
