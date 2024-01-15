use crate::rpc::fetch_storage;
use hex_literal::hex;
//use crate::cli::BreedArgs;
use tuxedo_core::{
    types::{Input, Output, OutputRef},
    verifier::Sr25519Signature,
    verifier::UpForGrabs,
};

use std::{thread::sleep, time::Duration};

use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use parity_scale_codec::Encode;
use sc_keystore::LocalKeystore;
use sled::Db;
use rand::Rng;
use rand::distributions::Alphanumeric;
use sp_runtime::traits::{BlakeTwo256, Hash};

use runtime::{
    kitties::{KittyData, Parent,KittyHelpers,MomKittyStatus,DadKittyStatus,
        KittyDNA,FreeKittyConstraintChecker,KittyConstraintChecker},
    OuterVerifier, Transaction,
};

const SHAWN_PUB_KEY_BYTES: [u8; 32] =
    hex!("d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67");

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub async fn mint_kitty(client: &HttpClient,name: Option<String>,keystore: &LocalKeystore,) -> anyhow::Result<()> {

    let parent = Parent::mom();// Selected as 
    let random_str = name.clone().unwrap();
    let mut array = [0; 4];
    let kitty_name: &[u8; 4] = {
        array.copy_from_slice(name.clone().unwrap().clone().as_bytes());
        &array
    };

    let length = 5; // lets keep the length of random string as 10.

    // Generate a random string of the specified length
    // For now I can mint as many kitties aas possible.
    let random_string = generate_random_string(length)+name.unwrap().as_str(); 
    let dna_preimage: &[u8] = random_str.as_bytes();

    let child_kitty = KittyData {
        parent,
        dna: KittyDNA(BlakeTwo256::hash(dna_preimage)),
        name:*kitty_name,
        ..Default::default()
    };

    let output = Output {
        payload: child_kitty.into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: sp_core::H256(SHAWN_PUB_KEY_BYTES),
        }),
    };

    let transaction = Transaction {
        inputs: Vec::new(),
        peeks: Vec::new(),
        outputs: vec![output],
        checker: KittyConstraintChecker::Mint.into(),
    };
    
    let child_kitty_ref = OutputRef {
        tx_hash: <BlakeTwo256 as Hash>::hash_of(&transaction.encode()),
        index: 0,
    };

    let spawn_hex = hex::encode(transaction.encode());
    let params = rpc_params![spawn_hex];
    let spawn_response: Result<String, _> = client.request("author_submitExtrinsic", params).await;

    println!("Node's response to spawn transaction: {:?}", spawn_response);

    sleep(Duration::from_secs(3));

    let child_kitty_from_storage: KittyData = fetch_storage::<OuterVerifier>(&child_kitty_ref, client)
        .await?
        .payload
        .extract()?;

    println!("Child kitty retrieved from storage: {:?}", child_kitty_from_storage);

    Ok(())
}
/*

pub async fn breed_kitty( db: &Db,client: &HttpClient,keystore: &LocalKeystore,
    args: BreedArgs) -> anyhow::Result<()> {
    
    Ok(())
}
*/