use crate::rpc::fetch_storage;

//use crate::cli::BreedArgs;
use tuxedo_core::{
    types::{Output, OutputRef,Input},
    verifier::Sr25519Signature,
};

use sc_keystore::LocalKeystore;
use anyhow::anyhow;
use sp_core::sr25519::Public;
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

use crate::cli::{MintKittyArgs,BreedKittyArgs};

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

pub async fn show_kitty_referance(db: &Db) -> anyhow::Result<()> {
    //let kittyRef = sync::get_kittyReferance_set(db, total_output_amount - total_input_amount)?
    
    Ok(())
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
    let mut gender = Parent::mom();
    if args.kitty_gender == "male" {
        gender = Parent::dad();
    }

    // Create the KittyData
    let child_kitty = KittyData {
        parent: gender,
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
    println!( "TransactionHash {:?}. ", <BlakeTwo256 as Hash>::hash_of(&transaction.encode()));

    println!(
        "Minted kitty encoded Ref {:?} with dna {:?}. ", hex::encode(minted_kitty_ref.encode()),
        minted_kitty
    );

    //crate::pretty_print_verifier(&output.verifier);

    Ok(())
}

pub async fn breed_kitty(db: &Db,
    client: &HttpClient, 
    keystore: &LocalKeystore,
    args: BreedKittyArgs) -> anyhow::Result<()> {
    log::info!("The Breed kittyArgs are:: {:?}", args);
    let kitty_mom = crate::sync::get_kitty_from_local_db_based_on_dna1(&db,args.parent_kitty_mom_dna);
    let kitty_dad = crate::sync::get_kitty_from_local_db_based_on_dna1(&db,args.parent_kitty_dad_dna);
    let Some((kitty_mom_info,out_ref_mom)) = kitty_mom.unwrap() else { todo!() };
    let Some((kitty_dad_info,out_ref_dad)) = kitty_dad.unwrap() else { todo!() };
    log::info!("kitty_mom_ref:: {:?}", out_ref_mom);
    log::info!("kitty_dad_ref:: {:?}", out_ref_dad);

    let mom_ref = Input {
        output_ref: out_ref_mom,
        redeemer: vec![], // We will sign the total transaction so this should be empty
    };

    let dad_ref = Input {
        output_ref:  out_ref_dad,
        redeemer: vec![], // We will sign the total transaction so this should be empty
    };
    
    let mut inputs: Vec<Input> = vec![];
    inputs.push(mom_ref);
    inputs.push(dad_ref);
    
    let mut new_mom: KittyData = kitty_mom_info;
    new_mom.parent = Parent::Mom(MomKittyStatus::HadBirthRecently);
    new_mom.num_breedings += 1;
    new_mom.free_breedings -= 1;

    // Create the Output mom
    let output_mom = Output {
        payload: new_mom.clone().into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: args.owner,
        }),
    };

    let mut new_dad = kitty_dad_info;
    new_dad.parent = Parent::Dad(DadKittyStatus::Tired);
    new_dad.num_breedings += 1;
    new_dad.free_breedings -= 1;
    // Create the Output dada
    let output_dad = Output {
        payload: new_dad.clone().into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: args.owner,
        }),
    };

    let child = KittyData {
        parent: Parent::Mom(MomKittyStatus::RearinToGo),
        free_breedings: 2,
        name:*b"tomy",
        dna: KittyDNA(BlakeTwo256::hash_of(&(
            new_mom.dna.clone(),
            new_dad.dna.clone(),
            new_mom.num_breedings ,
            new_dad.num_breedings ,
        ))),
        num_breedings: 0,
    };
    println!("New mom Dna = {:?}",new_mom.dna);
    println!("New Dad Dna = {:?}",new_dad.dna);
    println!("Child Dna = {:?}",child.dna);
    // Create the Output child
    let output_child = Output {
        payload: child.clone().into(),
        verifier: OuterVerifier::Sr25519Signature(Sr25519Signature {
            owner_pubkey: args.owner,
        }),
    };

    let new_family = Box::new(vec![output_mom, output_dad, output_child]);
    
     // Create the Transaction
     let mut transaction = Transaction {
        inputs: inputs,
        peeks: Vec::new(),
        outputs: (&[
            new_family[0].clone(),
            new_family[1].clone(),
            new_family[2].clone(),
        ]).to_vec(),
        checker: FreeKittyConstraintChecker::Breed.into(),
    };

    // Keep a copy of the stripped encoded transaction for signing purposes
    let stripped_encoded_transaction = transaction.clone().encode();

    // Iterate back through the inputs, signing, and putting the signatures in place.
    for input in &mut transaction.inputs {
        // Fetch the output from storage
        let utxo = fetch_storage::<OuterVerifier>(&input.output_ref, client).await?;

        // Construct the proof that it can be consumed
        let redeemer = match utxo.verifier {
            OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey }) => {
                let public = Public::from_h256(owner_pubkey);
                crate::keystore::sign_with(keystore, &public, &stripped_encoded_transaction)?
            }
            OuterVerifier::UpForGrabs(_) => Vec::new(),
            OuterVerifier::ThresholdMultiSignature(_) => todo!(),
        };

        // insert the proof
        input.redeemer = redeemer;
    }

    // Encode the transaction
    let spawn_hex = hex::encode(transaction.encode());

    // Send the transaction to the node
    let params = rpc_params![spawn_hex];
    let spawn_response: Result<String, _> = client.request("author_submitExtrinsic", params).await;
    println!("Node's response to spawn transaction: {:?}", spawn_response);

    // Print new output refs for user to check later
    let tx_hash = <BlakeTwo256 as Hash>::hash_of(&transaction.encode());
    for (i, output) in transaction.outputs.iter().enumerate() {
        let new_kitty_ref = OutputRef {
            tx_hash,
            index: i as u32,
        };
        let new_kitty = output.payload.extract::<KittyData>()?.dna.0;

        print!(
            "Created {:?} worth {:?}. ",
            hex::encode(new_kitty_ref.encode()),new_kitty
        );
        crate::pretty_print_verifier(&output.verifier);
    }

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
    println!("in kitty:apply_transaction output_ref = {:?}",output_ref);
    match output.verifier {
        OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey }) => {
            // Add it to the global unspent_outputs table
            crate::sync::add_fresh_kitty_to_db(db, &output_ref, &owner_pubkey, &kitty_detail)
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
