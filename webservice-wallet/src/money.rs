//! Wallet features related to spending money and checking balances.

use crate::rpc::fetch_storage;
use anyhow::anyhow;
use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};
use parity_scale_codec::Encode;
use runtime::{
    money::{Coin, MoneyConstraintChecker},
    OuterConstraintChecker, OuterVerifier, Transaction,
};
use sled::Db;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};
use tuxedo_core::{
    types::{Output, OutputRef},
    verifier::Sr25519Signature,
};
//use crate::original_get_db;

/// Create and send a transaction that mints the coins on the network
pub async fn mint_coins(client: &HttpClient, amount: u128, public_key: H256) -> anyhow::Result<()> {
    let transaction = Transaction {
        inputs: Vec::new(),
        peeks: Vec::new(),
        outputs: vec![(
            Coin::<0>::new(amount),
            OuterVerifier::Sr25519Signature(Sr25519Signature {
                owner_pubkey: public_key,
            }),
        )
            .into()],
        checker: OuterConstraintChecker::Money(MoneyConstraintChecker::Mint),
    };

    let spawn_hex = hex::encode(transaction.encode());
    let params = rpc_params![spawn_hex];
    let _spawn_response: Result<String, _> = client.request("author_submitExtrinsic", params).await;

    log::info!(
        "Node's response to mint-coin transaction: {:?}",
        _spawn_response
    );

    let minted_coin_ref = OutputRef {
        tx_hash: <BlakeTwo256 as Hash>::hash_of(&transaction.encode()),
        index: 0,
    };
    let output = &transaction.outputs[0];
    let amount = output.payload.extract::<Coin<0>>()?.0;
    print!(
        "Minted {:?} worth {amount}. ",
        hex::encode(minted_coin_ref.encode())
    );
    crate::pretty_print_verifier(&output.verifier);

    Ok(())
}

/// Given an output ref, fetch the details about this coin from the node's
/// storage.
pub async fn get_coin_from_storage(
    output_ref: &OutputRef,
    client: &HttpClient,
) -> anyhow::Result<(Coin<0>, OuterVerifier)> {
    let utxo = fetch_storage::<OuterVerifier>(output_ref, client).await?;
    let coin_in_storage: Coin<0> = utxo.payload.extract()?;

    Ok((coin_in_storage, utxo.verifier))
}

/// Apply a transaction to the local database, storing the new coins.
pub(crate) fn apply_transaction(
    db: &Db,
    tx_hash: <BlakeTwo256 as Hash>::Output,
    index: u32,
    output: &Output<OuterVerifier>,
) -> anyhow::Result<()> {
    let amount = output.payload.extract::<Coin<0>>()?.0;
    let output_ref = OutputRef { tx_hash, index };
    match output.verifier {
        OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey }) => {
            // Add it to the global unspent_outputs table
            crate::sync::add_unspent_output(db, &output_ref, &owner_pubkey, &amount)
        }
        _ => Err(anyhow!("{:?}", ())),
    }
}
