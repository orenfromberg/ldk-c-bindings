// This file is Copyright its original authors, visible in version control
// history and in the source files from which this was generated.
//
// This file is licensed under the license available in the LICENSE or LICENSE.md
// file in the root of this repository or, if no such file exists, the same
// license as that which applies to the original source files from which this
// source was automatically generated.

//! keysinterface provides keys into rust-lightning and defines some useful enums which describe
//! spendable on-chain outputs which the user owns and is responsible for using just as any other
//! on-chain output which is theirs.

use std::str::FromStr;
use std::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;


use lightning::chain::keysinterface::DelayedPaymentOutputDescriptor as nativeDelayedPaymentOutputDescriptorImport;
pub(crate) type nativeDelayedPaymentOutputDescriptor = nativeDelayedPaymentOutputDescriptorImport;

/// Information about a spendable output to a P2WSH script. See
/// SpendableOutputDescriptor::DelayedPaymentOutput for more details on how to spend this.
#[must_use]
#[repr(C)]
pub struct DelayedPaymentOutputDescriptor {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeDelayedPaymentOutputDescriptor,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for DelayedPaymentOutputDescriptor {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeDelayedPaymentOutputDescriptor>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the DelayedPaymentOutputDescriptor, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_free(this_obj: DelayedPaymentOutputDescriptor) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn DelayedPaymentOutputDescriptor_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeDelayedPaymentOutputDescriptor); }
}
#[allow(unused)]
impl DelayedPaymentOutputDescriptor {
	pub(crate) fn get_native_ref(&self) -> &'static nativeDelayedPaymentOutputDescriptor {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeDelayedPaymentOutputDescriptor {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeDelayedPaymentOutputDescriptor {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = std::ptr::null_mut();
		ret
	}
}
/// The outpoint which is spendable
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_outpoint(this_ptr: &DelayedPaymentOutputDescriptor) -> crate::lightning::chain::transaction::OutPoint {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().outpoint;
	crate::lightning::chain::transaction::OutPoint { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::chain::transaction::OutPoint<>) as *mut _) }, is_owned: false }
}
/// The outpoint which is spendable
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_outpoint(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: crate::lightning::chain::transaction::OutPoint) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.outpoint = *unsafe { Box::from_raw(val.take_inner()) };
}
/// Per commitment point to derive delayed_payment_key by key holder
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_per_commitment_point(this_ptr: &DelayedPaymentOutputDescriptor) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().per_commitment_point;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// Per commitment point to derive delayed_payment_key by key holder
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_per_commitment_point(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.per_commitment_point = val.into_rust();
}
/// The nSequence value which must be set in the spending input to satisfy the OP_CSV in
/// the witness_script.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_to_self_delay(this_ptr: &DelayedPaymentOutputDescriptor) -> u16 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().to_self_delay;
	*inner_val
}
/// The nSequence value which must be set in the spending input to satisfy the OP_CSV in
/// the witness_script.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_to_self_delay(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: u16) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.to_self_delay = val;
}
/// The output which is referenced by the given outpoint
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_output(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: crate::c_types::TxOut) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.output = val.into_rust();
}
/// The revocation point specific to the commitment transaction which was broadcast. Used to
/// derive the witnessScript for this output.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_revocation_pubkey(this_ptr: &DelayedPaymentOutputDescriptor) -> crate::c_types::PublicKey {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().revocation_pubkey;
	crate::c_types::PublicKey::from_rust(&inner_val)
}
/// The revocation point specific to the commitment transaction which was broadcast. Used to
/// derive the witnessScript for this output.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_revocation_pubkey(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.revocation_pubkey = val.into_rust();
}
/// Arbitrary identification information returned by a call to
/// `Sign::channel_keys_id()`. This may be useful in re-deriving keys used in
/// the channel to spend the output.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_channel_keys_id(this_ptr: &DelayedPaymentOutputDescriptor) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_keys_id;
	inner_val
}
/// Arbitrary identification information returned by a call to
/// `Sign::channel_keys_id()`. This may be useful in re-deriving keys used in
/// the channel to spend the output.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_channel_keys_id(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_keys_id = val.data;
}
/// The value of the channel which this output originated from, possibly indirectly.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_get_channel_value_satoshis(this_ptr: &DelayedPaymentOutputDescriptor) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_value_satoshis;
	*inner_val
}
/// The value of the channel which this output originated from, possibly indirectly.
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_set_channel_value_satoshis(this_ptr: &mut DelayedPaymentOutputDescriptor, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_value_satoshis = val;
}
/// Constructs a new DelayedPaymentOutputDescriptor given each field
#[must_use]
#[no_mangle]
pub extern "C" fn DelayedPaymentOutputDescriptor_new(mut outpoint_arg: crate::lightning::chain::transaction::OutPoint, mut per_commitment_point_arg: crate::c_types::PublicKey, mut to_self_delay_arg: u16, mut output_arg: crate::c_types::TxOut, mut revocation_pubkey_arg: crate::c_types::PublicKey, mut channel_keys_id_arg: crate::c_types::ThirtyTwoBytes, mut channel_value_satoshis_arg: u64) -> DelayedPaymentOutputDescriptor {
	DelayedPaymentOutputDescriptor { inner: ObjOps::heap_alloc(nativeDelayedPaymentOutputDescriptor {
		outpoint: *unsafe { Box::from_raw(outpoint_arg.take_inner()) },
		per_commitment_point: per_commitment_point_arg.into_rust(),
		to_self_delay: to_self_delay_arg,
		output: output_arg.into_rust(),
		revocation_pubkey: revocation_pubkey_arg.into_rust(),
		channel_keys_id: channel_keys_id_arg.data,
		channel_value_satoshis: channel_value_satoshis_arg,
	}), is_owned: true }
}
impl Clone for DelayedPaymentOutputDescriptor {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeDelayedPaymentOutputDescriptor>::is_null(self.inner) { std::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn DelayedPaymentOutputDescriptor_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeDelayedPaymentOutputDescriptor)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the DelayedPaymentOutputDescriptor
pub extern "C" fn DelayedPaymentOutputDescriptor_clone(orig: &DelayedPaymentOutputDescriptor) -> DelayedPaymentOutputDescriptor {
	orig.clone()
}
#[no_mangle]
/// Serialize the DelayedPaymentOutputDescriptor object into a byte array which can be read by DelayedPaymentOutputDescriptor_read
pub extern "C" fn DelayedPaymentOutputDescriptor_write(obj: &DelayedPaymentOutputDescriptor) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn DelayedPaymentOutputDescriptor_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeDelayedPaymentOutputDescriptor) })
}
#[no_mangle]
/// Read a DelayedPaymentOutputDescriptor from a byte array, created by DelayedPaymentOutputDescriptor_write
pub extern "C" fn DelayedPaymentOutputDescriptor_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_DelayedPaymentOutputDescriptorDecodeErrorZ {
	let res: Result<lightning::chain::keysinterface::DelayedPaymentOutputDescriptor, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_res
}

use lightning::chain::keysinterface::StaticPaymentOutputDescriptor as nativeStaticPaymentOutputDescriptorImport;
pub(crate) type nativeStaticPaymentOutputDescriptor = nativeStaticPaymentOutputDescriptorImport;

/// Information about a spendable output to our \"payment key\". See
/// SpendableOutputDescriptor::StaticPaymentOutput for more details on how to spend this.
#[must_use]
#[repr(C)]
pub struct StaticPaymentOutputDescriptor {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeStaticPaymentOutputDescriptor,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for StaticPaymentOutputDescriptor {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeStaticPaymentOutputDescriptor>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the StaticPaymentOutputDescriptor, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_free(this_obj: StaticPaymentOutputDescriptor) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn StaticPaymentOutputDescriptor_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeStaticPaymentOutputDescriptor); }
}
#[allow(unused)]
impl StaticPaymentOutputDescriptor {
	pub(crate) fn get_native_ref(&self) -> &'static nativeStaticPaymentOutputDescriptor {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeStaticPaymentOutputDescriptor {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeStaticPaymentOutputDescriptor {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = std::ptr::null_mut();
		ret
	}
}
/// The outpoint which is spendable
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_get_outpoint(this_ptr: &StaticPaymentOutputDescriptor) -> crate::lightning::chain::transaction::OutPoint {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().outpoint;
	crate::lightning::chain::transaction::OutPoint { inner: unsafe { ObjOps::nonnull_ptr_to_inner((inner_val as *const lightning::chain::transaction::OutPoint<>) as *mut _) }, is_owned: false }
}
/// The outpoint which is spendable
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_set_outpoint(this_ptr: &mut StaticPaymentOutputDescriptor, mut val: crate::lightning::chain::transaction::OutPoint) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.outpoint = *unsafe { Box::from_raw(val.take_inner()) };
}
/// The output which is referenced by the given outpoint
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_set_output(this_ptr: &mut StaticPaymentOutputDescriptor, mut val: crate::c_types::TxOut) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.output = val.into_rust();
}
/// Arbitrary identification information returned by a call to
/// `Sign::channel_keys_id()`. This may be useful in re-deriving keys used in
/// the channel to spend the output.
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_get_channel_keys_id(this_ptr: &StaticPaymentOutputDescriptor) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_keys_id;
	inner_val
}
/// Arbitrary identification information returned by a call to
/// `Sign::channel_keys_id()`. This may be useful in re-deriving keys used in
/// the channel to spend the output.
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_set_channel_keys_id(this_ptr: &mut StaticPaymentOutputDescriptor, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_keys_id = val.data;
}
/// The value of the channel which this transactions spends.
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_get_channel_value_satoshis(this_ptr: &StaticPaymentOutputDescriptor) -> u64 {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().channel_value_satoshis;
	*inner_val
}
/// The value of the channel which this transactions spends.
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_set_channel_value_satoshis(this_ptr: &mut StaticPaymentOutputDescriptor, mut val: u64) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.channel_value_satoshis = val;
}
/// Constructs a new StaticPaymentOutputDescriptor given each field
#[must_use]
#[no_mangle]
pub extern "C" fn StaticPaymentOutputDescriptor_new(mut outpoint_arg: crate::lightning::chain::transaction::OutPoint, mut output_arg: crate::c_types::TxOut, mut channel_keys_id_arg: crate::c_types::ThirtyTwoBytes, mut channel_value_satoshis_arg: u64) -> StaticPaymentOutputDescriptor {
	StaticPaymentOutputDescriptor { inner: ObjOps::heap_alloc(nativeStaticPaymentOutputDescriptor {
		outpoint: *unsafe { Box::from_raw(outpoint_arg.take_inner()) },
		output: output_arg.into_rust(),
		channel_keys_id: channel_keys_id_arg.data,
		channel_value_satoshis: channel_value_satoshis_arg,
	}), is_owned: true }
}
impl Clone for StaticPaymentOutputDescriptor {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeStaticPaymentOutputDescriptor>::is_null(self.inner) { std::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn StaticPaymentOutputDescriptor_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeStaticPaymentOutputDescriptor)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the StaticPaymentOutputDescriptor
pub extern "C" fn StaticPaymentOutputDescriptor_clone(orig: &StaticPaymentOutputDescriptor) -> StaticPaymentOutputDescriptor {
	orig.clone()
}
#[no_mangle]
/// Serialize the StaticPaymentOutputDescriptor object into a byte array which can be read by StaticPaymentOutputDescriptor_read
pub extern "C" fn StaticPaymentOutputDescriptor_write(obj: &StaticPaymentOutputDescriptor) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn StaticPaymentOutputDescriptor_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeStaticPaymentOutputDescriptor) })
}
#[no_mangle]
/// Read a StaticPaymentOutputDescriptor from a byte array, created by StaticPaymentOutputDescriptor_write
pub extern "C" fn StaticPaymentOutputDescriptor_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_StaticPaymentOutputDescriptorDecodeErrorZ {
	let res: Result<lightning::chain::keysinterface::StaticPaymentOutputDescriptor, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_res
}
/// When on-chain outputs are created by rust-lightning (which our counterparty is not able to
/// claim at any point in the future) an event is generated which you must track and be able to
/// spend on-chain. The information needed to do this is provided in this enum, including the
/// outpoint describing which txid and output index is available, the full output which exists at
/// that txid/index, and any keys or other information required to sign.
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum SpendableOutputDescriptor {
	/// An output to a script which was provided via KeysInterface directly, either from
	/// `get_destination_script()` or `get_shutdown_scriptpubkey()`, thus you should already know
	/// how to spend it. No secret keys are provided as rust-lightning was never given any key.
	/// These may include outputs from a transaction punishing our counterparty or claiming an HTLC
	/// on-chain using the payment preimage or after it has timed out.
	StaticOutput {
		/// The outpoint which is spendable
		outpoint: crate::lightning::chain::transaction::OutPoint,
		/// The output which is referenced by the given outpoint.
		output: crate::c_types::TxOut,
	},
	/// An output to a P2WSH script which can be spent with a single signature after a CSV delay.
	///
	/// The witness in the spending input should be:
	/// <BIP 143 signature> <empty vector> (MINIMALIF standard rule) <provided witnessScript>
	///
	/// Note that the nSequence field in the spending input must be set to to_self_delay
	/// (which means the transaction is not broadcastable until at least to_self_delay
	/// blocks after the outpoint confirms).
	///
	/// These are generally the result of a \"revocable\" output to us, spendable only by us unless
	/// it is an output from an old state which we broadcast (which should never happen).
	///
	/// To derive the delayed_payment key which is used to sign for this input, you must pass the
	/// holder delayed_payment_base_key (ie the private key which corresponds to the pubkey in
	/// Sign::pubkeys().delayed_payment_basepoint) and the provided per_commitment_point to
	/// chan_utils::derive_private_key. The public key can be generated without the secret key
	/// using chan_utils::derive_public_key and only the delayed_payment_basepoint which appears in
	/// Sign::pubkeys().
	///
	/// To derive the revocation_pubkey provided here (which is used in the witness
	/// script generation), you must pass the counterparty revocation_basepoint (which appears in the
	/// call to Sign::ready_channel) and the provided per_commitment point
	/// to chan_utils::derive_public_revocation_key.
	///
	/// The witness script which is hashed and included in the output script_pubkey may be
	/// regenerated by passing the revocation_pubkey (derived as above), our delayed_payment pubkey
	/// (derived as above), and the to_self_delay contained here to
	/// chan_utils::get_revokeable_redeemscript.
	DelayedPaymentOutput(crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor),
	/// An output to a P2WPKH, spendable exclusively by our payment key (ie the private key which
	/// corresponds to the public key in Sign::pubkeys().payment_point).
	/// The witness in the spending input, is, thus, simply:
	/// <BIP 143 signature> <payment key>
	///
	/// These are generally the result of our counterparty having broadcast the current state,
	/// allowing us to claim the non-HTLC-encumbered outputs immediately.
	StaticPaymentOutput(crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor),
}
use lightning::chain::keysinterface::SpendableOutputDescriptor as nativeSpendableOutputDescriptor;
impl SpendableOutputDescriptor {
	#[allow(unused)]
	pub(crate) fn to_native(&self) -> nativeSpendableOutputDescriptor {
		match self {
			SpendableOutputDescriptor::StaticOutput {ref outpoint, ref output, } => {
				let mut outpoint_nonref = (*outpoint).clone();
				let mut output_nonref = (*output).clone();
				nativeSpendableOutputDescriptor::StaticOutput {
					outpoint: *unsafe { Box::from_raw(outpoint_nonref.take_inner()) },
					output: output_nonref.into_rust(),
				}
			},
			SpendableOutputDescriptor::DelayedPaymentOutput (ref a, ) => {
				let mut a_nonref = (*a).clone();
				nativeSpendableOutputDescriptor::DelayedPaymentOutput (
					*unsafe { Box::from_raw(a_nonref.take_inner()) },
				)
			},
			SpendableOutputDescriptor::StaticPaymentOutput (ref a, ) => {
				let mut a_nonref = (*a).clone();
				nativeSpendableOutputDescriptor::StaticPaymentOutput (
					*unsafe { Box::from_raw(a_nonref.take_inner()) },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_native(self) -> nativeSpendableOutputDescriptor {
		match self {
			SpendableOutputDescriptor::StaticOutput {mut outpoint, mut output, } => {
				nativeSpendableOutputDescriptor::StaticOutput {
					outpoint: *unsafe { Box::from_raw(outpoint.take_inner()) },
					output: output.into_rust(),
				}
			},
			SpendableOutputDescriptor::DelayedPaymentOutput (mut a, ) => {
				nativeSpendableOutputDescriptor::DelayedPaymentOutput (
					*unsafe { Box::from_raw(a.take_inner()) },
				)
			},
			SpendableOutputDescriptor::StaticPaymentOutput (mut a, ) => {
				nativeSpendableOutputDescriptor::StaticPaymentOutput (
					*unsafe { Box::from_raw(a.take_inner()) },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_native(native: &nativeSpendableOutputDescriptor) -> Self {
		match native {
			nativeSpendableOutputDescriptor::StaticOutput {ref outpoint, ref output, } => {
				let mut outpoint_nonref = (*outpoint).clone();
				let mut output_nonref = (*output).clone();
				SpendableOutputDescriptor::StaticOutput {
					outpoint: crate::lightning::chain::transaction::OutPoint { inner: ObjOps::heap_alloc(outpoint_nonref), is_owned: true },
					output: crate::c_types::TxOut::from_rust(output_nonref),
				}
			},
			nativeSpendableOutputDescriptor::DelayedPaymentOutput (ref a, ) => {
				let mut a_nonref = (*a).clone();
				SpendableOutputDescriptor::DelayedPaymentOutput (
					crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor { inner: ObjOps::heap_alloc(a_nonref), is_owned: true },
				)
			},
			nativeSpendableOutputDescriptor::StaticPaymentOutput (ref a, ) => {
				let mut a_nonref = (*a).clone();
				SpendableOutputDescriptor::StaticPaymentOutput (
					crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor { inner: ObjOps::heap_alloc(a_nonref), is_owned: true },
				)
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn native_into(native: nativeSpendableOutputDescriptor) -> Self {
		match native {
			nativeSpendableOutputDescriptor::StaticOutput {mut outpoint, mut output, } => {
				SpendableOutputDescriptor::StaticOutput {
					outpoint: crate::lightning::chain::transaction::OutPoint { inner: ObjOps::heap_alloc(outpoint), is_owned: true },
					output: crate::c_types::TxOut::from_rust(output),
				}
			},
			nativeSpendableOutputDescriptor::DelayedPaymentOutput (mut a, ) => {
				SpendableOutputDescriptor::DelayedPaymentOutput (
					crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor { inner: ObjOps::heap_alloc(a), is_owned: true },
				)
			},
			nativeSpendableOutputDescriptor::StaticPaymentOutput (mut a, ) => {
				SpendableOutputDescriptor::StaticPaymentOutput (
					crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor { inner: ObjOps::heap_alloc(a), is_owned: true },
				)
			},
		}
	}
}
/// Frees any resources used by the SpendableOutputDescriptor
#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_free(this_ptr: SpendableOutputDescriptor) { }
/// Creates a copy of the SpendableOutputDescriptor
#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_clone(orig: &SpendableOutputDescriptor) -> SpendableOutputDescriptor {
	orig.clone()
}
#[no_mangle]
/// Utility method to constructs a new StaticOutput-variant SpendableOutputDescriptor
pub extern "C" fn SpendableOutputDescriptor_static_output(outpoint: crate::lightning::chain::transaction::OutPoint, output: crate::c_types::TxOut) -> SpendableOutputDescriptor {
	SpendableOutputDescriptor::StaticOutput {
		outpoint,
		output,
	}
}
#[no_mangle]
/// Utility method to constructs a new DelayedPaymentOutput-variant SpendableOutputDescriptor
pub extern "C" fn SpendableOutputDescriptor_delayed_payment_output(a: crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor) -> SpendableOutputDescriptor {
	SpendableOutputDescriptor::DelayedPaymentOutput(a, )
}
#[no_mangle]
/// Utility method to constructs a new StaticPaymentOutput-variant SpendableOutputDescriptor
pub extern "C" fn SpendableOutputDescriptor_static_payment_output(a: crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor) -> SpendableOutputDescriptor {
	SpendableOutputDescriptor::StaticPaymentOutput(a, )
}
#[no_mangle]
/// Serialize the SpendableOutputDescriptor object into a byte array which can be read by SpendableOutputDescriptor_read
pub extern "C" fn SpendableOutputDescriptor_write(obj: &SpendableOutputDescriptor) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(&unsafe { &*obj }.to_native())
}
#[no_mangle]
/// Read a SpendableOutputDescriptor from a byte array, created by SpendableOutputDescriptor_write
pub extern "C" fn SpendableOutputDescriptor_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_SpendableOutputDescriptorDecodeErrorZ {
	let res: Result<lightning::chain::keysinterface::SpendableOutputDescriptor, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::chain::keysinterface::SpendableOutputDescriptor::native_into(o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_res
}
/// A trait to sign lightning channel transactions as described in BOLT 3.
///
/// Signing services could be implemented on a hardware wallet. In this case,
/// the current Sign would be a front-end on top of a communication
/// channel connected to your secure device and lightning key material wouldn't
/// reside on a hot server. Nevertheless, a this deployment would still need
/// to trust the ChannelManager to avoid loss of funds as this latest component
/// could ask to sign commitment transaction with HTLCs paying to attacker pubkeys.
///
/// A more secure iteration would be to use hashlock (or payment points) to pair
/// invoice/incoming HTLCs with outgoing HTLCs to implement a no-trust-ChannelManager
/// at the price of more state and computation on the hardware wallet side. In the future,
/// we are looking forward to design such interface.
///
/// In any case, ChannelMonitor or fallback watchtowers are always going to be trusted
/// to act, as liveness and breach reply correctness are always going to be hard requirements
/// of LN security model, orthogonal of key management issues.
#[repr(C)]
pub struct BaseSign {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Gets the per-commitment point for a specific commitment number
	///
	/// Note that the commitment number starts at (1 << 48) - 1 and counts backwards.
	#[must_use]
	pub get_per_commitment_point: extern "C" fn (this_arg: *const c_void, idx: u64) -> crate::c_types::PublicKey,
	/// Gets the commitment secret for a specific commitment number as part of the revocation process
	///
	/// An external signer implementation should error here if the commitment was already signed
	/// and should refuse to sign it in the future.
	///
	/// May be called more than once for the same index.
	///
	/// Note that the commitment number starts at (1 << 48) - 1 and counts backwards.
	#[must_use]
	pub release_commitment_secret: extern "C" fn (this_arg: *const c_void, idx: u64) -> crate::c_types::ThirtyTwoBytes,
	/// Validate the counterparty's signatures on the holder commitment transaction and HTLCs.
	///
	/// This is required in order for the signer to make sure that releasing a commitment
	/// secret won't leave us without a broadcastable holder transaction.
	/// Policy checks should be implemented in this function, including checking the amount
	/// sent to us and checking the HTLCs.
	#[must_use]
	pub validate_holder_commitment: extern "C" fn (this_arg: *const c_void, holder_tx: &crate::lightning::ln::chan_utils::HolderCommitmentTransaction) -> crate::c_types::derived::CResult_NoneNoneZ,
	/// Gets the holder's channel public keys and basepoints
	pub pubkeys: crate::lightning::ln::chan_utils::ChannelPublicKeys,
	/// Fill in the pubkeys field as a reference to it will be given to Rust after this returns
	/// Note that this takes a pointer to this object, not the this_ptr like other methods do
	/// This function pointer may be NULL if pubkeys is filled in when this object is created and never needs updating.
	pub set_pubkeys: Option<extern "C" fn(&BaseSign)>,
	/// Gets an arbitrary identifier describing the set of keys which are provided back to you in
	/// some SpendableOutputDescriptor types. This should be sufficient to identify this
	/// Sign object uniquely and lookup or re-derive its keys.
	#[must_use]
	pub channel_keys_id: extern "C" fn (this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes,
	/// Create a signature for a counterparty's commitment transaction and associated HTLC transactions.
	///
	/// Note that if signing fails or is rejected, the channel will be force-closed.
	///
	/// Policy checks should be implemented in this function, including checking the amount
	/// sent to us and checking the HTLCs.
	#[must_use]
	pub sign_counterparty_commitment: extern "C" fn (this_arg: *const c_void, commitment_tx: &crate::lightning::ln::chan_utils::CommitmentTransaction) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ,
	/// Validate the counterparty's revocation.
	///
	/// This is required in order for the signer to make sure that the state has moved
	/// forward and it is safe to sign the next counterparty commitment.
	#[must_use]
	pub validate_counterparty_revocation: extern "C" fn (this_arg: *const c_void, idx: u64, secret: *const [u8; 32]) -> crate::c_types::derived::CResult_NoneNoneZ,
	/// Create a signatures for a holder's commitment transaction and its claiming HTLC transactions.
	/// This will only ever be called with a non-revoked commitment_tx.  This will be called with the
	/// latest commitment_tx when we initiate a force-close.
	/// This will be called with the previous latest, just to get claiming HTLC signatures, if we are
	/// reacting to a ChannelMonitor replica that decided to broadcast before it had been updated to
	/// the latest.
	/// This may be called multiple times for the same transaction.
	///
	/// An external signer implementation should check that the commitment has not been revoked.
	///
	/// May return Err if key derivation fails.  Callers, such as ChannelMonitor, will panic in such a case.
	#[must_use]
	pub sign_holder_commitment_and_htlcs: extern "C" fn (this_arg: *const c_void, commitment_tx: &crate::lightning::ln::chan_utils::HolderCommitmentTransaction) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ,
	/// Create a signature for the given input in a transaction spending an HTLC transaction output
	/// or a commitment transaction `to_local` output when our counterparty broadcasts an old state.
	///
	/// A justice transaction may claim multiple outputs at the same time if timelocks are
	/// similar, but only a signature for the input at index `input` should be signed for here.
	/// It may be called multiple times for same output(s) if a fee-bump is needed with regards
	/// to an upcoming timelock expiration.
	///
	/// Amount is value of the output spent by this input, committed to in the BIP 143 signature.
	///
	/// per_commitment_key is revocation secret which was provided by our counterparty when they
	/// revoked the state which they eventually broadcast. It's not a _holder_ secret key and does
	/// not allow the spending of any funds by itself (you need our holder revocation_secret to do
	/// so).
	#[must_use]
	pub sign_justice_revoked_output: extern "C" fn (this_arg: *const c_void, justice_tx: crate::c_types::Transaction, input: usize, amount: u64, per_commitment_key: *const [u8; 32]) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// Create a signature for the given input in a transaction spending a commitment transaction
	/// HTLC output when our counterparty broadcasts an old state.
	///
	/// A justice transaction may claim multiple outputs at the same time if timelocks are
	/// similar, but only a signature for the input at index `input` should be signed for here.
	/// It may be called multiple times for same output(s) if a fee-bump is needed with regards
	/// to an upcoming timelock expiration.
	///
	/// Amount is value of the output spent by this input, committed to in the BIP 143 signature.
	///
	/// per_commitment_key is revocation secret which was provided by our counterparty when they
	/// revoked the state which they eventually broadcast. It's not a _holder_ secret key and does
	/// not allow the spending of any funds by itself (you need our holder revocation_secret to do
	/// so).
	///
	/// htlc holds HTLC elements (hash, timelock), thus changing the format of the witness script
	/// (which is committed to in the BIP 143 signatures).
	#[must_use]
	pub sign_justice_revoked_htlc: extern "C" fn (this_arg: *const c_void, justice_tx: crate::c_types::Transaction, input: usize, amount: u64, per_commitment_key: *const [u8; 32], htlc: &crate::lightning::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// Create a signature for a claiming transaction for a HTLC output on a counterparty's commitment
	/// transaction, either offered or received.
	///
	/// Such a transaction may claim multiples offered outputs at same time if we know the
	/// preimage for each when we create it, but only the input at index `input` should be
	/// signed for here. It may be called multiple times for same output(s) if a fee-bump is
	/// needed with regards to an upcoming timelock expiration.
	///
	/// Witness_script is either a offered or received script as defined in BOLT3 for HTLC
	/// outputs.
	///
	/// Amount is value of the output spent by this input, committed to in the BIP 143 signature.
	///
	/// Per_commitment_point is the dynamic point corresponding to the channel state
	/// detected onchain. It has been generated by our counterparty and is used to derive
	/// channel state keys, which are then included in the witness script and committed to in the
	/// BIP 143 signature.
	#[must_use]
	pub sign_counterparty_htlc_transaction: extern "C" fn (this_arg: *const c_void, htlc_tx: crate::c_types::Transaction, input: usize, amount: u64, per_commitment_point: crate::c_types::PublicKey, htlc: &crate::lightning::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// Create a signature for a (proposed) closing transaction.
	///
	/// Note that, due to rounding, there may be one \"missing\" satoshi, and either party may have
	/// chosen to forgo their output as dust.
	#[must_use]
	pub sign_closing_transaction: extern "C" fn (this_arg: *const c_void, closing_tx: &crate::lightning::ln::chan_utils::ClosingTransaction) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// Signs a channel announcement message with our funding key, proving it comes from one
	/// of the channel participants.
	///
	/// Note that if this fails or is rejected, the channel will not be publicly announced and
	/// our counterparty may (though likely will not) close the channel on us for violating the
	/// protocol.
	#[must_use]
	pub sign_channel_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::lightning::ln::msgs::UnsignedChannelAnnouncement) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// Set the counterparty static channel data, including basepoints,
	/// counterparty_selected/holder_selected_contest_delay and funding outpoint.
	/// This is done as soon as the funding outpoint is known.  Since these are static channel data,
	/// they MUST NOT be allowed to change to different values once set.
	///
	/// channel_parameters.is_populated() MUST be true.
	///
	/// We bind holder_selected_contest_delay late here for API convenience.
	///
	/// Will be called before any signatures are applied.
	pub ready_channel: extern "C" fn (this_arg: *mut c_void, channel_parameters: &crate::lightning::ln::chan_utils::ChannelTransactionParameters),
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for BaseSign {}
unsafe impl Sync for BaseSign {}
#[no_mangle]
pub(crate) extern "C" fn BaseSign_clone_fields(orig: &BaseSign) -> BaseSign {
	BaseSign {
		this_arg: orig.this_arg,
		get_per_commitment_point: Clone::clone(&orig.get_per_commitment_point),
		release_commitment_secret: Clone::clone(&orig.release_commitment_secret),
		validate_holder_commitment: Clone::clone(&orig.validate_holder_commitment),
		pubkeys: Clone::clone(&orig.pubkeys),
		set_pubkeys: Clone::clone(&orig.set_pubkeys),
		channel_keys_id: Clone::clone(&orig.channel_keys_id),
		sign_counterparty_commitment: Clone::clone(&orig.sign_counterparty_commitment),
		validate_counterparty_revocation: Clone::clone(&orig.validate_counterparty_revocation),
		sign_holder_commitment_and_htlcs: Clone::clone(&orig.sign_holder_commitment_and_htlcs),
		sign_justice_revoked_output: Clone::clone(&orig.sign_justice_revoked_output),
		sign_justice_revoked_htlc: Clone::clone(&orig.sign_justice_revoked_htlc),
		sign_counterparty_htlc_transaction: Clone::clone(&orig.sign_counterparty_htlc_transaction),
		sign_closing_transaction: Clone::clone(&orig.sign_closing_transaction),
		sign_channel_announcement: Clone::clone(&orig.sign_channel_announcement),
		ready_channel: Clone::clone(&orig.ready_channel),
		free: Clone::clone(&orig.free),
	}
}

use lightning::chain::keysinterface::BaseSign as rustBaseSign;
impl rustBaseSign for BaseSign {
	fn get_per_commitment_point(&self, mut idx: u64, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> bitcoin::secp256k1::key::PublicKey {
		let mut ret = (self.get_per_commitment_point)(self.this_arg, idx);
		ret.into_rust()
	}
	fn release_commitment_secret(&self, mut idx: u64) -> [u8; 32] {
		let mut ret = (self.release_commitment_secret)(self.this_arg, idx);
		ret.data
	}
	fn validate_holder_commitment(&self, mut holder_tx: &lightning::ln::chan_utils::HolderCommitmentTransaction) -> Result<(), ()> {
		let mut ret = (self.validate_holder_commitment)(self.this_arg, &crate::lightning::ln::chan_utils::HolderCommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((holder_tx as *const lightning::ln::chan_utils::HolderCommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn pubkeys(&self) -> &lightning::ln::chan_utils::ChannelPublicKeys {
		if let Some(f) = self.set_pubkeys {
			(f)(&self);
		}
		self.pubkeys.get_native_ref()
	}
	fn channel_keys_id(&self) -> [u8; 32] {
		let mut ret = (self.channel_keys_id)(self.this_arg);
		ret.data
	}
	fn sign_counterparty_commitment(&self, mut commitment_tx: &lightning::ln::chan_utils::CommitmentTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		let mut ret = (self.sign_counterparty_commitment)(self.this_arg, &crate::lightning::ln::chan_utils::CommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((commitment_tx as *const lightning::ln::chan_utils::CommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).to_rust(); let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.into_rust().drain(..) { local_orig_ret_0_1.push( { item.into_rust() }); }; let mut local_ret_0 = (orig_ret_0_0.into_rust(), local_orig_ret_0_1); local_ret_0 }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn validate_counterparty_revocation(&self, mut idx: u64, mut secret: &bitcoin::secp256k1::key::SecretKey) -> Result<(), ()> {
		let mut ret = (self.validate_counterparty_revocation)(self.this_arg, idx, secret.as_ref());
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_holder_commitment_and_htlcs(&self, mut commitment_tx: &lightning::ln::chan_utils::HolderCommitmentTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		let mut ret = (self.sign_holder_commitment_and_htlcs)(self.this_arg, &crate::lightning::ln::chan_utils::HolderCommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((commitment_tx as *const lightning::ln::chan_utils::HolderCommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).to_rust(); let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.into_rust().drain(..) { local_orig_ret_0_1.push( { item.into_rust() }); }; let mut local_ret_0 = (orig_ret_0_0.into_rust(), local_orig_ret_0_1); local_ret_0 }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_justice_revoked_output(&self, mut justice_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_key: &bitcoin::secp256k1::key::SecretKey, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_justice_revoked_output)(self.this_arg, crate::c_types::Transaction::from_bitcoin(justice_tx), input, amount, per_commitment_key.as_ref());
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_justice_revoked_htlc(&self, mut justice_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_key: &bitcoin::secp256k1::key::SecretKey, mut htlc: &lightning::ln::chan_utils::HTLCOutputInCommitment, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_justice_revoked_htlc)(self.this_arg, crate::c_types::Transaction::from_bitcoin(justice_tx), input, amount, per_commitment_key.as_ref(), &crate::lightning::ln::chan_utils::HTLCOutputInCommitment { inner: unsafe { ObjOps::nonnull_ptr_to_inner((htlc as *const lightning::ln::chan_utils::HTLCOutputInCommitment<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_counterparty_htlc_transaction(&self, mut htlc_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_point: &bitcoin::secp256k1::key::PublicKey, mut htlc: &lightning::ln::chan_utils::HTLCOutputInCommitment, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_counterparty_htlc_transaction)(self.this_arg, crate::c_types::Transaction::from_bitcoin(htlc_tx), input, amount, crate::c_types::PublicKey::from_rust(&per_commitment_point), &crate::lightning::ln::chan_utils::HTLCOutputInCommitment { inner: unsafe { ObjOps::nonnull_ptr_to_inner((htlc as *const lightning::ln::chan_utils::HTLCOutputInCommitment<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_closing_transaction(&self, mut closing_tx: &lightning::ln::chan_utils::ClosingTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_closing_transaction)(self.this_arg, &crate::lightning::ln::chan_utils::ClosingTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((closing_tx as *const lightning::ln::chan_utils::ClosingTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_channel_announcement(&self, mut msg: &lightning::ln::msgs::UnsignedChannelAnnouncement, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_channel_announcement)(self.this_arg, &crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UnsignedChannelAnnouncement<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn ready_channel(&mut self, mut channel_parameters: &lightning::ln::chan_utils::ChannelTransactionParameters) {
		(self.ready_channel)(self.this_arg, &crate::lightning::ln::chan_utils::ChannelTransactionParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((channel_parameters as *const lightning::ln::chan_utils::ChannelTransactionParameters<>) as *mut _) }, is_owned: false })
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for BaseSign {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn BaseSign_free(this_ptr: BaseSign) { }
impl Drop for BaseSign {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}
/// A cloneable signer.
///
/// Although we require signers to be cloneable, it may be useful for developers to be able to use
/// signers in an un-sized way, for example as `dyn BaseSign`. Therefore we separate the Clone trait,
/// which implies Sized, into this derived trait.
#[repr(C)]
pub struct Sign {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Implementation of BaseSign for this object.
	pub BaseSign: crate::lightning::chain::keysinterface::BaseSign,
	/// Serialize the object into a byte array
	pub write: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z,
	/// Called, if set, after this Sign has been cloned into a duplicate object.
	/// The new Sign is provided, and should be mutated as needed to perform a
	/// deep copy of the object pointed to by this_arg or avoid any double-freeing.
	pub cloned: Option<extern "C" fn (new_Sign: &mut Sign)>,
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for Sign {}
unsafe impl Sync for Sign {}
#[no_mangle]
pub(crate) extern "C" fn Sign_clone_fields(orig: &Sign) -> Sign {
	Sign {
		this_arg: orig.this_arg,
		BaseSign: crate::lightning::chain::keysinterface::BaseSign_clone_fields(&orig.BaseSign),
		write: Clone::clone(&orig.write),
		cloned: Clone::clone(&orig.cloned),
		free: Clone::clone(&orig.free),
	}
}
impl lightning::chain::keysinterface::BaseSign for Sign {
	fn get_per_commitment_point(&self, mut idx: u64, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> bitcoin::secp256k1::key::PublicKey {
		let mut ret = (self.BaseSign.get_per_commitment_point)(self.BaseSign.this_arg, idx);
		ret.into_rust()
	}
	fn release_commitment_secret(&self, mut idx: u64) -> [u8; 32] {
		let mut ret = (self.BaseSign.release_commitment_secret)(self.BaseSign.this_arg, idx);
		ret.data
	}
	fn validate_holder_commitment(&self, mut holder_tx: &lightning::ln::chan_utils::HolderCommitmentTransaction) -> Result<(), ()> {
		let mut ret = (self.BaseSign.validate_holder_commitment)(self.BaseSign.this_arg, &crate::lightning::ln::chan_utils::HolderCommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((holder_tx as *const lightning::ln::chan_utils::HolderCommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn pubkeys(&self) -> &lightning::ln::chan_utils::ChannelPublicKeys {
		if let Some(f) = self.BaseSign.set_pubkeys {
			(f)(&self.BaseSign);
		}
		self.BaseSign.pubkeys.get_native_ref()
	}
	fn channel_keys_id(&self) -> [u8; 32] {
		let mut ret = (self.BaseSign.channel_keys_id)(self.BaseSign.this_arg);
		ret.data
	}
	fn sign_counterparty_commitment(&self, mut commitment_tx: &lightning::ln::chan_utils::CommitmentTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		let mut ret = (self.BaseSign.sign_counterparty_commitment)(self.BaseSign.this_arg, &crate::lightning::ln::chan_utils::CommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((commitment_tx as *const lightning::ln::chan_utils::CommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).to_rust(); let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.into_rust().drain(..) { local_orig_ret_0_1.push( { item.into_rust() }); }; let mut local_ret_0 = (orig_ret_0_0.into_rust(), local_orig_ret_0_1); local_ret_0 }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn validate_counterparty_revocation(&self, mut idx: u64, mut secret: &bitcoin::secp256k1::key::SecretKey) -> Result<(), ()> {
		let mut ret = (self.BaseSign.validate_counterparty_revocation)(self.BaseSign.this_arg, idx, secret.as_ref());
		let mut local_ret = match ret.result_ok { true => Ok( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) })*/ }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_holder_commitment_and_htlcs(&self, mut commitment_tx: &lightning::ln::chan_utils::HolderCommitmentTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		let mut ret = (self.BaseSign.sign_holder_commitment_and_htlcs)(self.BaseSign.this_arg, &crate::lightning::ln::chan_utils::HolderCommitmentTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((commitment_tx as *const lightning::ln::chan_utils::HolderCommitmentTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).to_rust(); let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.into_rust().drain(..) { local_orig_ret_0_1.push( { item.into_rust() }); }; let mut local_ret_0 = (orig_ret_0_0.into_rust(), local_orig_ret_0_1); local_ret_0 }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_justice_revoked_output(&self, mut justice_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_key: &bitcoin::secp256k1::key::SecretKey, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.BaseSign.sign_justice_revoked_output)(self.BaseSign.this_arg, crate::c_types::Transaction::from_bitcoin(justice_tx), input, amount, per_commitment_key.as_ref());
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_justice_revoked_htlc(&self, mut justice_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_key: &bitcoin::secp256k1::key::SecretKey, mut htlc: &lightning::ln::chan_utils::HTLCOutputInCommitment, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.BaseSign.sign_justice_revoked_htlc)(self.BaseSign.this_arg, crate::c_types::Transaction::from_bitcoin(justice_tx), input, amount, per_commitment_key.as_ref(), &crate::lightning::ln::chan_utils::HTLCOutputInCommitment { inner: unsafe { ObjOps::nonnull_ptr_to_inner((htlc as *const lightning::ln::chan_utils::HTLCOutputInCommitment<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_counterparty_htlc_transaction(&self, mut htlc_tx: &bitcoin::blockdata::transaction::Transaction, mut input: usize, mut amount: u64, mut per_commitment_point: &bitcoin::secp256k1::key::PublicKey, mut htlc: &lightning::ln::chan_utils::HTLCOutputInCommitment, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.BaseSign.sign_counterparty_htlc_transaction)(self.BaseSign.this_arg, crate::c_types::Transaction::from_bitcoin(htlc_tx), input, amount, crate::c_types::PublicKey::from_rust(&per_commitment_point), &crate::lightning::ln::chan_utils::HTLCOutputInCommitment { inner: unsafe { ObjOps::nonnull_ptr_to_inner((htlc as *const lightning::ln::chan_utils::HTLCOutputInCommitment<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_closing_transaction(&self, mut closing_tx: &lightning::ln::chan_utils::ClosingTransaction, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.BaseSign.sign_closing_transaction)(self.BaseSign.this_arg, &crate::lightning::ln::chan_utils::ClosingTransaction { inner: unsafe { ObjOps::nonnull_ptr_to_inner((closing_tx as *const lightning::ln::chan_utils::ClosingTransaction<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn sign_channel_announcement(&self, mut msg: &lightning::ln::msgs::UnsignedChannelAnnouncement, mut _secp_ctx: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.BaseSign.sign_channel_announcement)(self.BaseSign.this_arg, &crate::lightning::ln::msgs::UnsignedChannelAnnouncement { inner: unsafe { ObjOps::nonnull_ptr_to_inner((msg as *const lightning::ln::msgs::UnsignedChannelAnnouncement<>) as *mut _) }, is_owned: false });
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn ready_channel(&mut self, mut channel_parameters: &lightning::ln::chan_utils::ChannelTransactionParameters) {
		(self.BaseSign.ready_channel)(self.BaseSign.this_arg, &crate::lightning::ln::chan_utils::ChannelTransactionParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((channel_parameters as *const lightning::ln::chan_utils::ChannelTransactionParameters<>) as *mut _) }, is_owned: false })
	}
}
impl lightning::util::ser::Writeable for Sign {
	fn write<W: lightning::util::ser::Writer>(&self, w: &mut W) -> Result<(), ::std::io::Error> {
		let vec = (self.write)(self.this_arg);
		w.write_all(vec.as_slice())
	}
}
#[no_mangle]
/// Creates a copy of a Sign
pub extern "C" fn Sign_clone(orig: &Sign) -> Sign {
	let mut res = Sign_clone_fields(orig);
	if let Some(f) = orig.cloned { (f)(&mut res) };
	res
}
impl Clone for Sign {
	fn clone(&self) -> Self {
		Sign_clone(self)
	}
}

use lightning::chain::keysinterface::Sign as rustSign;
impl rustSign for Sign {
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for Sign {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn Sign_free(this_ptr: Sign) { }
impl Drop for Sign {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}
/// A trait to describe an object which can get user secrets and key material.
#[repr(C)]
pub struct KeysInterface {
	/// An opaque pointer which is passed to your function implementations as an argument.
	/// This has no meaning in the LDK, and can be NULL or any other value.
	pub this_arg: *mut c_void,
	/// Get node secret key (aka node_id or network_key).
	///
	/// This method must return the same value each time it is called.
	#[must_use]
	pub get_node_secret: extern "C" fn (this_arg: *const c_void) -> crate::c_types::SecretKey,
	/// Get a script pubkey which we send funds to when claiming on-chain contestable outputs.
	///
	/// This method should return a different value each time it is called, to avoid linking
	/// on-chain funds across channels as controlled to the same user.
	#[must_use]
	pub get_destination_script: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z,
	/// Get a script pubkey which we will send funds to when closing a channel.
	///
	/// This method should return a different value each time it is called, to avoid linking
	/// on-chain funds across channels as controlled to the same user.
	#[must_use]
	pub get_shutdown_scriptpubkey: extern "C" fn (this_arg: *const c_void) -> crate::lightning::ln::script::ShutdownScript,
	/// Get a new set of Sign for per-channel secrets. These MUST be unique even if you
	/// restarted with some stale data!
	///
	/// This method must return a different value each time it is called.
	#[must_use]
	pub get_channel_signer: extern "C" fn (this_arg: *const c_void, inbound: bool, channel_value_satoshis: u64) -> crate::lightning::chain::keysinterface::Sign,
	/// Gets a unique, cryptographically-secure, random 32 byte value. This is used for encrypting
	/// onion packets and for temporary channel IDs. There is no requirement that these be
	/// persisted anywhere, though they must be unique across restarts.
	///
	/// This method must return a different value each time it is called.
	#[must_use]
	pub get_secure_random_bytes: extern "C" fn (this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes,
	/// Reads a `Signer` for this `KeysInterface` from the given input stream.
	/// This is only called during deserialization of other objects which contain
	/// `Sign`-implementing objects (ie `ChannelMonitor`s and `ChannelManager`s).
	/// The bytes are exactly those which `<Self::Signer as Writeable>::write()` writes, and
	/// contain no versioning scheme. You may wish to include your own version prefix and ensure
	/// you've read all of the provided bytes to ensure no corruption occurred.
	#[must_use]
	pub read_chan_signer: extern "C" fn (this_arg: *const c_void, reader: crate::c_types::u8slice) -> crate::c_types::derived::CResult_SignDecodeErrorZ,
	/// Sign an invoice's preimage (note that this is the preimage of the invoice, not the HTLC's
	/// preimage). By parameterizing by the preimage instead of the hash, we allow implementors of
	/// this trait to parse the invoice and make sure they're signing what they expect, rather than
	/// blindly signing the hash.
	#[must_use]
	pub sign_invoice: extern "C" fn (this_arg: *const c_void, invoice_preimage: crate::c_types::derived::CVec_u8Z) -> crate::c_types::derived::CResult_RecoverableSignatureNoneZ,
	/// Get secret key material as bytes for use in encrypting and decrypting inbound payment data.
	///
	/// This method must return the same value each time it is called.
	#[must_use]
	pub get_inbound_payment_key_material: extern "C" fn (this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes,
	/// Frees any resources associated with this object given its this_arg pointer.
	/// Does not need to free the outer struct containing function pointers and may be NULL is no resources need to be freed.
	pub free: Option<extern "C" fn(this_arg: *mut c_void)>,
}
unsafe impl Send for KeysInterface {}
unsafe impl Sync for KeysInterface {}
#[no_mangle]
pub(crate) extern "C" fn KeysInterface_clone_fields(orig: &KeysInterface) -> KeysInterface {
	KeysInterface {
		this_arg: orig.this_arg,
		get_node_secret: Clone::clone(&orig.get_node_secret),
		get_destination_script: Clone::clone(&orig.get_destination_script),
		get_shutdown_scriptpubkey: Clone::clone(&orig.get_shutdown_scriptpubkey),
		get_channel_signer: Clone::clone(&orig.get_channel_signer),
		get_secure_random_bytes: Clone::clone(&orig.get_secure_random_bytes),
		read_chan_signer: Clone::clone(&orig.read_chan_signer),
		sign_invoice: Clone::clone(&orig.sign_invoice),
		get_inbound_payment_key_material: Clone::clone(&orig.get_inbound_payment_key_material),
		free: Clone::clone(&orig.free),
	}
}

use lightning::chain::keysinterface::KeysInterface as rustKeysInterface;
impl rustKeysInterface for KeysInterface {
	type Signer = crate::lightning::chain::keysinterface::Sign;
	fn get_node_secret(&self) -> bitcoin::secp256k1::key::SecretKey {
		let mut ret = (self.get_node_secret)(self.this_arg);
		ret.into_rust()
	}
	fn get_destination_script(&self) -> bitcoin::blockdata::script::Script {
		let mut ret = (self.get_destination_script)(self.this_arg);
		::bitcoin::blockdata::script::Script::from(ret.into_rust())
	}
	fn get_shutdown_scriptpubkey(&self) -> lightning::ln::script::ShutdownScript {
		let mut ret = (self.get_shutdown_scriptpubkey)(self.this_arg);
		*unsafe { Box::from_raw(ret.take_inner()) }
	}
	fn get_channel_signer(&self, mut inbound: bool, mut channel_value_satoshis: u64) -> crate::lightning::chain::keysinterface::Sign {
		let mut ret = (self.get_channel_signer)(self.this_arg, inbound, channel_value_satoshis);
		ret
	}
	fn get_secure_random_bytes(&self) -> [u8; 32] {
		let mut ret = (self.get_secure_random_bytes)(self.this_arg);
		ret.data
	}
	fn read_chan_signer(&self, mut reader: &[u8]) -> Result<crate::lightning::chain::keysinterface::Sign, lightning::ln::msgs::DecodeError> {
		let mut local_reader = crate::c_types::u8slice::from_slice(reader);
		let mut ret = (self.read_chan_signer)(self.this_arg, local_reader);
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) }).take_inner()) } })};
		local_ret
	}
	fn sign_invoice(&self, mut invoice_preimage: Vec<u8>) -> Result<bitcoin::secp256k1::recovery::RecoverableSignature, ()> {
		let mut local_invoice_preimage = Vec::new(); for mut item in invoice_preimage.drain(..) { local_invoice_preimage.push( { item }); };
		let mut ret = (self.sign_invoice)(self.this_arg, local_invoice_preimage.into());
		let mut local_ret = match ret.result_ok { true => Ok( { (*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.result)) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(<*mut _>::take_ptr(&mut ret.contents.err)) })*/ })};
		local_ret
	}
	fn get_inbound_payment_key_material(&self) -> lightning::chain::keysinterface::KeyMaterial {
		let mut ret = (self.get_inbound_payment_key_material)(self.this_arg);
		::lightning::chain::keysinterface::KeyMaterial(ret.data)
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for KeysInterface {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// Calls the free function if one is set
#[no_mangle]
pub extern "C" fn KeysInterface_free(this_ptr: KeysInterface) { }
impl Drop for KeysInterface {
	fn drop(&mut self) {
		if let Some(f) = self.free {
			f(self.this_arg);
		}
	}
}

use lightning::chain::keysinterface::InMemorySigner as nativeInMemorySignerImport;
pub(crate) type nativeInMemorySigner = nativeInMemorySignerImport;

/// A simple implementation of Sign that just keeps the private keys in memory.
///
/// This implementation performs no policy checks and is insufficient by itself as
/// a secure external signer.
#[must_use]
#[repr(C)]
pub struct InMemorySigner {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeInMemorySigner,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for InMemorySigner {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeInMemorySigner>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the InMemorySigner, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn InMemorySigner_free(this_obj: InMemorySigner) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn InMemorySigner_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeInMemorySigner); }
}
#[allow(unused)]
impl InMemorySigner {
	pub(crate) fn get_native_ref(&self) -> &'static nativeInMemorySigner {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeInMemorySigner {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeInMemorySigner {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = std::ptr::null_mut();
		ret
	}
}
/// Private key of anchor tx
#[no_mangle]
pub extern "C" fn InMemorySigner_get_funding_key(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().funding_key;
	inner_val.as_ref()
}
/// Private key of anchor tx
#[no_mangle]
pub extern "C" fn InMemorySigner_set_funding_key(this_ptr: &mut InMemorySigner, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.funding_key = val.into_rust();
}
/// Holder secret key for blinded revocation pubkey
#[no_mangle]
pub extern "C" fn InMemorySigner_get_revocation_base_key(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().revocation_base_key;
	inner_val.as_ref()
}
/// Holder secret key for blinded revocation pubkey
#[no_mangle]
pub extern "C" fn InMemorySigner_set_revocation_base_key(this_ptr: &mut InMemorySigner, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.revocation_base_key = val.into_rust();
}
/// Holder secret key used for our balance in counterparty-broadcasted commitment transactions
#[no_mangle]
pub extern "C" fn InMemorySigner_get_payment_key(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().payment_key;
	inner_val.as_ref()
}
/// Holder secret key used for our balance in counterparty-broadcasted commitment transactions
#[no_mangle]
pub extern "C" fn InMemorySigner_set_payment_key(this_ptr: &mut InMemorySigner, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.payment_key = val.into_rust();
}
/// Holder secret key used in HTLC tx
#[no_mangle]
pub extern "C" fn InMemorySigner_get_delayed_payment_base_key(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().delayed_payment_base_key;
	inner_val.as_ref()
}
/// Holder secret key used in HTLC tx
#[no_mangle]
pub extern "C" fn InMemorySigner_set_delayed_payment_base_key(this_ptr: &mut InMemorySigner, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.delayed_payment_base_key = val.into_rust();
}
/// Holder htlc secret key used in commitment tx htlc outputs
#[no_mangle]
pub extern "C" fn InMemorySigner_get_htlc_base_key(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().htlc_base_key;
	inner_val.as_ref()
}
/// Holder htlc secret key used in commitment tx htlc outputs
#[no_mangle]
pub extern "C" fn InMemorySigner_set_htlc_base_key(this_ptr: &mut InMemorySigner, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.htlc_base_key = val.into_rust();
}
/// Commitment seed
#[no_mangle]
pub extern "C" fn InMemorySigner_get_commitment_seed(this_ptr: &InMemorySigner) -> *const [u8; 32] {
	let mut inner_val = &mut this_ptr.get_native_mut_ref().commitment_seed;
	inner_val
}
/// Commitment seed
#[no_mangle]
pub extern "C" fn InMemorySigner_set_commitment_seed(this_ptr: &mut InMemorySigner, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *ObjOps::untweak_ptr(this_ptr.inner) }.commitment_seed = val.data;
}
impl Clone for InMemorySigner {
	fn clone(&self) -> Self {
		Self {
			inner: if <*mut nativeInMemorySigner>::is_null(self.inner) { std::ptr::null_mut() } else {
				ObjOps::heap_alloc(unsafe { &*ObjOps::untweak_ptr(self.inner) }.clone()) },
			is_owned: true,
		}
	}
}
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn InMemorySigner_clone_void(this_ptr: *const c_void) -> *mut c_void {
	Box::into_raw(Box::new(unsafe { (*(this_ptr as *mut nativeInMemorySigner)).clone() })) as *mut c_void
}
#[no_mangle]
/// Creates a copy of the InMemorySigner
pub extern "C" fn InMemorySigner_clone(orig: &InMemorySigner) -> InMemorySigner {
	orig.clone()
}
/// Create a new InMemorySigner
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_new(mut funding_key: crate::c_types::SecretKey, mut revocation_base_key: crate::c_types::SecretKey, mut payment_key: crate::c_types::SecretKey, mut delayed_payment_base_key: crate::c_types::SecretKey, mut htlc_base_key: crate::c_types::SecretKey, mut commitment_seed: crate::c_types::ThirtyTwoBytes, mut channel_value_satoshis: u64, mut channel_keys_id: crate::c_types::ThirtyTwoBytes) -> crate::lightning::chain::keysinterface::InMemorySigner {
	let mut ret = lightning::chain::keysinterface::InMemorySigner::new(secp256k1::SECP256K1, funding_key.into_rust(), revocation_base_key.into_rust(), payment_key.into_rust(), delayed_payment_base_key.into_rust(), htlc_base_key.into_rust(), commitment_seed.data, channel_value_satoshis, channel_keys_id.data);
	crate::lightning::chain::keysinterface::InMemorySigner { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

/// Counterparty pubkeys.
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_counterparty_pubkeys(this_arg: &InMemorySigner) -> crate::lightning::ln::chan_utils::ChannelPublicKeys {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.counterparty_pubkeys();
	crate::lightning::ln::chan_utils::ChannelPublicKeys { inner: unsafe { ObjOps::nonnull_ptr_to_inner((ret as *const lightning::ln::chan_utils::ChannelPublicKeys<>) as *mut _) }, is_owned: false }
}

/// The contest_delay value specified by our counterparty and applied on holder-broadcastable
/// transactions, ie the amount of time that we have to wait to recover our funds if we
/// broadcast a transaction.
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_counterparty_selected_contest_delay(this_arg: &InMemorySigner) -> u16 {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.counterparty_selected_contest_delay();
	ret
}

/// The contest_delay value specified by us and applied on transactions broadcastable
/// by our counterparty, ie the amount of time that they have to wait to recover their funds
/// if they broadcast a transaction.
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_holder_selected_contest_delay(this_arg: &InMemorySigner) -> u16 {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.holder_selected_contest_delay();
	ret
}

/// Whether the holder is the initiator
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_is_outbound(this_arg: &InMemorySigner) -> bool {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.is_outbound();
	ret
}

/// Funding outpoint
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_funding_outpoint(this_arg: &InMemorySigner) -> crate::lightning::chain::transaction::OutPoint {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.funding_outpoint();
	crate::lightning::chain::transaction::OutPoint { inner: unsafe { ObjOps::nonnull_ptr_to_inner((ret as *const lightning::chain::transaction::OutPoint<>) as *mut _) }, is_owned: false }
}

/// Obtain a ChannelTransactionParameters for this channel, to be used when verifying or
/// building transactions.
///
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_get_channel_parameters(this_arg: &InMemorySigner) -> crate::lightning::ln::chan_utils::ChannelTransactionParameters {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.get_channel_parameters();
	crate::lightning::ln::chan_utils::ChannelTransactionParameters { inner: unsafe { ObjOps::nonnull_ptr_to_inner((ret as *const lightning::ln::chan_utils::ChannelTransactionParameters<>) as *mut _) }, is_owned: false }
}

/// Whether anchors should be used.
/// Will panic if ready_channel wasn't called.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_opt_anchors(this_arg: &InMemorySigner) -> bool {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.opt_anchors();
	ret
}

/// Sign the single input of spend_tx at index `input_idx` which spends the output
/// described by descriptor, returning the witness stack for the input.
///
/// Returns an Err if the input at input_idx does not exist, has a non-empty script_sig,
/// or is not spending the outpoint described by `descriptor.outpoint`.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_sign_counterparty_payment_input(this_arg: &InMemorySigner, mut spend_tx: crate::c_types::Transaction, mut input_idx: usize, descriptor: &crate::lightning::chain::keysinterface::StaticPaymentOutputDescriptor) -> crate::c_types::derived::CResult_CVec_CVec_u8ZZNoneZ {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.sign_counterparty_payment_input(&spend_tx.into_bitcoin(), input_idx, descriptor.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { let mut local_ret_0 = Vec::new(); for mut item in o.drain(..) { local_ret_0.push( { let mut local_ret_0_0 = Vec::new(); for mut item in item.drain(..) { local_ret_0_0.push( { item }); }; local_ret_0_0.into() }); }; local_ret_0.into() }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}

/// Sign the single input of spend_tx at index `input_idx` which spends the output
/// described by descriptor, returning the witness stack for the input.
///
/// Returns an Err if the input at input_idx does not exist, has a non-empty script_sig,
/// is not spending the outpoint described by `descriptor.outpoint`, or does not have a
/// sequence set to `descriptor.to_self_delay`.
#[must_use]
#[no_mangle]
pub extern "C" fn InMemorySigner_sign_dynamic_p2wsh_input(this_arg: &InMemorySigner, mut spend_tx: crate::c_types::Transaction, mut input_idx: usize, descriptor: &crate::lightning::chain::keysinterface::DelayedPaymentOutputDescriptor) -> crate::c_types::derived::CResult_CVec_CVec_u8ZZNoneZ {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.sign_dynamic_p2wsh_input(&spend_tx.into_bitcoin(), input_idx, descriptor.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { let mut local_ret_0 = Vec::new(); for mut item in o.drain(..) { local_ret_0.push( { let mut local_ret_0_0 = Vec::new(); for mut item in item.drain(..) { local_ret_0_0.push( { item }); }; local_ret_0_0.into() }); }; local_ret_0.into() }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}

impl From<nativeInMemorySigner> for crate::lightning::chain::keysinterface::BaseSign {
	fn from(obj: nativeInMemorySigner) -> Self {
		let mut rust_obj = InMemorySigner { inner: ObjOps::heap_alloc(obj), is_owned: true };
		let mut ret = InMemorySigner_as_BaseSign(&rust_obj);
		// We want to free rust_obj when ret gets drop()'d, not rust_obj, so wipe rust_obj's pointer and set ret's free() fn
		rust_obj.inner = std::ptr::null_mut();
		ret.free = Some(InMemorySigner_free_void);
		ret
	}
}
/// Constructs a new BaseSign which calls the relevant methods on this_arg.
/// This copies the `inner` pointer in this_arg and thus the returned BaseSign must be freed before this_arg is
#[no_mangle]
pub extern "C" fn InMemorySigner_as_BaseSign(this_arg: &InMemorySigner) -> crate::lightning::chain::keysinterface::BaseSign {
	crate::lightning::chain::keysinterface::BaseSign {
		this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
		free: None,
		get_per_commitment_point: InMemorySigner_BaseSign_get_per_commitment_point,
		release_commitment_secret: InMemorySigner_BaseSign_release_commitment_secret,
		validate_holder_commitment: InMemorySigner_BaseSign_validate_holder_commitment,

		pubkeys: crate::lightning::ln::chan_utils::ChannelPublicKeys { inner: std::ptr::null_mut(), is_owned: true },
		set_pubkeys: Some(InMemorySigner_BaseSign_set_pubkeys),
		channel_keys_id: InMemorySigner_BaseSign_channel_keys_id,
		sign_counterparty_commitment: InMemorySigner_BaseSign_sign_counterparty_commitment,
		validate_counterparty_revocation: InMemorySigner_BaseSign_validate_counterparty_revocation,
		sign_holder_commitment_and_htlcs: InMemorySigner_BaseSign_sign_holder_commitment_and_htlcs,
		sign_justice_revoked_output: InMemorySigner_BaseSign_sign_justice_revoked_output,
		sign_justice_revoked_htlc: InMemorySigner_BaseSign_sign_justice_revoked_htlc,
		sign_counterparty_htlc_transaction: InMemorySigner_BaseSign_sign_counterparty_htlc_transaction,
		sign_closing_transaction: InMemorySigner_BaseSign_sign_closing_transaction,
		sign_channel_announcement: InMemorySigner_BaseSign_sign_channel_announcement,
		ready_channel: InMemorySigner_BaseSign_ready_channel,
	}
}

#[must_use]
extern "C" fn InMemorySigner_BaseSign_get_per_commitment_point(this_arg: *const c_void, mut idx: u64) -> crate::c_types::PublicKey {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::get_per_commitment_point(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, idx, secp256k1::SECP256K1);
	crate::c_types::PublicKey::from_rust(&ret)
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_release_commitment_secret(this_arg: *const c_void, mut idx: u64) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::release_commitment_secret(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, idx);
	crate::c_types::ThirtyTwoBytes { data: ret }
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_validate_holder_commitment(this_arg: *const c_void, _holder_tx: &crate::lightning::ln::chan_utils::HolderCommitmentTransaction) -> crate::c_types::derived::CResult_NoneNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::validate_holder_commitment(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, _holder_tx.get_native_ref());
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { () /*o*/ }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_pubkeys(this_arg: *const c_void) -> crate::lightning::ln::chan_utils::ChannelPublicKeys {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::pubkeys(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, );
	crate::lightning::ln::chan_utils::ChannelPublicKeys { inner: unsafe { ObjOps::nonnull_ptr_to_inner((ret as *const lightning::ln::chan_utils::ChannelPublicKeys<>) as *mut _) }, is_owned: false }
}
extern "C" fn InMemorySigner_BaseSign_set_pubkeys(trait_self_arg: &BaseSign) {
	// This is a bit race-y in the general case, but for our specific use-cases today, we're safe
	// Specifically, we must ensure that the first time we're called it can never be in parallel
	if trait_self_arg.pubkeys.inner.is_null() {
		unsafe { &mut *(trait_self_arg as *const BaseSign  as *mut BaseSign) }.pubkeys = InMemorySigner_BaseSign_pubkeys(trait_self_arg.this_arg);
	}
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_channel_keys_id(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::channel_keys_id(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, );
	crate::c_types::ThirtyTwoBytes { data: ret }
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_counterparty_commitment(this_arg: *const c_void, commitment_tx: &crate::lightning::ln::chan_utils::CommitmentTransaction) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_counterparty_commitment(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, commitment_tx.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = o; let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.drain(..) { local_orig_ret_0_1.push( { crate::c_types::Signature::from_rust(&item) }); }; let mut local_ret_0 = (crate::c_types::Signature::from_rust(&orig_ret_0_0), local_orig_ret_0_1.into()).into(); local_ret_0 }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_validate_counterparty_revocation(this_arg: *const c_void, mut _idx: u64, _secret: *const [u8; 32]) -> crate::c_types::derived::CResult_NoneNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::validate_counterparty_revocation(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, _idx, &::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *_secret}[..]).unwrap());
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { () /*o*/ }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_holder_commitment_and_htlcs(this_arg: *const c_void, commitment_tx: &crate::lightning::ln::chan_utils::HolderCommitmentTransaction) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_holder_commitment_and_htlcs(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, commitment_tx.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = o; let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.drain(..) { local_orig_ret_0_1.push( { crate::c_types::Signature::from_rust(&item) }); }; let mut local_ret_0 = (crate::c_types::Signature::from_rust(&orig_ret_0_0), local_orig_ret_0_1.into()).into(); local_ret_0 }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_justice_revoked_output(this_arg: *const c_void, mut justice_tx: crate::c_types::Transaction, mut input: usize, mut amount: u64, per_commitment_key: *const [u8; 32]) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_justice_revoked_output(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, &justice_tx.into_bitcoin(), input, amount, &::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *per_commitment_key}[..]).unwrap(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Signature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_justice_revoked_htlc(this_arg: *const c_void, mut justice_tx: crate::c_types::Transaction, mut input: usize, mut amount: u64, per_commitment_key: *const [u8; 32], htlc: &crate::lightning::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_justice_revoked_htlc(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, &justice_tx.into_bitcoin(), input, amount, &::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *per_commitment_key}[..]).unwrap(), htlc.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Signature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_counterparty_htlc_transaction(this_arg: *const c_void, mut htlc_tx: crate::c_types::Transaction, mut input: usize, mut amount: u64, mut per_commitment_point: crate::c_types::PublicKey, htlc: &crate::lightning::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_counterparty_htlc_transaction(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, &htlc_tx.into_bitcoin(), input, amount, &per_commitment_point.into_rust(), htlc.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Signature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_closing_transaction(this_arg: *const c_void, closing_tx: &crate::lightning::ln::chan_utils::ClosingTransaction) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_closing_transaction(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, closing_tx.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Signature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
#[must_use]
extern "C" fn InMemorySigner_BaseSign_sign_channel_announcement(this_arg: *const c_void, msg: &crate::lightning::ln::msgs::UnsignedChannelAnnouncement) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = <nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::sign_channel_announcement(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, msg.get_native_ref(), secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Signature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}
extern "C" fn InMemorySigner_BaseSign_ready_channel(this_arg: *mut c_void, channel_parameters: &crate::lightning::ln::chan_utils::ChannelTransactionParameters) {
	<nativeInMemorySigner as lightning::chain::keysinterface::BaseSign<>>::ready_channel(unsafe { &mut *(this_arg as *mut nativeInMemorySigner) }, channel_parameters.get_native_ref())
}

impl From<nativeInMemorySigner> for crate::lightning::chain::keysinterface::Sign {
	fn from(obj: nativeInMemorySigner) -> Self {
		let mut rust_obj = InMemorySigner { inner: ObjOps::heap_alloc(obj), is_owned: true };
		let mut ret = InMemorySigner_as_Sign(&rust_obj);
		// We want to free rust_obj when ret gets drop()'d, not rust_obj, so wipe rust_obj's pointer and set ret's free() fn
		rust_obj.inner = std::ptr::null_mut();
		ret.free = Some(InMemorySigner_free_void);
		ret
	}
}
/// Constructs a new Sign which calls the relevant methods on this_arg.
/// This copies the `inner` pointer in this_arg and thus the returned Sign must be freed before this_arg is
#[no_mangle]
pub extern "C" fn InMemorySigner_as_Sign(this_arg: &InMemorySigner) -> crate::lightning::chain::keysinterface::Sign {
	crate::lightning::chain::keysinterface::Sign {
		this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
		free: None,
		BaseSign: crate::lightning::chain::keysinterface::BaseSign {
			this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
			free: None,
			get_per_commitment_point: InMemorySigner_BaseSign_get_per_commitment_point,
			release_commitment_secret: InMemorySigner_BaseSign_release_commitment_secret,
			validate_holder_commitment: InMemorySigner_BaseSign_validate_holder_commitment,

			pubkeys: crate::lightning::ln::chan_utils::ChannelPublicKeys { inner: std::ptr::null_mut(), is_owned: true },
			set_pubkeys: Some(InMemorySigner_BaseSign_set_pubkeys),
			channel_keys_id: InMemorySigner_BaseSign_channel_keys_id,
			sign_counterparty_commitment: InMemorySigner_BaseSign_sign_counterparty_commitment,
			validate_counterparty_revocation: InMemorySigner_BaseSign_validate_counterparty_revocation,
			sign_holder_commitment_and_htlcs: InMemorySigner_BaseSign_sign_holder_commitment_and_htlcs,
			sign_justice_revoked_output: InMemorySigner_BaseSign_sign_justice_revoked_output,
			sign_justice_revoked_htlc: InMemorySigner_BaseSign_sign_justice_revoked_htlc,
			sign_counterparty_htlc_transaction: InMemorySigner_BaseSign_sign_counterparty_htlc_transaction,
			sign_closing_transaction: InMemorySigner_BaseSign_sign_closing_transaction,
			sign_channel_announcement: InMemorySigner_BaseSign_sign_channel_announcement,
			ready_channel: InMemorySigner_BaseSign_ready_channel,
		},
		write: InMemorySigner_write_void,
		cloned: Some(Sign_InMemorySigner_cloned),
	}
}

extern "C" fn Sign_InMemorySigner_cloned(new_obj: &mut crate::lightning::chain::keysinterface::Sign) {
	new_obj.this_arg = InMemorySigner_clone_void(new_obj.this_arg);
	new_obj.free = Some(InMemorySigner_free_void);
	new_obj.BaseSign.this_arg = new_obj.this_arg;
	new_obj.BaseSign.free = None;
}

#[no_mangle]
/// Serialize the InMemorySigner object into a byte array which can be read by InMemorySigner_read
pub extern "C" fn InMemorySigner_write(obj: &InMemorySigner) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn InMemorySigner_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeInMemorySigner) })
}
#[no_mangle]
/// Read a InMemorySigner from a byte array, created by InMemorySigner_write
pub extern "C" fn InMemorySigner_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_InMemorySignerDecodeErrorZ {
	let res: Result<lightning::chain::keysinterface::InMemorySigner, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::chain::keysinterface::InMemorySigner { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_res
}

use lightning::chain::keysinterface::KeysManager as nativeKeysManagerImport;
pub(crate) type nativeKeysManager = nativeKeysManagerImport;

/// Simple KeysInterface implementor that takes a 32-byte seed for use as a BIP 32 extended key
/// and derives keys from that.
///
/// Your node_id is seed/0'
/// ChannelMonitor closes may use seed/1'
/// Cooperative closes may use seed/2'
/// The two close keys may be needed to claim on-chain funds!
#[must_use]
#[repr(C)]
pub struct KeysManager {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeKeysManager,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for KeysManager {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeKeysManager>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the KeysManager, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn KeysManager_free(this_obj: KeysManager) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn KeysManager_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeKeysManager); }
}
#[allow(unused)]
impl KeysManager {
	pub(crate) fn get_native_ref(&self) -> &'static nativeKeysManager {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeKeysManager {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeKeysManager {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = std::ptr::null_mut();
		ret
	}
}
/// Constructs a KeysManager from a 32-byte seed. If the seed is in some way biased (eg your
/// CSRNG is busted) this may panic (but more importantly, you will possibly lose funds).
/// starting_time isn't strictly required to actually be a time, but it must absolutely,
/// without a doubt, be unique to this instance. ie if you start multiple times with the same
/// seed, starting_time must be unique to each run. Thus, the easiest way to achieve this is to
/// simply use the current time (with very high precision).
///
/// The seed MUST be backed up safely prior to use so that the keys can be re-created, however,
/// obviously, starting_time should be unique every time you reload the library - it is only
/// used to generate new ephemeral key data (which will be stored by the individual channel if
/// necessary).
///
/// Note that the seed is required to recover certain on-chain funds independent of
/// ChannelMonitor data, though a current copy of ChannelMonitor data is also required for any
/// channel, and some on-chain during-closing funds.
///
/// Note that until the 0.1 release there is no guarantee of backward compatibility between
/// versions. Once the library is more fully supported, the docs will be updated to include a
/// detailed description of the guarantee.
#[must_use]
#[no_mangle]
pub extern "C" fn KeysManager_new(seed: *const [u8; 32], mut starting_time_secs: u64, mut starting_time_nanos: u32) -> KeysManager {
	let mut ret = lightning::chain::keysinterface::KeysManager::new(unsafe { &*seed}, starting_time_secs, starting_time_nanos);
	KeysManager { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

/// Derive an old Sign containing per-channel secrets based on a key derivation parameters.
///
/// Key derivation parameters are accessible through a per-channel secrets
/// Sign::channel_keys_id and is provided inside DynamicOuputP2WSH in case of
/// onchain output detection for which a corresponding delayed_payment_key must be derived.
#[must_use]
#[no_mangle]
pub extern "C" fn KeysManager_derive_channel_keys(this_arg: &KeysManager, mut channel_value_satoshis: u64, params: *const [u8; 32]) -> crate::lightning::chain::keysinterface::InMemorySigner {
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.derive_channel_keys(channel_value_satoshis, unsafe { &*params});
	crate::lightning::chain::keysinterface::InMemorySigner { inner: ObjOps::heap_alloc(ret), is_owned: true }
}

/// Creates a Transaction which spends the given descriptors to the given outputs, plus an
/// output to the given change destination (if sufficient change value remains). The
/// transaction will have a feerate, at least, of the given value.
///
/// Returns `Err(())` if the output value is greater than the input value minus required fee or
/// if a descriptor was duplicated.
///
/// We do not enforce that outputs meet the dust limit or that any output scripts are standard.
///
/// May panic if the `SpendableOutputDescriptor`s were not generated by Channels which used
/// this KeysManager or one of the `InMemorySigner` created by this KeysManager.
#[must_use]
#[no_mangle]
pub extern "C" fn KeysManager_spend_spendable_outputs(this_arg: &KeysManager, mut descriptors: crate::c_types::derived::CVec_SpendableOutputDescriptorZ, mut outputs: crate::c_types::derived::CVec_TxOutZ, mut change_destination_script: crate::c_types::derived::CVec_u8Z, mut feerate_sat_per_1000_weight: u32) -> crate::c_types::derived::CResult_TransactionNoneZ {
	let mut local_descriptors = Vec::new(); for mut item in descriptors.into_rust().drain(..) { local_descriptors.push( { item.into_native() }); };
	let mut local_outputs = Vec::new(); for mut item in outputs.into_rust().drain(..) { local_outputs.push( { item.into_rust() }); };
	let mut ret = unsafe { &*ObjOps::untweak_ptr(this_arg.inner) }.spend_spendable_outputs(&local_descriptors.iter().collect::<Vec<_>>()[..], local_outputs, ::bitcoin::blockdata::script::Script::from(change_destination_script.into_rust()), feerate_sat_per_1000_weight, secp256k1::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::Transaction::from_bitcoin(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}

impl From<nativeKeysManager> for crate::lightning::chain::keysinterface::KeysInterface {
	fn from(obj: nativeKeysManager) -> Self {
		let mut rust_obj = KeysManager { inner: ObjOps::heap_alloc(obj), is_owned: true };
		let mut ret = KeysManager_as_KeysInterface(&rust_obj);
		// We want to free rust_obj when ret gets drop()'d, not rust_obj, so wipe rust_obj's pointer and set ret's free() fn
		rust_obj.inner = std::ptr::null_mut();
		ret.free = Some(KeysManager_free_void);
		ret
	}
}
/// Constructs a new KeysInterface which calls the relevant methods on this_arg.
/// This copies the `inner` pointer in this_arg and thus the returned KeysInterface must be freed before this_arg is
#[no_mangle]
pub extern "C" fn KeysManager_as_KeysInterface(this_arg: &KeysManager) -> crate::lightning::chain::keysinterface::KeysInterface {
	crate::lightning::chain::keysinterface::KeysInterface {
		this_arg: unsafe { ObjOps::untweak_ptr((*this_arg).inner) as *mut c_void },
		free: None,
		get_node_secret: KeysManager_KeysInterface_get_node_secret,
		get_destination_script: KeysManager_KeysInterface_get_destination_script,
		get_shutdown_scriptpubkey: KeysManager_KeysInterface_get_shutdown_scriptpubkey,
		get_channel_signer: KeysManager_KeysInterface_get_channel_signer,
		get_secure_random_bytes: KeysManager_KeysInterface_get_secure_random_bytes,
		read_chan_signer: KeysManager_KeysInterface_read_chan_signer,
		sign_invoice: KeysManager_KeysInterface_sign_invoice,
		get_inbound_payment_key_material: KeysManager_KeysInterface_get_inbound_payment_key_material,
	}
}

#[must_use]
extern "C" fn KeysManager_KeysInterface_get_node_secret(this_arg: *const c_void) -> crate::c_types::SecretKey {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_node_secret(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, );
	crate::c_types::SecretKey::from_rust(ret)
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_inbound_payment_key_material(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_inbound_payment_key_material(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, );
	crate::c_types::ThirtyTwoBytes { data: ret.0 }
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_destination_script(this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_destination_script(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, );
	ret.into_bytes().into()
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_shutdown_scriptpubkey(this_arg: *const c_void) -> crate::lightning::ln::script::ShutdownScript {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_shutdown_scriptpubkey(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, );
	crate::lightning::ln::script::ShutdownScript { inner: ObjOps::heap_alloc(ret), is_owned: true }
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_channel_signer(this_arg: *const c_void, mut _inbound: bool, mut channel_value_satoshis: u64) -> crate::lightning::chain::keysinterface::Sign {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_channel_signer(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, _inbound, channel_value_satoshis);
	Into::into(ret)
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_secure_random_bytes(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::get_secure_random_bytes(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, );
	crate::c_types::ThirtyTwoBytes { data: ret }
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_read_chan_signer(this_arg: *const c_void, mut reader: crate::c_types::u8slice) -> crate::c_types::derived::CResult_SignDecodeErrorZ {
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::read_chan_signer(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, reader.to_slice());
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { Into::into(o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError { inner: ObjOps::heap_alloc(e), is_owned: true } }).into() };
	local_ret
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_sign_invoice(this_arg: *const c_void, mut invoice_preimage: crate::c_types::derived::CVec_u8Z) -> crate::c_types::derived::CResult_RecoverableSignatureNoneZ {
	let mut local_invoice_preimage = Vec::new(); for mut item in invoice_preimage.into_rust().drain(..) { local_invoice_preimage.push( { item }); };
	let mut ret = <nativeKeysManager as lightning::chain::keysinterface::KeysInterface<>>::sign_invoice(unsafe { &mut *(this_arg as *mut nativeKeysManager) }, local_invoice_preimage);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::c_types::RecoverableSignature::from_rust(&o) }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}

