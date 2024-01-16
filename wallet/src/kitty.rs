
use crate::{rpc::fetch_storage, sync};
use hex_literal::hex;
//use crate::cli::BreedArgs;
use tuxedo_core::{
    types::{Input, Output, OutputRef},
    verifier::Sr25519Signature,
    verifier::UpForGrabs,
};

use anyhow::anyhow;
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

use crate::cli::MintKittyArgs;

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub async fn mint_kitty(
    db: &Db,
    client: &HttpClient,
    keystore: &LocalKeystore,
    args: MintKittyArgs,
) -> anyhow::Result<()> {

    let parent = Parent::mom();// Selected as 
    let mut array = [0; 4];
    let kitty_name: &[u8; 4] = {
        array.copy_from_slice(args.kitty_name.clone().unwrap().clone().as_bytes());
        &array
    };

    let length = 5; // lets keep the length of random string as 5.

    // Generate a random string of the specified length
    // For now I can mint as many kitties aas possible.
    let random_string = generate_random_string(length)+args.kitty_name.unwrap().as_str(); 
    let dna_preimage: &[u8] = random_string.as_bytes();

    let child_kitty = KittyData {
        parent,
        dna: KittyDNA(BlakeTwo256::hash(dna_preimage)),
        name:*kitty_name,
        ..Default::default()
    };

    let output = Output {
        payload: child_kitty.into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: args.owner,
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

/// Apply a transaction to the local database, storing the new coins.
pub(crate) fn apply_transaction(
    db: &Db,
    tx_hash: <BlakeTwo256 as Hash>::Output,
    index: u32,
    output: &Output<OuterVerifier>,
) -> anyhow::Result<()> {
    let mut kitty_detail: KittyData = output.payload.extract()?;

    let output_ref = OutputRef { tx_hash, index };
    match output.verifier {
        OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey }) => {
            // Add it to the global unspent_outputs table
            crate::sync::add_owned_kitty_to_db(db, &output_ref, &owner_pubkey, &kitty_detail)
        }
        _ => Err(anyhow!("{:?}", ())),
    }
}

pub( crate ) fn get_kitty_name(kitty:&KittyData) -> Option<String> {
    if let Ok(kitty_name) = std::str::from_utf8(&kitty.name) {
        let string_from_array: String = kitty_name.to_string();
        return Some(kitty_name.to_string());
    } else {
        println!("Invalid UTF-8 data in the Kittyname");
    }
    None
}

/*

pub async fn breed_kitty( db: &Db,client: &HttpClient,keystore: &LocalKeystore,
    args: BreedArgs) -> anyhow::Result<()> {
    
    Ok(())
}
*/