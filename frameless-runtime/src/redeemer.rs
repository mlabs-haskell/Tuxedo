//! A redeemer is a piece of logic that determines whether an input ca nbe consumed in a given context.
//! Because there are multiple reasonable ways to make this decision, we expose a trait to encapsulate
//! the various options. Each runtime will choose to make one or more redeemers available to its users
//! and they will be aggregated into an enum. The most common and useful redeemers are included here
//! with Tuxedo core, but downstream developers are expected to create their own as well.

use sp_std::vec::Vec;
use sp_core::H256;
use sp_application_crypto::sr25519::Signature;

/// A means of checking that an output can be redeemed (aka spent). This check is made on a
/// per-output basis and neither knows nor cares anything about the verification logic that will
/// be applied to the transaction as a whole. Nonetheless, in order to avoid malleability, we
/// we take the entire stripped and serialized transaction as a parameter.
trait Redeemer {
    fn redeem(&self, simplified_tx: Vec<u8>, witness: Vec<u8>) -> bool;
}

/// A typical redeemer that checks an sr25519 signature
pub struct SigCheck{
    owner_pubkey: H256,
}

impl Redeemer for SigCheck {
    fn redeem(&self, simplified_tx: Vec<u8>, witness: Vec<u8>) -> bool {
        // Check that the sig is 64 bytes to begin with.
        if witness.len() != 64 {
            return false;
        }

        let sig = Signature(witness[..64]);
        sp_io::crypto::sr25519_verify(&sig, &simplified_tx, self.owner_pubkey)
    }
}

/// A simple redeemer that allows anyone to consume an output at any time
pub struct UpForGrabs;

impl Redeemer for UpForGrabs {
    fn redeem(&self, _simplified_tx: Vec<u8>, _witness: Vec<u8>) -> bool {
        true
    }
}