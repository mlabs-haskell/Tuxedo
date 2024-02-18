//! This module defines TradableKitty, a specialized type of kitty with additional features.
//!
//! TradableKitties are designed for trading, and they extend the functionality of the basic kitty.
//! Key features of TradableKitties include:
//! - `ListKittyForSale`: Basic kitties are converted to tradable kitties, with additional feild Price.
//! - `DelistKittyFromSale`: Trdabale kitties are converted to Kitties when owner dont want to sell them.
//! - `UpdateKittyPrice`: Owner can update the price of Tradablekitty.
//! - `UpdateKittyName` : Owner can update the name of tradableKitty
//! - `Buy`: Enable users to purchase TradableKitties from others, facilitating secure and fair exchanges.
//!
//!
//! TradableKitties provide an enhanced user experience by introducing trading capabilities.


#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{
    transaction_validity::TransactionPriority,
};
use sp_std::prelude::*;
use sp_std::collections::btree_map::BTreeMap;
use tuxedo_core::{
    dynamic_typing::{DynamicallyTypedData, UtxoData},
    ensure,
    SimpleConstraintChecker,
};
//use kitties::KittyHelpers;
use money::{Coin, MoneyConstraintChecker};
use kitties::{KittyData,KittyDNA};
use money::ConstraintCheckerError as MoneyError;
use kitties::ConstraintCheckerError as KittyError;


#[cfg(test)]
mod tests;

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
pub struct TradableKittyData {
    pub kitty_basic_data: KittyData,
    pub price: Option<u128>,
}

impl Default for TradableKittyData {
    fn default() -> Self {
        Self {
            kitty_basic_data: KittyData::default(),
            price: None,
        }
    }
}

impl TryFrom<&DynamicallyTypedData> for TradableKittyData {
    type Error = TradeableKittyError;
    fn try_from(a: &DynamicallyTypedData) -> Result<Self, Self::Error> {
        a.extract::<TradableKittyData>()
            .map_err(|_| TradeableKittyError::BadlyTyped)
    }
}

impl UtxoData for TradableKittyData {
    const TYPE_ID: [u8; 4] = *b"tdkt";
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]

pub enum TradeableKittyError {
    /// Error in the underlying money piece.
    MoneyError(money::ConstraintCheckerError),
    /// Error in the underlying kitty piece.
    KittyError(kitties::ConstraintCheckerError),
    /// Dynamic typing issue.
    /// This error doesn't discriminate between badly typed inputs and outputs.
    BadlyTyped,
    /// output missing updating nothing.
    OutputUtxoMissingError,
    /// No input for kitty Update.
    OutputMissingListingNothingForSale,
    /// Not enough amount to buy kitty
    InsufficientCollateralToBuyKitty,
    /// No input for kitty Update.
    NumberOfInputOutputMismatch,
    /// No input for kitty Update.
    KittyBasicPropertiesAltered,
    /// Kitty not avilable for sale
    KittyNotForSale,
    /// Kitty price cant be none when it is avilable for sale
    KittyPriceCantBeNone,
    /// No input for kitty Update.
    KittyPriceUnaltered,
  }

impl From<money::ConstraintCheckerError> for TradeableKittyError {
    fn from(error: money::ConstraintCheckerError) -> Self {
        TradeableKittyError::MoneyError(error)
    }
}

impl From<kitties::ConstraintCheckerError> for TradeableKittyError {
    fn from(error: kitties::ConstraintCheckerError) -> Self {
        TradeableKittyError::KittyError(error)
    }
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
pub enum TradableKittyConstraintChecker<const ID: u8>   {
    /// List kitty for sale, means kitty will converted to tradable kitty once transaction is executed 
    ListKittyForSale,
    /// Delist kitty from sale, means tradable kitty will converted back to kitty  
    DelistKittyFromSale,
    /// Update price of tradable kitty.
    UpdateKittyPrice,
    // Update name of kitty 
    UpdateKittyName,
    /// Fo buying a new kitty from others
    Buy,
}
/*
#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
enum Kitty_update_properties {
    /// For updating the tradable kitty name 
    KittyName,
    /// For updating the tradable kitty price
    KittyPrice,
}
*/

fn extract_basic_kitty_List(tradable_kitty_data: &[DynamicallyTypedData], 
    kitty_data_list: &mut Vec<DynamicallyTypedData>) -> Result<(), TradeableKittyError>{
    for utxo in tradable_kitty_data {
        if let Ok(tradable_kitty) = utxo.extract::<TradableKittyData>() {
            log::info!("TradableKittyConstraintChecker found kitty in i/p {:?}",tradable_kitty);
            kitty_data_list.push(tradable_kitty.kitty_basic_data.clone().into());
        } else {
            log::error!("TradableKittyConstraintChecker found something else i/p {:?}",utxo);
            return Err(TradeableKittyError::BadlyTyped);
        }
    }
    Ok(())
}

fn check_can_buy<const ID: u8> (
    input_data: &[DynamicallyTypedData],
    output_data: &[DynamicallyTypedData]) -> Result<(), TradeableKittyError> {

    
    let mut input_coin_data: Vec<DynamicallyTypedData> = Vec::new();
    let mut output_coin_data: Vec<DynamicallyTypedData> = Vec::new();
    let mut input_kitty_data: Vec<DynamicallyTypedData> = Vec::new();
    let mut output_kitty_data: Vec<DynamicallyTypedData> = Vec::new();

    let mut total_input_amount: u128 = 0;
    let mut total_price_of_kitty: u128 = 0;

    // Map to verify that output_kitty is same as input_kitty based on the dna after buy operation
    let mut dna_to_tdkitty_map: BTreeMap<KittyDNA, TradableKittyData> = BTreeMap::new();
    
    // Seperate the coin and tdkitty in to seperate vecs from the input_data .
    for utxo in input_data {
        if let Ok(coin) = utxo.extract::<Coin<ID>>() {
            log::info!("TradableKittyConstraintChecker found coin in i/p {:?}",coin);
            let utxo_value = coin.0;
            
            ensure!(utxo_value > 0, TradeableKittyError::MoneyError(MoneyError::ZeroValueCoin));
            input_coin_data.push(utxo.clone());
            total_input_amount = total_input_amount.checked_add(utxo_value)
            .ok_or(TradeableKittyError::MoneyError(MoneyError::ValueOverflow))?;
            
            // Process Kitty
        } else if let Ok(td_input_kitty) = utxo.extract::<TradableKittyData>() {
            log::info!("TradableKittyConstraintChecker found kitty in i/p {:?}",td_input_kitty);
            
            // Trying to buy kitty which is not listed for sale.
            let price = match td_input_kitty.price {
                None => return Err(TradeableKittyError::KittyNotForSale),
                Some(p) => p
            };

            input_kitty_data.push(utxo.clone());
            dna_to_tdkitty_map.insert(td_input_kitty.clone().kitty_basic_data.dna, td_input_kitty);
            total_price_of_kitty = total_price_of_kitty.checked_add(price)
            .ok_or( TradeableKittyError::MoneyError(MoneyError::ValueOverflow))?;
        } else {
            log::error!("TradableKittyConstraintChecker found something else i/p {:?}",utxo);
            return Err(TradeableKittyError::BadlyTyped);
        }
    }

    // Seperate the coin and tdkitty in to seperate vecs from the output_data .
    for utxo in output_data {
        if let Ok(coin) = utxo.extract::<Coin<ID>>() {
            log::info!("TradableKittyConstraintChecker found coin in o/p {:?}",coin);
            let utxo_value = coin.0;
            ensure!(utxo_value > 0, TradeableKittyError::MoneyError(MoneyError::ZeroValueCoin));
            output_coin_data.push(utxo.clone());
            // Process Coin
        } else if let Ok(td_output_kitty) = utxo.extract::<TradableKittyData>() {
            log::info!("TradableKittyConstraintChecker found kitty in o/p {:?}",td_output_kitty);
            match dna_to_tdkitty_map.get(&td_output_kitty.kitty_basic_data.dna) {
                Some(found_kitty) => {
                    // After buy opertaion, basic kitty properties cant be updated.
                    ensure!(
                        found_kitty.kitty_basic_data == td_output_kitty.kitty_basic_data, // basic kitty data is unaltered 
                        TradeableKittyError::KittyBasicPropertiesAltered // this need to be chan
                    );
                }
                None => {
                    return Err(TradeableKittyError::OutputUtxoMissingError);
                }
            }
            output_kitty_data.push(utxo.clone());
        } else {
            log::error!("TradableKittyConstraintChecker found something else in o/p  {:?}",utxo);
            return Err(TradeableKittyError::BadlyTyped);
        }
    }

    ensure!(
        input_kitty_data.len() == output_kitty_data.len() && !input_kitty_data.is_empty(), {
            log::warn!("input_data.len() = {:?}  and output_data.len() {:?}",input_kitty_data.len(),output_kitty_data.len());
            TradeableKittyError::NumberOfInputOutputMismatch
        }
    );

    log::info!("InsufficientCollateralToBuyKitty  total_price_of_kitty = {:?}total_input_amount = {:?}",
    total_price_of_kitty,total_input_amount);
    ensure!(total_price_of_kitty <= total_input_amount,
        TradeableKittyError::InsufficientCollateralToBuyKitty
    );

    // Filterd coins sent to MoneyConstraintChecker for money validation.
    MoneyConstraintChecker::<0>::Spend.check(&input_coin_data, &[], &output_coin_data)?;
    Ok(())
}

fn check_tdkitty_price_update(
    input_data: &[DynamicallyTypedData],
    output_data: &[DynamicallyTypedData]) -> Result<TransactionPriority, TradeableKittyError> {
    ensure!(
        input_data.len() == output_data.len() && !input_data.is_empty(), {
            log::warn!("input_data.len() = {:?}  and output_data.len() {:?}",input_data.len(),output_data.len());
            TradeableKittyError::NumberOfInputOutputMismatch
        }
    );

    let mut dna_to_tdkitty_map: BTreeMap<KittyDNA, TradableKittyData> = BTreeMap::new();

    for utxo in input_data { 
        let td_input_kitty = utxo
            .extract::<TradableKittyData>()
            .map_err(|_| TradeableKittyError::BadlyTyped)?;
        dna_to_tdkitty_map.insert(td_input_kitty.clone().kitty_basic_data.dna, td_input_kitty);
    }

    for utxo in output_data {
        let td_output_kitty = utxo
            .extract::<TradableKittyData>()
            .map_err(|_| TradeableKittyError::BadlyTyped)?;

        if let Some(found_kitty) = dna_to_tdkitty_map.get(&td_output_kitty.kitty_basic_data.dna) {
            // Element found, access the value
            log::info!("Found value: {:?}", found_kitty);
            ensure!(
                found_kitty.kitty_basic_data == td_output_kitty.kitty_basic_data, // basic kitty data is unaltered 
                TradeableKittyError::KittyBasicPropertiesAltered // this need to be chan
            );
            let kitty_price = match td_output_kitty.price {
                Some(price) => { 
                    ensure!(
                        found_kitty.price != td_output_kitty.price, // kitty ptice is unaltered 
                        TradeableKittyError::KittyPriceUnaltered // this need to be chan
                    );
                },
                None => return Err(TradeableKittyError::KittyPriceCantBeNone),
            };
        } else {
            return Err(TradeableKittyError::OutputUtxoMissingError);
        }
    }
    return Ok(0);
}


// Wrapper function for checking conversion from basic kitty to tradable kitty.
fn check_can_list_kitty_for_sale(
    input_data: &[DynamicallyTypedData],
    output_data: &[DynamicallyTypedData]) -> Result<TransactionPriority, TradeableKittyError> {
        
    check_kitty_tdkitty_interconversion(&input_data,&output_data)?;
    return Ok(0);
}

// Wrapper function for checking conversion from  tradable kitty to basic kitty.
fn check_can_delist_kitty_from_sale(
    input_data: &[DynamicallyTypedData],
    output_data: &[DynamicallyTypedData]) -> Result<TransactionPriority, TradeableKittyError> {
    // Below is conversion from tradable kitty to kitty, reverse of the ListKittyForSale, 
    // hence input params are rebversed
    check_kitty_tdkitty_interconversion(&output_data,&input_data)?;
    return Ok(0);
}


///  validaes inter-conversion b/w both kitty & tradable kitty.Used by listForSale & delistFromSale functions .
fn check_kitty_tdkitty_interconversion(
    kitty_data: &[DynamicallyTypedData],
    tradable_kitty_data: &[DynamicallyTypedData]) -> Result<TransactionPriority, TradeableKittyError> {

    ensure!(
        kitty_data.len() == tradable_kitty_data.len() && !kitty_data.is_empty(), {
            log::warn!("kitty_data.len() = {:?}  and tradable_kitty_data.len() {:?}",kitty_data.len(),tradable_kitty_data.len());
            TradeableKittyError::NumberOfInputOutputMismatch
        }
    );

    let mut map: BTreeMap<KittyDNA, KittyData> = BTreeMap::new();

    for utxo in kitty_data { 
        let utxo_kitty = utxo
            .extract::<KittyData>()
            .map_err(|_| TradeableKittyError::BadlyTyped)?;
            map.insert(utxo_kitty.clone().dna, utxo_kitty);
    }

    for utxo in tradable_kitty_data {
        let utxo_tradable_kitty = utxo
            .extract::<TradableKittyData>()
            .map_err(|_| TradeableKittyError::BadlyTyped)?;

        if let Some(kitty) = map.get(&utxo_tradable_kitty.kitty_basic_data.dna) {
            // Element found, access the value
            log::info!("Found value: {:?}", kitty);
            ensure!(
                *kitty == utxo_tradable_kitty.kitty_basic_data, // basic kitty data is unaltered 
                TradeableKittyError::KittyBasicPropertiesAltered // this need to be chan
            );
            let kitty_price = match utxo_tradable_kitty.price {
                Some(price) => { log::info!("price of tradable kitty {:?}", price); },
                None => return Err(TradeableKittyError::KittyPriceCantBeNone),
            };
        } else {
            return Err(TradeableKittyError::OutputMissingListingNothingForSale);
        }
    }
    return Ok(0);
}

impl<const ID: u8>  SimpleConstraintChecker for TradableKittyConstraintChecker<ID>  {
   type Error = TradeableKittyError;

    fn check(
        &self,
        input_data: &[DynamicallyTypedData],
        _peeks: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData],
    ) -> Result<TransactionPriority, Self::Error> {
        log::info!("TradableKittyConstraintChecker called ");
        match &self {
            Self::ListKittyForSale => {
                //  Below is conversion from kitty to tradable kitty
                check_can_list_kitty_for_sale(&input_data,&output_data)?;
                return Ok(0);
            }
            Self::DelistKittyFromSale => {
                check_can_delist_kitty_from_sale(&input_data,&output_data)?;

            }
            Self::UpdateKittyPrice => {
                check_tdkitty_price_update(input_data,output_data)?;
            }
            Self::UpdateKittyName => {
                let mut input_basic_kitty_data: Vec<DynamicallyTypedData> = Vec::new();
                let mut output_basic_kitty_data: Vec<DynamicallyTypedData> = Vec::new();
                let _ =extract_basic_kitty_List(&input_data,&mut input_basic_kitty_data)?;
                let _ =extract_basic_kitty_List(&output_data,&mut output_basic_kitty_data)?;
                kitties::can_kitty_name_be_updated(&input_basic_kitty_data,&output_basic_kitty_data)?;
                //check_tdkitty_price_update(input_data,output_data,Kitty_update_properties::KittyName)?;
            }
            Self::Buy => {
                check_can_buy::<ID>(input_data,output_data)?;
                return Ok(0);
            }
        }
        Ok(0)
    }
}
