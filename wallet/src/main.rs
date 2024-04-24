//! A simple CLI wallet. For now it is a toy just to start testing things out.

use clap::Parser;
use jsonrpsee::http_client::HttpClientBuilder;
use parity_scale_codec::{Decode, Encode};
use runtime::OuterVerifier;
use std::path::PathBuf;
use tuxedo_core::{types::OutputRef, verifier::*};

use sp_core::H256;
use crate::keystore::SHAWN_PUB_KEY;

//mod amoeba;
mod cli;
mod keystore;
mod kitty;
mod money;
mod output_filter;
mod rpc;
mod sync;
mod timestamp;

use cli::{Cli, Command};

/// The default RPC endpoint for the wallet to connect to
const DEFAULT_ENDPOINT: &str = "http://localhost:9944";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line args
    let cli = Cli::parse();
    log::info!("cli from cmd args = {:?}", cli);

    // If the user specified --tmp or --dev, then use a temporary directory.
    let tmp = cli.tmp || cli.dev;

    // Setup the data paths.
    let data_path = match tmp {
        true => temp_dir(),
        _ => cli.path.unwrap_or_else(default_data_path),
    };
    let keystore_path = data_path.join("keystore");
    let db_path = data_path.join("wallet_database");

    // Setup the keystore
    let keystore = sc_keystore::LocalKeystore::open(keystore_path.clone(), None)?;

    crate::keystore::insert_development_key_for_this_session(&keystore)?;

    if cli.dev {
        // Insert the example Shawn key so example transactions can be signed.
        log::info!("cli is dv");
        crate::keystore::insert_development_key_for_this_session(&keystore)?;
    }

    // Setup jsonrpsee and endpoint-related information.
    // https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/http.rs
    let client = HttpClientBuilder::default().build(cli.endpoint)?;

    // Read node's genesis block.
    let node_genesis_hash = rpc::node_get_block_hash(0, &client)
        .await?
        .expect("node should be able to return some genesis hash");
    let node_genesis_block = rpc::node_get_block(node_genesis_hash, &client)
        .await?
        .expect("node should be able to return some genesis block");
    log::debug!("Node's Genesis block::{:?}", node_genesis_hash);

    // Open the local database
    let db = sync::open_db(db_path, node_genesis_hash, node_genesis_block.clone())?;

    let num_blocks =
        sync::height(&db)?.expect("db should be initialized automatically when opening.");
    log::info!("Number of blocks in the db: {num_blocks}");

    // The filter function that will determine whether the local database should track a given utxo
    // is based on whether that utxo is privately owned by a key that is in our keystore.
    let keystore_filter = |v: &OuterVerifier| -> bool {
        matches![v,
            OuterVerifier::Sr25519Signature(Sr25519Signature { owner_pubkey })
                if crate::keystore::has_key(&keystore, owner_pubkey)
        ] || matches![v, OuterVerifier::UpForGrabs(UpForGrabs)] // used for timestamp
    };

    if !sled::Db::was_recovered(&db) {
        // This is a new instance, so we need to apply the genesis block to the database.
        sync::apply_block(&db, node_genesis_block, node_genesis_hash, &keystore_filter).await?;
    }

    // Synchronize the wallet with attached node unless instructed otherwise.
    if cli.no_sync {
        log::warn!("Skipping sync with node. Using previously synced information.")
    } else {
        sync::synchronize(&db, &client, &keystore_filter).await?;

        log::info!(
            "Wallet database synchronized with node to height {:?}",
            sync::height(&db)?.expect("We just synced, so there is a height available")
        );
    }

    // Dispatch to proper subcommand
    match cli.command {
        Some(Command::GetBlock { block_height }) => {
            let node_hash = rpc::node_get_block_hash(block_height.unwrap(), &client)
                .await?
                .expect("node should be able to return some  hash");
            let node_block = rpc::node_get_block(node_hash, &client)
                .await?
                .expect("node should be able to return some genesis block");
            log::info!("Node's  block hash ::{:?}", node_hash);
            log::info!("Node's  block::{:?}", node_block);
            Ok(())
        }

        // Some(Command::AmoebaDemo) => amoeba::amoeba_demo(&client).await,
        // Command::MultiSigDemo => multi_sig::multi_sig_demo(&client).await,
        Some(Command::MintCoins(args)) => money::mint_coins(&client, args).await,

        Some(Command::VerifyCoin { output_ref }) => {
            println!("Details of coin {}:", hex::encode(output_ref.encode()));

            // Print the details from storage
            let (coin_from_storage, verifier_from_storage) =
                money::get_coin_from_storage(&output_ref, &client).await?;
            print!("Found in storage.  Value: {}, ", coin_from_storage.0);
            pretty_print_verifier(&verifier_from_storage);

            // Print the details from the local db
            match sync::get_unspent(&db, &output_ref)? {
                Some((owner, amount)) => {
                    println!("Found in local db. Value: {amount}, owned by {owner}");
                }
                None => {
                    println!("Not found in local db");
                }
            }

            Ok(())
        }

        Some(Command::VerifyKitty { output_ref }) => {
            println!("Details of Kitty {}:", hex::encode(output_ref.encode()));

            // Print the details from storage
            let (kitty_from_storage, verifier_from_storage) =
                kitty::get_kitty_from_storage(&output_ref, &client).await?;
            print!(
                "Found in storage.  Kitty-Name:{:?}  Dna-Value: {}, ",
                kitty::convert_kitty_name_string(&kitty_from_storage),
                kitty_from_storage.dna.0
            );
            pretty_print_verifier(&verifier_from_storage);

            // Print the details from the local db
            match sync::get_kitty_fromlocaldb(&db, &output_ref)? {
                Some((owner, kitty)) => {
                    println!("Found in local db. Value: {kitty}, owned by {owner}");
                }
                None => {
                    println!("Not found in local db");
                }
            }
            Ok(())
        }
        Some(Command::SpendCoins(args)) => money::spend_coins(&db, &client, &keystore, args).await,
        Some(Command::InsertKey { seed }) => crate::keystore::insert_key(&keystore, &seed),
        Some(Command::GenerateKey { password }) => {
            crate::keystore::generate_key(&keystore, password)?;
            Ok(())
        }
        Some(Command::ShowKeys) => {
            crate::keystore::get_keys(&keystore)?.for_each(|pubkey| {
                println!("key: 0x{}", hex::encode(pubkey));
            });

            Ok(())
        }
        Some(Command::RemoveKey { pub_key }) => {
            println!("CAUTION!!! About permanently remove {pub_key}. This action CANNOT BE REVERSED. Type \"proceed\" to confirm deletion.");

            let mut confirmation = String::new();
            std::io::stdin()
                .read_line(&mut confirmation)
                .expect("Failed to read line");

            if confirmation.trim() == "proceed" {
                crate::keystore::remove_key(&keystore_path, &pub_key)
            } else {
                println!("Deletion aborted. That was close.");
                Ok(())
            }
        }
        Some(Command::ShowBalance) => {
            println!("Balance Summary");
            let mut total = 0;
            let balances = sync::get_balances(&db)?;
            for (account, balance) in balances {
                total += balance;
                println!("{account}: {balance}");
            }
            println!("--------------------");
            println!("total      : {total}");

            Ok(())
        }
        Some(Command::ShowAllOutputs) => {
            println!("###### Unspent outputs ###########");
            sync::print_unspent_tree(&db)?;

            Ok(())
        }
        Some(Command::ShowTimestamp) => {
            println!("Timestamp: {}", timestamp::get_timestamp(&db)?);
            Ok(())
        }

        Some(Command::CreateKitty(args)) => kitty::create_kitty(&client, args).await,
        Some(Command::BreedKitty(args)) => kitty::breed_kitty(&db, &client, &keystore, args).await,
        Some(Command::ListKittyForSale(args)) => {
            kitty::list_kitty_for_sale(&db, &client, &keystore, args).await
        }
        Some(Command::DelistKittyFromSale(args)) => {
            kitty::delist_kitty_for_sale(&db, &client, &keystore, args).await
        }
        Some(Command::UpdateKittyName(args)) => {
            kitty::update_kitty_name(&db, &client, &keystore, args).await
        }
        Some(Command::UpdateKittyPrice(args)) => {
            kitty::update_kitty_price(&db, &client, &keystore, args).await
        }
        Some(Command::BuyKitty(args)) => kitty::buy_kitty(&db, &client, &keystore, args).await,
        Some(Command::ShowAllKitties) => {
            println!("Show All Kitty Summary");
            println!("==========================================");

            let owned_kitties = sync::get_all_kitties_from_local_db(&db)?;

            for (owner, kitty_data) in owned_kitties {
                println!("Owner -> {:?}", owner);
                println!(
                    "{:?} => {:?} -> ",
                    kitty::convert_kitty_name_string(&kitty_data),
                    kitty_data
                );
                println!("--------------------------------------------------");
            }
            println!("=-===================================================");
            let owned_tradable_kitties = sync::get_all_tradable_kitties_from_local_db(&db)?;
            for (owner, kitty_data) in owned_tradable_kitties {
                println!("Owner -> {:?}", owner);
                println!(
                    "{:?} => {:?} -> ",
                    kitty::convert_td_kitty_name_string(&kitty_data),
                    kitty_data
                );
                println!("--------------------------------------------------");
            }
            println!("=-===================================================");
            Ok(())
        }
        Some(Command::ShowOwnedKitties(args)) => {
            println!("ShowOwnedKitties Kitty Summary");
            println!("==========================================");
            let owned_kitties = sync::get_owned_kitties_from_local_db(&db, &args)?;
            for (_owner, kitty_data, _output_ref) in owned_kitties {
                println!(
                    "{:?} => {:?} -> ",
                    kitty::convert_kitty_name_string(&kitty_data),
                    kitty_data
                );
                println!("--------------------------------------------------");
            }
            println!("=-===================================================");
            let owned_tradable_kitties =
                sync::get_owned_tradable_kitties_from_local_db(&db, &args)?;
            for (_owner, kitty_data, _output_ref) in owned_tradable_kitties {
                println!(
                    "{:?} => {:?} -> ",
                    kitty::convert_td_kitty_name_string(&kitty_data),
                    kitty_data
                );
                println!("--------------------------------------------------");
            }
            println!("=-===================================================");
            println!("++++++++++++++++++++++++++++++++++++++++++++++++");

            Ok(())
        }

        None => {
            log::info!("No Wallet Command invoked. Exiting.");
            Ok(())
        }
    }?;

    if tmp {
        // Cleanup the temporary directory.
        std::fs::remove_dir_all(data_path.clone()).map_err(|e| {
            log::warn!(
                "Unable to remove temporary data directory at {}\nPlease remove it manually.",
                data_path.to_string_lossy()
            );
            e
        })?;
    }

    Ok(())
}

fn parse_recipient_coins(s: &str) -> Result<(H256, Vec<u128>), &'static str> {
    //    println!("In parse_recipient_coins");
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() >= 2 {
            let recipient = h256_from_string(parts[0]);
            match recipient {
                Ok(r) => {
                    let coins = parts[1..].iter().filter_map(|&c| c.parse().ok()).collect();
                    println!("Recipient: {}", r);
                    return Ok((r, coins));
                }
                _ => {
     //               println!("For default Recipient");
                    let coins = parts[0..].iter().filter_map(|&c| c.parse().ok()).collect();
                    return Ok((h256_from_string_without_stripping(SHAWN_PUB_KEY).unwrap(), coins));
                }
            };
        }
      //  println!("Sending the error value ");
        Err("Invalid input format")
    }

/// Parse a string into an H256 that represents a public key
pub(crate) fn h256_from_string(s: &str) -> anyhow::Result<H256> {
    let st:&str;
    if s.len() > 2{
        st = strip_0x_prefix(s);
    } else {
        st = s;
    }
    let s = st;
    let mut bytes: [u8; 32] = [0; 32];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H256::from(bytes))
}

pub(crate) fn h256_from_string_without_stripping(s: &str) -> anyhow::Result<H256> {
    let mut bytes: [u8; 32] = [0; 32];
    hex::decode_to_slice(s, &mut bytes as &mut [u8])
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
    Ok(H256::from(bytes))
}

/// Parse an output ref from a string
fn output_ref_from_string(s: &str) -> Result<OutputRef, clap::Error> {
    let st:&str;
    if s.contains("0x") {
        st = strip_0x_prefix(s);
    } else {
        st = s;
    }
    let s = st;
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
