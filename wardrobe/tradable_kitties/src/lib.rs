//! An NFT game inspired by cryptokitties.
//! This is a game which allows for kitties to be bred based on a few factors
//! 1.) Mom and Tired have to be in a state where they are ready to breed
//! 2.) Each Mom and Dad have some DNA and the child will have unique DNA combined from the both of them
//!     Linkable back to the Mom and Dad
//! 3.) The game also allows Kitties to have a cooling off period inbetween breeding before they can be bred again.
//! 4.) A rest operation allows for a Mom Kitty and a Dad Kitty to be cooled off
//!
//! In order to submit a valid transaction you must strutucture it as follows:
//! 1.) Input must contain 1 mom and 1 dad
//! 2.) Output must contain Mom, Dad, and newly created Child
//! 3.) A child's DNA is calculated by:
//!         BlakeTwo256::hash_of(MomDna, DadDna, MomCurrNumBreedings, DadCurrNumberBreedings)
//!
//! There are a only a finite amount of free breedings available before it starts to cost money
//! to breed kitties.

#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, Hash as HashT},
    transaction_validity::TransactionPriority,
};
use sp_std::prelude::*;
use tuxedo_core::{
    dynamic_typing::{DynamicallyTypedData, UtxoData},
    traits::Cash,
    ensure,
    types::Transaction,
    SimpleConstraintChecker, Verifier,
};
use kitties::KittyHelpers;
use kitties::Breed as BasicKittyBreed;
use money::{Coin, MoneyConstraintChecker};
use kitties::KittyData;
use kitties::ConstraintCheckerError;
use tuxedo_core::types::Output;


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
    pub price: Option<u64>,
    pub is_available_for_sale: bool,
}

impl Default for TradableKittyData {
    fn default() -> Self {
        Self {
            kitty_basic_data: KittyData::default(),
//            kitty_basic_data.kitty_name: *b"tdkt",
            price: Some(100),
            is_available_for_sale: true,
        }
    }
}

impl TryFrom<&DynamicallyTypedData> for TradableKittyData {
    type Error = TradableKittyConstraintCheckerError;
    fn try_from(a: &DynamicallyTypedData) -> Result<Self, Self::Error> {
        a.extract::<TradableKittyData>()
            .map_err(|_| TradableKittyConstraintCheckerError::BadlyTyped)
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
pub enum TradableKittyConstraintCheckerError {
    /// Dynamic typing issue.
    /// This error doesn't discriminate between badly typed inputs and outputs.
    BadlyTyped,
    /// Needed when spending for breeding.
    MinimumSpendAndBreedNotMet,
    /// Need two parents to breed.
    TwoParentsDoNotExist,
    /// Incorrect number of outputs when it comes to breeding.
    NotEnoughFamilyMembers,
    /// Incorrect number of outputs when it comes to Minting.
    IncorrectNumberOfKittiesForMintOperation,
    /// Mom has recently given birth and isnt ready to breed.
    MomNotReadyYet,
    /// Dad cannot breed because he is still too tired.
    DadTooTired,
    /// Cannot have two moms when breeding.
    TwoMomsNotValid,
    /// Cannot have two dads when breeding.
    TwoDadsNotValid,
    /// New Mom after breeding should be in HadBirthRecently state.
    NewMomIsStillRearinToGo,
    /// New Dad after breeding should be in Tired state.
    NewDadIsStillRearinToGo,
    /// Number of free breedings of new parent is not correct.
    NewParentFreeBreedingsIncorrect,
    /// New parents DNA does not match the old one parent has to still be the same kitty.
    NewParentDnaDoesntMatchOld,
    /// New parent Breedings has not incremented or is incorrect.
    NewParentNumberBreedingsIncorrect,
    /// New child DNA is not correct given the protocol.
    NewChildDnaIncorrect,
    /// New child doesnt have the correct number of free breedings.
    NewChildFreeBreedingsIncorrect,
    /// New child has non zero breedings which is impossible because it was just born.
    NewChildHasNonZeroBreedings,
    /// New child parent info is either in Tired state or HadBirthRecently state which is not possible.
    NewChildIncorrectParentInfo,
    /// Too many breedings for this kitty can no longer breed.
    TooManyBreedingsForKitty,
    /// Not enough free breedings available for these parents.
    NotEnoughFreeBreedings,
    /// The transaction attempts to mint no Kitty . This is not allowed.
    MintingNothing,
    /// No Need of parents to mint.
    MintingWithInputs,
    /// Kitty Update has more than one outputs.
    MultipleOutputsForKittyUpdateError,
    /// Kitty Update has more than one outputs.
    InValidNumberOfInputsForKittyUpdate,
    /// Basic kitty properties cannot be updated.
    BasicKittyPropertiesCannotBeUpdated,
    /// Kitty FreeBreeding cannot be updated.
    FreeBreedingCannotBeUpdated,
    /// Kitty NumOfBreeding cannot be updated.
    NumOfBreedingCannotBeUpdated,
    /// Kitty updated price is incorrect.
    UpdatedKittyIncorrectPrice,
    /// No input for kitty Update.
    InputMissingUpdatingNothing,
    /// No output for kitty Update.
    OutputMissingUpadtingNothing,

    /// No input for kitty Update.
    InputMissingBuyingNothing,
    /// No output for kitty Update.
    OutputMissingBuyingNothing,
    /// Incorrect number of outputs when it comes to Minting.
    IncorrectNumberOfInputKittiesForBuyOperation,
    /// Incorrect number of outputs when it comes to Minting.
    IncorrectNumberOfOutputKittiesForBuyOperation,
    /// Kitty not avilable for sale
    KittyNotAvilableForSale,
    /// Not enough amount to buy kitty 
    InsufficientCollateralToBuyKitty,
    
    // From below money constraintchecker errors are added 

    /// The transaction attempts to spend without consuming any inputs.
    /// Either the output value will exceed the input value, or if there are no outputs,
    /// it is a waste of processing power, so it is not allowed.
    SpendingNothing,
    /// The value of the spent input coins is less than the value of the newly created
    /// output coins. This would lead to money creation and is not allowed.
    OutputsExceedInputs,
    /// The value consumed or created by this transaction overflows the value type.
    /// This could lead to problems like https://bitcointalk.org/index.php?topic=823.0
    ValueOverflow,
    /// The transaction attempted to create a coin with zero value. This is not allowed
    /// because it wastes state space.
    ZeroValueCoin,
}

impl From<money::ConstraintCheckerError> for TradableKittyConstraintCheckerError {
    fn from(error:money::ConstraintCheckerError) -> Self {
        match error {
            money::ConstraintCheckerError::BadlyTyped => TradableKittyConstraintCheckerError::BadlyTyped,
            money::ConstraintCheckerError::MintingWithInputs => TradableKittyConstraintCheckerError::MintingWithInputs,
            money::ConstraintCheckerError::MintingNothing => TradableKittyConstraintCheckerError::MintingNothing,
            money::ConstraintCheckerError::SpendingNothing => TradableKittyConstraintCheckerError::SpendingNothing,
            money::ConstraintCheckerError::OutputsExceedInputs => TradableKittyConstraintCheckerError::OutputsExceedInputs,
            money::ConstraintCheckerError::ValueOverflow => TradableKittyConstraintCheckerError::ValueOverflow,
            money::ConstraintCheckerError::ZeroValueCoin => TradableKittyConstraintCheckerError::ZeroValueCoin,
        }
    }
}

// Implement From trait for mapping ConstraintCheckerError to TradableKittyConstraintCheckerError
impl From<ConstraintCheckerError> for TradableKittyConstraintCheckerError {
    fn from(error: ConstraintCheckerError) -> Self {
        match error {
            ConstraintCheckerError::BadlyTyped => TradableKittyConstraintCheckerError::BadlyTyped,
            ConstraintCheckerError::MinimumSpendAndBreedNotMet => TradableKittyConstraintCheckerError::MinimumSpendAndBreedNotMet,
            ConstraintCheckerError::TwoParentsDoNotExist => TradableKittyConstraintCheckerError::TwoParentsDoNotExist,
            ConstraintCheckerError::NotEnoughFamilyMembers => TradableKittyConstraintCheckerError::NotEnoughFamilyMembers,
            ConstraintCheckerError::IncorrectNumberOfKittiesForMintOperation => TradableKittyConstraintCheckerError::IncorrectNumberOfKittiesForMintOperation,
            ConstraintCheckerError::MomNotReadyYet => TradableKittyConstraintCheckerError::MomNotReadyYet,
            ConstraintCheckerError::DadTooTired => TradableKittyConstraintCheckerError::DadTooTired,
            ConstraintCheckerError::TwoMomsNotValid => TradableKittyConstraintCheckerError::TwoMomsNotValid,
            ConstraintCheckerError::TwoDadsNotValid => TradableKittyConstraintCheckerError::TwoDadsNotValid,
            ConstraintCheckerError::NewMomIsStillRearinToGo => TradableKittyConstraintCheckerError::NewMomIsStillRearinToGo,
            ConstraintCheckerError::NewDadIsStillRearinToGo => TradableKittyConstraintCheckerError::NewDadIsStillRearinToGo,
            ConstraintCheckerError::NewParentFreeBreedingsIncorrect => TradableKittyConstraintCheckerError::NewParentFreeBreedingsIncorrect,
            ConstraintCheckerError::NewParentDnaDoesntMatchOld => TradableKittyConstraintCheckerError::NewParentDnaDoesntMatchOld,
            ConstraintCheckerError::NewParentNumberBreedingsIncorrect => TradableKittyConstraintCheckerError::NewParentNumberBreedingsIncorrect,
            ConstraintCheckerError::NewChildDnaIncorrect => TradableKittyConstraintCheckerError::NewChildDnaIncorrect,
            ConstraintCheckerError::NewChildFreeBreedingsIncorrect => TradableKittyConstraintCheckerError::NewChildFreeBreedingsIncorrect,
            ConstraintCheckerError::NewChildHasNonZeroBreedings => TradableKittyConstraintCheckerError::NewChildHasNonZeroBreedings,
            ConstraintCheckerError::NewChildIncorrectParentInfo => TradableKittyConstraintCheckerError::NewChildIncorrectParentInfo,
            ConstraintCheckerError::TooManyBreedingsForKitty => TradableKittyConstraintCheckerError::TooManyBreedingsForKitty,
            ConstraintCheckerError::NotEnoughFreeBreedings => TradableKittyConstraintCheckerError::NotEnoughFreeBreedings,
            ConstraintCheckerError::MintingNothing => TradableKittyConstraintCheckerError::MintingNothing,
            ConstraintCheckerError::MintingWithInputs => TradableKittyConstraintCheckerError::MintingWithInputs,
        }
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
    /// A mint transaction that creates kitties from one parent(either mom or dad).
    Mint,
    /// A typical Breed transaction where kitties are consumed and new family(Parents(mom,dad) and child) is created.
    Breed,
    ///Update various properties of kitty.
    UpdateProperties,
    ///Can buy a new kitty from others
    Buy,
}

trait Buy {
    /*
    /// The Cost to buy a kitty if it is not free.
    const COST: u128;
    */
    /// Error type for all Kitty errors.
    type Error: Into<TradableKittyConstraintCheckerError>;

    fn can_buy(input_data: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData]) -> Result<(), Self::Error>;

    fn check_can_kitty_be_traded(input_data: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData]) -> Result<(), Self::Error>;
}

pub trait Breed {
    type Error: Into<TradableKittyConstraintCheckerError>;
    fn can_breed(mom: &TradableKittyData, dad: &TradableKittyData) -> Result<(), Self::Error>;
    fn check_new_family(mom: &TradableKittyData, dad: &TradableKittyData,
        newFamily: &[DynamicallyTypedData])-> Result<(), Self::Error>;
}

trait UpdateKittyProperty {
    /// The Cost to update a kitty property if it is not free.
    const COST: u128;
    /// Error type for all Kitty errors.
    type Error: Into<TradableKittyConstraintCheckerError>;
    
    fn check_updated_kitty(
        original_kitty: &TradableKittyData,
        updated_kitty: &TradableKittyData,
    ) -> Result<(), Self::Error>;

}

pub struct TradableKittyHelpers<const ID: u8> ;

impl<const ID: u8>  UpdateKittyProperty for TradableKittyHelpers<ID> {

    const COST: u128 = 5u128;
    type Error = TradableKittyConstraintCheckerError;

    fn check_updated_kitty(
        original_kitty: &TradableKittyData,
        updated_kitty: &TradableKittyData,
    ) -> Result<(), Self::Error> {
        ensure!(
            original_kitty.kitty_basic_data.parent == 
                updated_kitty.kitty_basic_data.parent,
            Self::Error::BasicKittyPropertiesCannotBeUpdated,
        );
        ensure!(
            original_kitty.kitty_basic_data.free_breedings == 
                updated_kitty.kitty_basic_data.free_breedings,
            Self::Error::BasicKittyPropertiesCannotBeUpdated,
        );
        ensure!(
            original_kitty.kitty_basic_data.dna == 
                updated_kitty.kitty_basic_data.dna,
            Self::Error::BasicKittyPropertiesCannotBeUpdated,
        );
        ensure!(
            original_kitty.kitty_basic_data.num_breedings == 
                updated_kitty.kitty_basic_data.num_breedings,
            Self::Error::BasicKittyPropertiesCannotBeUpdated,
        );
        // Name can be updated.
        if !updated_kitty.is_available_for_sale &&
                    updated_kitty.price != None {
            return Err(Self::Error::UpdatedKittyIncorrectPrice);
        }

        if updated_kitty.is_available_for_sale &&
                    (updated_kitty.price == None ||
                     updated_kitty.price.unwrap() == 0) {
            return Err(Self::Error::UpdatedKittyIncorrectPrice);
        }
        //Todo some more check are required.
        Ok(())
    }
}

impl<const ID: u8>  Breed for TradableKittyHelpers<ID> {
    type Error = TradableKittyConstraintCheckerError;
    fn can_breed(mom: &TradableKittyData, dad: &TradableKittyData) -> Result<(), Self::Error> {
        log::info!("TradableKitty can_breed");
        KittyHelpers::can_breed(&mom.kitty_basic_data,
        &dad.kitty_basic_data)?;
        Ok(())
    }

    fn check_new_family(mom: &TradableKittyData, dad: &TradableKittyData,
        new_tradable_kitty_family: &[DynamicallyTypedData])-> Result<(), Self::Error> {
        log::info!("TradableKitty check_new_family");
        let new_tradable_kitty_mom = TradableKittyData::try_from(&new_tradable_kitty_family[0])?;
        let new_tradable_kitty_dad = TradableKittyData::try_from(&new_tradable_kitty_family[1])?;
        let new_tradable_kitty_child = TradableKittyData::try_from(&new_tradable_kitty_family[2])?;

        let new_kitty_mom:DynamicallyTypedData = new_tradable_kitty_mom.kitty_basic_data.into();
        let new_kitty_dad:DynamicallyTypedData = new_tradable_kitty_dad.kitty_basic_data.into();
        let new_kitty_child:DynamicallyTypedData = new_tradable_kitty_child.kitty_basic_data.into();

        let mut new_family: Vec<DynamicallyTypedData> = Vec::new();
        new_family.push(new_kitty_mom);
        new_family.push(new_kitty_dad);
        new_family.push(new_kitty_child);


        KittyHelpers::check_new_family(
            &mom.kitty_basic_data, 
            &dad.kitty_basic_data, 
            &new_family)?;
            Ok(())
    }
}

impl<const ID: u8>  Buy for TradableKittyHelpers<ID> {

 //   const COST: u128 = 5u128;
    type Error = TradableKittyConstraintCheckerError;
    fn can_buy(
        input_data: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData]) -> Result<(), Self::Error> {

        
        let mut input_coin_data: Vec<DynamicallyTypedData> = Vec::new();
        let mut input_kitty_data: Vec<DynamicallyTypedData> = Vec::new();
        let mut output_coin_data: Vec<DynamicallyTypedData> = Vec::new();
        let mut output_kitty_data: Vec<DynamicallyTypedData> = Vec::new();

        let mut total_input_amount = 0;
        let mut total_price_of_kitty = 0;
        
        for utxo in input_data {
            if let Ok(coin) = utxo.extract::<Coin<ID>>() {
                log::info!("TradableKittyConstraintChecker found coin in i/p {:?}",coin);
                let utxo_value = coin.0;
                ensure!(utxo_value > 0, TradableKittyConstraintCheckerError::ZeroValueCoin);
                input_coin_data.push(utxo.clone());
                total_input_amount+=utxo_value
                // Process Coin
            } else if let Ok(tradable_kitty) = utxo.extract::<TradableKittyData>() {
                log::info!("TradableKittyConstraintChecker found kitty in i/p {:?}",tradable_kitty);
                if !tradable_kitty.is_available_for_sale {
                    log::error!("Kitty notavliable for sale");
                    return Err(Self::Error::KittyNotAvilableForSale);
                }
                input_kitty_data.push(utxo.clone());
                total_price_of_kitty+= tradable_kitty.price.unwrap();
                // Process TradableKittyData
                // You can also use the `tradable_kitty` variable here
            } else {
                log::error!("TradableKittyConstraintChecker found something else i/p {:?}",utxo);
                return Err(Self::Error::BadlyTyped);
            }
        }

        // Need to filter only Coins and send to MoneyConstraintChecker
        for utxo in output_data {
            if let Ok(coin) = utxo.extract::<Coin<ID>>() {
                log::info!("TradableKittyConstraintChecker found coin in o/p {:?}",coin);
                let utxo_value = coin.0;
                ensure!(utxo_value > 0, TradableKittyConstraintCheckerError::ZeroValueCoin);
                output_coin_data.push(utxo.clone());
                // Process Coin
            } else if let Ok(tradable_kitty) = utxo.extract::<TradableKittyData>() {
                log::info!("TradableKittyConstraintChecker found kitty in o/p {:?}",tradable_kitty);
                output_kitty_data.push(utxo.clone());
                // Process TradableKittyData
                // You can also use the `tradable_kitty` variable here
            } else {
                log::error!("TradableKittyConstraintChecker found something else in o/p  {:?}",utxo);
                return Err(Self::Error::BadlyTyped);
            }
        }

        // As of now buying only a single kitty is supported.
        // There is no way to check the how much amount is sent which seller.
        ensure!(input_kitty_data.len() == 1, Self::Error::IncorrectNumberOfInputKittiesForBuyOperation);
        ensure!(output_kitty_data.len() == 1, Self::Error::IncorrectNumberOfOutputKittiesForBuyOperation);
        log::info!("InsufficientCollateralToBuyKitty  total_price_of_kitty = {:?}total_input_amount = {:?}",
        total_price_of_kitty,total_input_amount);
        ensure!(
            u128::from(total_price_of_kitty) <= total_input_amount,
            TradableKittyConstraintCheckerError::InsufficientCollateralToBuyKitty
        );

        // Need to filter only Coins and send to MoneyConstraintChecker
        MoneyConstraintChecker::<0>::Spend.check(&input_coin_data, &[], &output_coin_data)?;
        log::info!("MoneyConstraintChecker passed ");

        Self::check_can_kitty_be_traded(&input_kitty_data,&output_kitty_data)?;

        
        Ok(())
    }

    fn check_can_kitty_be_traded(input_data: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData]) -> Result<(), Self::Error> {
            
            log::info!("check_can_kitty_be_traded() called ");
            ensure!(
                !input_data.is_empty(),
                TradableKittyConstraintCheckerError::InputMissingBuyingNothing
            );

            // Make sure there is at least one output being minted
            ensure!(
                !output_data.is_empty(),
                TradableKittyConstraintCheckerError::OutputMissingBuyingNothing
            );

            // Make sure the outputs are the right type
            for utxo in output_data {
                let utxo_kitty = utxo
                    .extract::<TradableKittyData>()
                    .map_err(|_| TradableKittyConstraintCheckerError::BadlyTyped)?;
            }

            log::info!("Buy kitty checks completed");

        Ok(())
    }
}

impl<const ID: u8>  SimpleConstraintChecker for TradableKittyConstraintChecker<ID>  {
    type Error = TradableKittyConstraintCheckerError;
    /// Checks:
    ///     - `input_data` is of length 2
    ///     - `output_data` is of length 3
    ///
    fn check(
        &self,
        input_data: &[DynamicallyTypedData],
        _peeks: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData],
    ) -> Result<TransactionPriority, Self::Error> {
        log::info!("TradableKittyConstraintChecker called ");
        match &self {
            Self::Mint => {
                // Make sure there are no inputs being consumed
                log::info!("Self::Mint()  called ");
                ensure!(
                    input_data.is_empty(),
                    TradableKittyConstraintCheckerError::MintingWithInputs
                );

                // Make sure there is at least one output being minted
                ensure!(
                    !output_data.is_empty(),
                    TradableKittyConstraintCheckerError::MintingNothing
                );

                // Make sure the outputs are the right type
                for utxo in output_data {
                    let utxo_kitty = utxo
                        .extract::<TradableKittyData>()
                        .map_err(|_| TradableKittyConstraintCheckerError::BadlyTyped)?;
                }
                log::info!("Mint kitty completed");
                return Ok(0);
            }
            Self::Breed => {
                log::info!("Breed called");
                ensure!(input_data.len() == 2, Self::Error::TwoParentsDoNotExist);
                let mom = TradableKittyData::try_from(&input_data[0])?;
                let dad = TradableKittyData::try_from(&input_data[1])?;
                TradableKittyHelpers::<ID>::can_breed(&mom,&dad)?;
                ensure!(output_data.len() == 3, Self::Error::NotEnoughFamilyMembers);
                TradableKittyHelpers::<ID>::check_new_family(
                    &mom, 
                    &dad, 
                    output_data)?;
                return Ok(0);
            }
            Self::UpdateProperties => {
                log::info!("UpdateProperties called");
                ensure!(
                    !input_data.is_empty(),
                    TradableKittyConstraintCheckerError::InputMissingUpdatingNothing
                );

                // Make sure there is at least one output being minted
                ensure!(
                    !output_data.is_empty(),
                    TradableKittyConstraintCheckerError::OutputMissingUpadtingNothing
                );
                let original_kitty = TradableKittyData::try_from(&input_data[0])?;
                let updated_kitty = TradableKittyData::try_from(&output_data[0])?;
                // Needs more check for updating the kitties
                TradableKittyHelpers::<ID>::check_updated_kitty(&original_kitty,&updated_kitty)?;
            }
            Self::Buy => {
                TradableKittyHelpers::<ID>::can_buy(input_data,output_data)?;
                return Ok(0);
            }
        }
        Ok(0)
    }
}
