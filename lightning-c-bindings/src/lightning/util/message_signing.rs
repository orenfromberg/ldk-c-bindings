// This file is Copyright its original authors, visible in version control
// history and in the source files from which this was generated.
//
// This file is licensed under the license available in the LICENSE or LICENSE.md
// file in the root of this repository or, if no such file exists, the same
// license as that which applies to the original source files from which this
// source was automatically generated.

//! Lightning message signing and verification lives here. These tools can be used to sign messages using the node's
//! secret so receivers are sure that they come from you. You can also use this to verify that a given message comes
//! from a specific node.
//! Furthermore, these tools can be used to sign / verify messages using ephemeral keys not tied to node's identities.
//!
//! Note this is not part of the specs, but follows lnd's signing and verifying protocol, which can is defined as follows:
//!
//! signature = zbase32(SigRec(sha256d((\"Lightning Signed Message:\" + msg)))
//! zbase32 from <https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt>
//! SigRec has first byte 31 + recovery id, followed by 64 byte sig.
//!
//! This implementation is compatible with both lnd's and c-lightning's
//!
//! <https://lightning.readthedocs.io/lightning-signmessage.7.html>
//! <https://api.lightning.community/#signmessage>

use std::str::FromStr;
use std::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;

/// Creates a digital signature of a message given a SecretKey, like the node's secret.
/// A receiver knowing the PublicKey (e.g. the node's id) and the message can be sure that the signature was generated by the caller.
/// Signatures are EC recoverable, meaning that given the message and the signature the PublicKey of the signer can be extracted.
#[no_mangle]
pub extern "C" fn sign(mut msg: crate::c_types::u8slice, sk: *const [u8; 32]) -> crate::c_types::derived::CResult_StringErrorZ {
	let mut ret = lightning::util::message_signing::sign(msg.to_slice(), &::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *sk}[..]).unwrap());
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { o.into() }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::c_types::Secp256k1Error::from_rust(e) }).into() };
	local_ret
}

/// Recovers the PublicKey of the signer of the message given the message and the signature.
#[no_mangle]
pub extern "C" fn recover_pk(mut msg: crate::c_types::u8slice, mut sig: crate::c_types::Str) -> crate::c_types::derived::CResult_PublicKeyErrorZ {
	let mut ret = lightning::util::message_signing::recover_pk(msg.to_slice(), sig.into_str());
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::PublicKey::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::c_types::Secp256k1Error::from_rust(e) }).into() };
	local_ret
}

/// Verifies a message was signed by a PrivateKey that derives to a given PublicKey, given a message, a signature,
/// and the PublicKey.
#[no_mangle]
pub extern "C" fn verify(mut msg: crate::c_types::u8slice, mut sig: crate::c_types::Str, mut pk: crate::c_types::PublicKey) -> bool {
	let mut ret = lightning::util::message_signing::verify(msg.to_slice(), sig.into_str(), &pk.into_rust());
	ret
}

