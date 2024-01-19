use crate::rpc::fetch_storage;

//use crate::cli::BreedArgs;
use tuxedo_core::{
    types::{Output, OutputRef},
    verifier::Sr25519Signature,
};

use anyhow::anyhow;
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use parity_scale_codec::Encode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sled::Db;
use sp_runtime::traits::{BlakeTwo256, Hash};

use runtime::{
    kitties::{
        DadKittyStatus, FreeKittyConstraintChecker, KittyDNA, KittyData, KittyHelpers,
        MomKittyStatus, Parent,
    },
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

pub async fn mint_kitty(client: &HttpClient, args: MintKittyArgs) -> anyhow::Result<()> {
    // Check the length of the kitty_name
    if args.kitty_name.len() != 4 {
        return Err(anyhow!(
            "Please input a name of length 4 characters. Current length: {}",
            args.kitty_name.len()
        ));
    }
    let mut array = [0; 4];
    let kitty_name: &[u8; 4] = {
        array.copy_from_slice(args.kitty_name.clone().as_bytes());
        &array
    };

    // Generate a random string of length 5
    let random_string = generate_random_string(5) + args.kitty_name.as_str();
    let dna_preimage: &[u8] = random_string.as_bytes();

    // Create the KittyData
    let child_kitty = KittyData {
        parent: Parent::mom(),
        dna: KittyDNA(BlakeTwo256::hash(dna_preimage)),
        name: *kitty_name, // Default value for now, as the provided code does not specify a value
        ..Default::default()
    };

    // Create the Output
    let output = Output {
        payload: child_kitty.into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: args.owner,
        }),
    };

    // Create the Transaction
    let transaction = Transaction {
        inputs: Vec::new(),
        peeks: Vec::new(),
        outputs: vec![output],
        checker: FreeKittyConstraintChecker::Mint.into(),
    };

    // Encode the transaction
    let spawn_hex = hex::encode(transaction.encode());

    // Send the transaction to the node
    let params = rpc_params![spawn_hex];
    let spawn_response: Result<String, _> = client.request("author_submitExtrinsic", params).await;

    println!("Node's response to spawn transaction: {:?}", spawn_response);

    // Extract information about the minted kitty
    let minted_kitty_ref = OutputRef {
        tx_hash: <BlakeTwo256 as Hash>::hash_of(&transaction.encode()),
        index: 0,
    };

    let output = &transaction.outputs[0];
    let minted_kitty = output.payload.extract::<KittyData>()?.dna.0;
    println!(
        "Minted kitty reference {:?} with Name {:?}. ",
        hex::encode(minted_kitty_ref.encode()),
        minted_kitty
    );

    crate::pretty_print_verifier(&output.verifier);

    Ok(())
}

/// Apply a transaction to the local database, storing the new coins.
pub(crate) fn apply_transaction(
    db: &Db,
    tx_hash: <BlakeTwo256 as Hash>::Output,
    index: u32,
    output: &Output<OuterVerifier>,
) -> anyhow::Result<()> {
    let kitty_detail: KittyData = output.payload.extract()?;

    let output_ref = OutputRef { tx_hash, index };
    match output.verifier {
        OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey }) => {
            // Add it to the global unspent_outputs table
            crate::sync::add_owned_kitty_to_db(db, &output_ref, &owner_pubkey, &kitty_detail)
        }
        _ => Err(anyhow!("{:?}", ())),
    }
}

pub(crate) fn get_kitty_name(kitty: &KittyData) -> Option<String> {
    if let Ok(kitty_name) = std::str::from_utf8(&kitty.name) {
        return Some(kitty_name.to_string());
    } else {
        println!("Invalid UTF-8 data in the Kittyname");
    }
    None
}

/// Given an output ref, fetch the details about this coin from the node's
/// storage.
pub async fn get_kitty_from_storage(
    output_ref: &OutputRef,
    client: &HttpClient,
) -> anyhow::Result<(KittyData, OuterVerifier)> {
    let utxo = fetch_storage::<OuterVerifier>(output_ref, client).await?;
    let kitty_in_storage: KittyData = utxo.payload.extract()?;

    Ok((kitty_in_storage, utxo.verifier))
}

/*

pub async fn breed_kitty( db: &Db,client: &HttpClient,keystore: &LocalKeystore,
    args: BreedArgs) -> anyhow::Result<()> {

    Ok(())
}
*/
