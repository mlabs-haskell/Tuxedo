//! An NFT game inspired by cryptokitties.
//! This is a game which allows for kitties to be create,bred and update name of kitty.
//!
//! ## Features
//!
//! - **Create:** Generate a new kitty.
//!   To submit a valid transaction for creating a kitty, adhere to the following structure:
//!   1. Input must be empty.
//!   2. Output must contain only the newly created kittities as a child.
//!
//!    **Note 1:** Multiple kitties can be created at the same time in the same txn..
//!
//! - **Update Name:** Modify the name of a kitty.
//!   To submit a valid transaction for updating a kitty's name, adhere to the following structure:
//!   1. Input must be the kitty to be updated.
//!   2. Output must contain the kitty with the updated name.
//!
//!    **Note 1:** All other properties such as DNA, parents, free breedings, etc., must remain unaltered in the output.
//!    **Note 2:** The input and output kitties must follow same order.
//!
//! - **Breed:** Breeds a new kitty using mom and dad based on below factors
//! 1.) Mom and Dad have to be in a state where they are ready to breed
//! 2.) Each Mom and Dad have some DNA and the child will have unique DNA combined from the both of them
//!     Linkable back to the Mom and Dad
//! 3.) The game also allows Kitties to have a cooling off period inbetween breeding before they can be bred again.
//! 4.) A rest operation allows for a Mom Kitty and a Dad Kitty to be cooled off
//!
//! In order to submit a valid breed transaction you must structure it as follows:
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
use sp_std::collections::btree_set::BTreeSet; // For checking the uniqueness of input and output based on dna.
use sp_std::prelude::*;
use tuxedo_core::{
    dynamic_typing::{DynamicallyTypedData, UtxoData},
    ensure,
    types::Transaction,
    SimpleConstraintChecker, Verifier,
};

#[cfg(test)]
mod tests;

/// The main constraint checker for the kitty piece. Allows below :
/// Create : Allows creation of kitty without parents, Multiple kitties can be created in same txn.
/// UpdateKittyName : Allows updating the name of the kitty s, Multiple kitty name can be updated in same txn.
/// Breed : Allows breeding of kitty.
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
    /// Txn that creates kitty without parents.Multiple kitties can be created  at the same time.
    Create,
    /// Txn that updates kitty Name. Multiple kitty names can be updated. input & output must follow the same order.
    UpdateKittyName,
    /// Txn where kitties are consumed and new family(Parents(mom,dad) and child) is created.
    Breed,
}

/// Dad kitty status with respect to breeding.
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
    /// Can breed.
    RearinToGo,
    /// Can't breed due to tired.
    Tired,
}

/// Mad kitty status with respect to breeding.
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
    /// Can breed.
    RearinToGo,
    /// Can't breed due to recent child kitty delivery.
    HadBirthRecently,
}

/// Parent stuct contains 1 mom kitty and 1 dad kitty.
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

/// Kitty data contains basic informationsuch as below :
/// parent: 1 mom kitty and 1 dad kitty.
/// free_breedings: Free breeding allowed on a kitty.
/// dna :Its a unique per kitty.
/// num_breedings: number of free breedings are remaining.
/// name: Name of kitty.
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
    pub name: [u8; 4],
}

impl KittyData {
    /// Create a mint transaction for a single Kitty.
    pub fn mint<V, OV, OC>(parent: Parent, dna_preimage: &[u8], v: V) -> Transaction<OV, OC>
    where
        V: Verifier,
        OV: Verifier + From<V>,
        OC: tuxedo_core::ConstraintChecker<OV> + From<FreeKittyConstraintChecker>,
    {
        Transaction {
            inputs: vec![],
            peeks: vec![],
            outputs: vec![(
                KittyData {
                    parent,
                    dna: KittyDNA(BlakeTwo256::hash(dna_preimage)),
                    ..Default::default()
                },
                v,
            )
                .into()],
            checker: FreeKittyConstraintChecker::Create.into(),
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
            name: *b"kity",
        }
    }
}

impl UtxoData for KittyData {
    const TYPE_ID: [u8; 4] = *b"Kitt";
}

/// Reasons that kitty opertaion may go wrong.
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
    /// The transaction attempts to create no Kitty.
    CreatingNothing,
    /// Inputs(Parents) not required for mint.
    CreatingWithInputs,
    /// No input for kitty Update.
    InvalidNumberOfInputOutput,
    /// Duplicate kitty found i.e based on the DNA.
    DuplicateKittyFound,
    /// Dna mismatch between input and output.
    DnaMismatchBetweenInputAndOutput,
    /// Name is not updated
    KittyNameUnAltered,
    /// Kitty FreeBreeding cannot be updated.
    FreeBreedingCannotBeUpdated,
    /// Kitty NumOfBreeding cannot be updated.
    NumOfBreedingCannotBeUpdated,
    /// Gender cannot be updated
    KittyGenderCannotBeUpdated,
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
        match &self {
            Self::Create => {
                // Make sure there are no inputs being consumed
                ensure!(
                    input_data.is_empty(),
                    ConstraintCheckerError::CreatingWithInputs
                );

                // Make sure there is at least one output being minted
                ensure!(
                    !output_data.is_empty(),
                    ConstraintCheckerError::CreatingNothing
                );

                // Make sure the outputs are the right type
                for utxo in output_data {
                    let _utxo_kitty = utxo
                        .extract::<KittyData>()
                        .map_err(|_| ConstraintCheckerError::BadlyTyped)?;
                }
                Ok(0)
            }
            Self::Breed => {
                // Check that we are consuming at least one input
                ensure!(input_data.len() == 2, Self::Error::TwoParentsDoNotExist);

                let mom = KittyData::try_from(&input_data[0])?;
                let dad = KittyData::try_from(&input_data[1])?;
                KittyHelpers::can_breed(&mom, &dad)?;
                // Output must be Mom, Dad, Child
                ensure!(output_data.len() == 3, Self::Error::NotEnoughFamilyMembers);
                KittyHelpers::check_new_family(&mom, &dad, output_data)?;
                Ok(0)
            }
            Self::UpdateKittyName => {
                can_kitty_name_be_updated(input_data, output_data)?;
                Ok(0)
            }
        }
    }
}

/// Checks:
///     - Input and output is of kittyType
///     - Only name is updated and ther basic properties are not updated.
///     - Order between input and output must be same.
pub fn can_kitty_name_be_updated(
    input_data: &[DynamicallyTypedData],
    output_data: &[DynamicallyTypedData],
) -> Result<TransactionPriority, ConstraintCheckerError> {
    ensure!(
        input_data.len() == output_data.len() && !input_data.is_empty(),
        { ConstraintCheckerError::InvalidNumberOfInputOutput }
    );
    let mut dna_to_kitty_set: BTreeSet<KittyDNA> = BTreeSet::new();

    for i in 0..input_data.len() {
        let utxo_input_kitty = input_data[i]
            .clone()
            .extract::<KittyData>()
            .map_err(|_| ConstraintCheckerError::BadlyTyped)?;

        if dna_to_kitty_set.contains(&utxo_input_kitty.dna) {
            return Err(ConstraintCheckerError::DuplicateKittyFound);
        } else {
            dna_to_kitty_set.insert(utxo_input_kitty.clone().dna);
        }

        let utxo_output_kitty = output_data[i]
            .clone()
            .extract::<KittyData>()
            .map_err(|_| ConstraintCheckerError::BadlyTyped)?;
        
        if utxo_input_kitty.dna != utxo_output_kitty.dna {
            return Err(ConstraintCheckerError::DnaMismatchBetweenInputAndOutput);
        }
        
        check_kitty_name_update(&utxo_input_kitty, &utxo_output_kitty)?;
    }
    return Ok(0);
}

/// Checks:
///     - Private function used by can_kitty_name_be_updated.
///     - Only name is updated and ther basic properties are not updated.
///
fn check_kitty_name_update(
    original_kitty: &KittyData,
    updated_kitty: &KittyData,
) -> Result<TransactionPriority, ConstraintCheckerError> {
    ensure!(
        original_kitty != updated_kitty,
        ConstraintCheckerError::KittyNameUnAltered
    );
    ensure!(
        original_kitty.free_breedings == updated_kitty.free_breedings,
        ConstraintCheckerError::FreeBreedingCannotBeUpdated
    );
    ensure!(
        original_kitty.num_breedings == updated_kitty.num_breedings,
        ConstraintCheckerError::NumOfBreedingCannotBeUpdated
    );
    ensure!(
        original_kitty.parent == updated_kitty.parent,
        ConstraintCheckerError::KittyGenderCannotBeUpdated
    );
    return Ok(0);
}
