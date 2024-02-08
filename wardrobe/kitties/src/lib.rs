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
    ensure,
    types::Transaction,
    SimpleConstraintChecker, Verifier,
};
use money::Coin;

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
pub enum FreeKittyConstraintChecker {
    /// A typical Breed transaction where kitties are consumed and new family(Parents(mom,dad) and child) is created.
    Breed,
    /// A mint transaction that creates kitties from one parent(either mom or dad).
    Mint,
	/*
    ///Update various properties of kitty.
     UpdateProperties,
    ///Can buy a new kitty from others
    Buy,
	*/
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
pub enum  PaidKittyConstraintChecker<const ID: u8> {
    Buy,
    Breed,
}
*/

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
pub enum DadKittyStatus {
    #[default]
    RearinToGo,
    Tired,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
pub enum MomKittyStatus {
    #[default]
    RearinToGo,
    HadBirthRecently,
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
pub enum Parent {
    Mom(MomKittyStatus),
    Dad(DadKittyStatus),
}

impl Parent {
    pub fn dad() -> Self {
        Parent::Dad(DadKittyStatus::RearinToGo)
    }

    pub fn mom() -> Self {
        Parent::Mom(MomKittyStatus::RearinToGo)
    }
}

impl Default for Parent {
    fn default() -> Self {
        Parent::Mom(MomKittyStatus::RearinToGo)
    }
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Clone,
    Encode,
    Decode,
    Hash,
    Debug,
    TypeInfo,
)]
pub struct KittyDNA(pub H256);

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
pub struct KittyData {
    pub parent: Parent,
    pub free_breedings: u64, // Ignore in breed for money case
    pub dna: KittyDNA,
    pub num_breedings: u128,
    pub name: [u8; 4], // Name of kitty is stored in onChain s of now.
   // pub price: Option<u64>,
   // pub is_available_for_sale: bool,
}

impl KittyData {
    /// Create a mint transaction for a single Kitty.
    pub fn mint<V, OV, OC>(
        parent: Parent,
        dna_preimage: &[u8],
        kitty_name: [u8; 4],
        v: V,
    ) -> Transaction<OV, OC>
    where
        V: Verifier,
        OV: Verifier + From<V>,
        OC: tuxedo_core::ConstraintChecker<OV> + core::convert::From<FreeKittyConstraintChecker>,
    {
        Transaction {
            inputs: vec![],
            peeks: vec![],
            outputs: vec![(
                KittyData {
                    parent,
                    dna: KittyDNA(BlakeTwo256::hash(dna_preimage)),
                    name: kitty_name,
                    ..Default::default()
                },
                v,
            )
                .into()],
            //checker: FreeKittyConstraintChecker.into(),
            checker: FreeKittyConstraintChecker::Mint.into(),
        }
    }
}

impl Default for KittyData {
    fn default() -> Self {
        Self {
            parent: Parent::Mom(MomKittyStatus::RearinToGo),
            free_breedings: 2,
            dna: KittyDNA(H256::from_slice(b"mom_kitty_1asdfasdfasdfasdfasdfa")),
            num_breedings: 3,
            name: *b"kty0",
        //    price: Some(100),
        //    is_available_for_sale: true,
        }
    }
}

impl UtxoData for KittyData {
    const TYPE_ID: [u8; 4] = *b"Kitt";
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
pub enum ConstraintCheckerError {
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
	/*
    /// The transaction attempts to updat no Kitty . This is not allowed.
    InputMissingUpadtingNothing,
    /// The transaction attempts to updat no Kitty . This is not allowed.
    OutputMissingUpadtingNothing,

    
    /// Kitty Update has more than one outputs.
    MultipleOutputsForKittyUpdateError,
    /// Kitty Update has more than one outputs.
    InValidNumberOfInputsForKittyUpdate,
    /// Kitty Dna cannot be updated.
    DnaCannotBeUpdated,
    /// Kitty FreeBreeding cannot be updated.
    FreeBreedingCannotBeUpdated,
    /// Kitty NumOfBreeding cannot be updated.
    NumOfBreedingCannotBeUpdated,
    /// Kitty updated price is incorrect.
    UpdatedKittyIncorrectPrice,
	*/
}

pub trait Breed {
    /// The Cost to breed a kitty if it is not free.
    const COST: u128;
    /// Number of free breedings a kitty will have.
    const NUM_FREE_BREEDINGS: u64;
    /// Error type for all Kitty errors.
    type Error: Into<ConstraintCheckerError>;
    /// Check if the two parents (Mom, Dad) proposed are capable of breeding.
    fn can_breed(mom: &KittyData, dad: &KittyData) -> Result<(), Self::Error>;
    /// Checks if mom is in the correct state and capable of breeding.
    fn check_mom_can_breed(mom: &KittyData) -> Result<(), Self::Error>;
    /// Checks if dad is in the correct state and capable of breeding.
    fn check_dad_can_breed(dad: &KittyData) -> Result<(), Self::Error>;
    /// Makes sure each parent has a non-zero number of free breedings.
    fn check_free_breedings(mom: &KittyData, dad: &KittyData) -> Result<(), Self::Error>;
    /// Checks outputs which consists of (Mom, Dad, Child) is correctly formulated.
    fn check_new_family(
        old_mom: &KittyData,
        old_dad: &KittyData,
        new_family: &[DynamicallyTypedData],
    ) -> Result<(), Self::Error>;
    /// Checks if new mom matches the old ones DNA and changes state correctly.
    fn check_new_mom(old_mom: &KittyData, new_mom: &KittyData) -> Result<(), Self::Error>;
    /// Checks if new dad matches the old ones DNA and changes state correctly.
    fn check_new_dad(old_dad: &KittyData, new_dad: &KittyData) -> Result<(), Self::Error>;
    /// Checks if new child DNA is formulated correctly and is initialized to the proper state.
    fn check_child(
        new_mom: &KittyData,
        new_dad: &KittyData,
        child: &KittyData,
    ) -> Result<(), Self::Error>;
}


trait UpdateKittyProperty {
    /// The Cost to update a kitty property if it is not free.
    const COST: u128;
    /// Error type for all Kitty errors.
    type Error: Into<ConstraintCheckerError>;
    
    fn check_updated_kitty(
        original_kitty: &KittyData,
        updated_kitty: &KittyData,
    ) -> Result<(), Self::Error>;

}

pub struct KittyHelpers;
impl Breed for KittyHelpers {
    const COST: u128 = 5u128;
    const NUM_FREE_BREEDINGS: u64 = 2u64;
    type Error = ConstraintCheckerError;
    /// Checks:
    ///     - Mom can breed
    ///     - Dad can breed
    ///
    fn can_breed(mom: &KittyData, dad: &KittyData) -> Result<(), Self::Error> {
        Self::check_mom_can_breed(mom)?;
        Self::check_dad_can_breed(dad)?;
        Self::check_free_breedings(mom, dad)?;
        Ok(())
    }

    /// Checks:
    ///     - Mom is in `RearinToGo` state
    ///     - Mom number of breedings is not maxed out
    ///
    fn check_mom_can_breed(mom: &KittyData) -> Result<(), Self::Error> {
        match &mom.parent {
            Parent::Mom(status) => {
                if let MomKittyStatus::HadBirthRecently = status {
                    return Err(Self::Error::MomNotReadyYet);
                }
            }
            Parent::Dad(_) => return Err(Self::Error::TwoDadsNotValid),
        }
        mom.num_breedings
            .checked_add(1)
            .ok_or(Self::Error::TooManyBreedingsForKitty)?;
        Ok(())
    }

    /// Checks:
    ///     - Dad is in `RearinToGo` state
    ///     - Dad number of breedings is not maxed out
    ///
    fn check_dad_can_breed(dad: &KittyData) -> Result<(), Self::Error> {
        match &dad.parent {
            Parent::Dad(status) => {
                if let DadKittyStatus::Tired = status {
                    return Err(Self::Error::DadTooTired);
                }
            }
            Parent::Mom(_) => return Err(Self::Error::TwoMomsNotValid),
        }
        dad.num_breedings
            .checked_add(1)
            .ok_or(Self::Error::TooManyBreedingsForKitty)?;
        Ok(())
    }

    /// Checks:
    ///     - Both parents free breedings is non-zero
    ///
    fn check_free_breedings(mom: &KittyData, dad: &KittyData) -> Result<(), Self::Error> {
        let mom_breedings = mom.free_breedings;
        let dad_breedings = dad.free_breedings;
        if (mom_breedings == 0) || (dad_breedings == 0) {
            return Err(Self::Error::NotEnoughFreeBreedings);
        }
        Ok(())
    }

    fn check_new_family(
        old_mom: &KittyData,
        old_dad: &KittyData,
        new_family: &[DynamicallyTypedData],
    ) -> Result<(), Self::Error> {
        log::info!("Kitty check_new_family");
        // Output Side
        ensure!(new_family.len() == 3, Self::Error::NotEnoughFamilyMembers);
        let new_mom = KittyData::try_from(&new_family[0])?;
        let new_dad = KittyData::try_from(&new_family[1])?;
        let child = KittyData::try_from(&new_family[2])?;
        Self::check_new_mom(old_mom, &new_mom)?;
        Self::check_new_dad(old_dad, &new_dad)?;
        Self::check_child(&new_mom, &new_dad, &child)?;
        Ok(())
    }

    /// Checks:
    ///     - Mom is now in `HadBirthRecently`
    ///     - Mom has 1 less `free_breedings`
    ///     - Mom's DNA matches old Mom
    ///     - Mom's num breedings is incremented
    ///
    fn check_new_mom(old_mom: &KittyData, new_mom: &KittyData) -> Result<(), Self::Error> {
        match &new_mom.parent {
            Parent::Mom(status) => {
                if let MomKittyStatus::RearinToGo = status {
                    return Err(Self::Error::NewMomIsStillRearinToGo);
                }
            }
            Parent::Dad(_) => return Err(Self::Error::TwoDadsNotValid),
        }

        ensure!(
            new_mom.free_breedings == old_mom.free_breedings - 1,
            Self::Error::NewParentFreeBreedingsIncorrect
        );
        ensure!(
            new_mom.num_breedings == old_mom.num_breedings + 1,
            Self::Error::NewParentNumberBreedingsIncorrect
        );
        ensure!(
            new_mom.dna == old_mom.dna,
            Self::Error::NewParentDnaDoesntMatchOld
        );

        Ok(())
    }

    /// Checks:
    ///     - Dad is now `Tired`
    ///     - Dad has 1 less `free_breedings`
    ///     - Dad's DNA matches old Dad
    ///     - Dad's num breedings is incremented
    ///
    fn check_new_dad(old_dad: &KittyData, new_dad: &KittyData) -> Result<(), Self::Error> {
        match &new_dad.parent {
            Parent::Dad(status) => {
                if let DadKittyStatus::RearinToGo = status {
                    return Err(Self::Error::NewDadIsStillRearinToGo);
                }
            }
            Parent::Mom(_) => return Err(Self::Error::TwoMomsNotValid),
        }

        ensure!(
            new_dad.free_breedings == old_dad.free_breedings - 1,
            Self::Error::NewParentFreeBreedingsIncorrect
        );
        ensure!(
            new_dad.num_breedings == old_dad.num_breedings + 1,
            Self::Error::NewParentNumberBreedingsIncorrect
        );
        ensure!(
            new_dad.dna == old_dad.dna,
            Self::Error::NewParentDnaDoesntMatchOld
        );

        Ok(())
    }

    /// Checks:
    ///     - DNA formation correct -> `hash_of(mom_dna + dad_dna + mom_num_breedings + dad_num_breedings)
    ///     - Free breedings is correct given the trait implementation in this case 2
    ///     - has non-zero bredings
    ///     - If Mom is in RearinToGo
    ///     - If Dad is in RearinToGo
    ///
    fn check_child(
        new_mom: &KittyData,
        new_dad: &KittyData,
        child: &KittyData,
    ) -> Result<(), Self::Error> {
        let new_dna = BlakeTwo256::hash_of(&(
            &new_mom.dna,
            &new_dad.dna,
            &new_mom.num_breedings,
            &new_dad.num_breedings,
        ));

        log::info!("new_mom.dna {:?} ", new_mom.dna);
        log::info!("new_dad.dna {:?} ", new_dad.dna);
        log::info!("Passed child.dna {:?} ", child.dna);
        log::info!("calculated  child.dna {:?} ", KittyDNA(new_dna));
        ensure!(
            child.dna == KittyDNA(new_dna),
            Self::Error::NewChildDnaIncorrect,
        );
        ensure!(
            child.free_breedings == Self::NUM_FREE_BREEDINGS,
            Self::Error::NewChildFreeBreedingsIncorrect
        );
        ensure!(
            child.num_breedings == 0,
            Self::Error::NewChildHasNonZeroBreedings,
        );

        match &child.parent {
            Parent::Mom(status) => {
                if let MomKittyStatus::HadBirthRecently = status {
                    return Err(Self::Error::NewChildIncorrectParentInfo);
                }
            }
            Parent::Dad(status) => {
                if let DadKittyStatus::Tired = status {
                    return Err(Self::Error::NewChildIncorrectParentInfo);
                }
            }
        }
        Ok(())
    }
}
/*
impl UpdateKittyProperty for KittyHelpers {

    const COST: u128 = 5u128;
    type Error = ConstraintCheckerError;

    fn check_updated_kitty(
        original_kitty: &KittyData,
        updated_kitty: &KittyData,
    ) -> Result<(), Self::Error> {
        ensure!(
            original_kitty.dna == updated_kitty.dna,
            Self::Error::DnaCannotBeUpdated,
        );
        ensure!(
            original_kitty.free_breedings == updated_kitty.free_breedings,
            Self::Error::FreeBreedingCannotBeUpdated,
        );
        ensure!(
            original_kitty.num_breedings == updated_kitty.num_breedings,
            Self::Error::NumOfBreedingCannotBeUpdated,
        );
        if !updated_kitty.is_available_for_sale &&
                    updated_kitty.price != None {
            return Err(Self::Error::UpdatedKittyIncorrectPrice);
        }

        if updated_kitty.is_available_for_sale &&
                    updated_kitty.price == None {
            return Err(Self::Error::UpdatedKittyIncorrectPrice);
        }
        Ok(())
    }
}
*/

impl TryFrom<&DynamicallyTypedData> for KittyData {
    type Error = ConstraintCheckerError;
    fn try_from(a: &DynamicallyTypedData) -> Result<Self, Self::Error> {
        a.extract::<KittyData>()
            .map_err(|_| ConstraintCheckerError::BadlyTyped)
    }
}

impl SimpleConstraintChecker for FreeKittyConstraintChecker {
    type Error = ConstraintCheckerError;

    fn check(
        &self,
        input_data: &[DynamicallyTypedData],
        _peeks: &[DynamicallyTypedData],
        output_data: &[DynamicallyTypedData],
    ) -> Result<TransactionPriority, Self::Error> {
        log::info!("FreeKittyConstraintChecker check()  called ");
        match &self {
            Self::Mint => {
                // Make sure there are no inputs being consumed
                log::info!("Self::Mint()  called ");
                ensure!(
                    input_data.is_empty(),
                    ConstraintCheckerError::MintingWithInputs
                );

                // Make sure there is at least one output being minted
                ensure!(
                    !output_data.is_empty(),
                    ConstraintCheckerError::MintingNothing
                );

                // Make sure the outputs are the right type
                for utxo in output_data {
                    let utxo_kitty = utxo
                        .extract::<KittyData>()
                        .map_err(|_| ConstraintCheckerError::BadlyTyped)?;
                }
                log::info!("Mint kitty completed");
                Ok(0)
            }
            Self::Breed => {
                // Check that we are consuming at least one input
                log::info!("Breed called");
                ensure!(input_data.len() == 2, Self::Error::TwoParentsDoNotExist);

                let mom = KittyData::try_from(&input_data[0])?;
                let dad = KittyData::try_from(&input_data[1])?;
                KittyHelpers::can_breed(&mom, &dad)?;
                // Output must be Mom, Dad, Child
                ensure!(output_data.len() == 3, Self::Error::NotEnoughFamilyMembers);
                KittyHelpers::check_new_family(&mom, &dad, output_data)?;
                Ok(0)
            }
			/*
            Self::UpdateProperties => {
                log::info!("Update properties is called ");
                ensure!(
                    input_data.is_empty(),
                    //ConstraintCheckerError::InputMissingUpadtingNothing
					ConstraintCheckerError::MintingNothing
                );

                // Make sure there is at least one output being minted
                ensure!(
                    !output_data.is_empty(),
                    //ConstraintCheckerError::OutputMissingUpadtingNothing
					ConstraintCheckerError::MintingNothing
                );
                let original_kitty = KittyData::try_from(&input_data[0])?;
                let updated_kitty = KittyData::try_from(&input_data[1])?;
                // Needs more check for updating the kitties
                KittyHelpers::check_updated_kitty(&original_kitty,&updated_kitty)?;
                Ok(0)
            }
            Self::Buy => {
                // Ned to implement the buy logic
                log::info!("Buy called ");
                Ok(0)
            }
			*/
        }
    }
}

/*
impl<const ID: u8> SimpleConstraintChecker for PaidKittyConstraintChecker<ID> {
    type Error = ConstraintCheckerError;
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
        log::info!("PaidKittyConstraintChecker called ");
        Ok(0)
    }
}
*/
