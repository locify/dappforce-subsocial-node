#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_module,
    dispatch::{DispatchError, DispatchResult}, ensure, traits::Get,
};
use sp_runtime::RuntimeDebug;
use sp_std::{
    collections::btree_set::BTreeSet,
    prelude::*,
};

pub type SpaceId = u64;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct WhoAndWhen<T: Trait> {
    pub account: T::AccountId,
    pub block: T::BlockNumber,
    pub time: T::Moment,
}

impl<T: Trait> WhoAndWhen<T> {
    pub fn new(account: T::AccountId) -> Self {
        WhoAndWhen {
            account,
            block: <system::Module<T>>::block_number(),
            time: <pallet_timestamp::Module<T>>::now(),
        }
    }
}

#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum User<AccountId> {
    Account(AccountId),
    Space(SpaceId),
}

pub trait Trait: system::Trait
    + pallet_timestamp::Trait
{
    /// A valid length of IPFS CID in bytes.
    type IpfsHashLen: Get<u32>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// A valid length of IPFS CID in bytes.
        const IpfsHashLen: u32 = T::IpfsHashLen::get();
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// IPFS CID is invalid.
        InvalidIpfsCid,
    }
}

fn num_bits<P>() -> usize {
    sp_std::mem::size_of::<P>() * 8
}

/// Returns `None` for `x == 0`.
pub fn log_2(x: u32) -> Option<u32> {
    if x > 0 {
        Some(
            num_bits::<u32>() as u32
            - x.leading_zeros()
            - 1
        )
    } else { None }
}

/// An example of a valid handle: `good_handle`.
pub fn is_valid_handle_char(c: u8) -> bool {
    match c {
        b'0'..=b'9' | b'a'..=b'z' | b'_' => true,
        _ => false,
    }
}

pub fn vec_remove_on<F: PartialEq>(vector: &mut Vec<F>, element: F) {
    if let Some(index) = vector.iter().position(|x| *x == element) {
        vector.swap_remove(index);
    }
}

impl<T: Trait> Module<T> {

    pub fn is_ipfs_hash_valid(ipfs_hash: Vec<u8>) -> DispatchResult {
        ensure!(ipfs_hash.len() == T::IpfsHashLen::get() as usize, Error::<T>::InvalidIpfsCid);
        Ok(())
    }

    pub fn convert_users_vec_to_btree_set(
        users_vec: Vec<User<T::AccountId>>
    ) -> Result<BTreeSet<User<T::AccountId>>, DispatchError> {
        let mut users_set: BTreeSet<User<T::AccountId>> = BTreeSet::new();

        for user in users_vec.iter() {
            users_set.insert(user.clone());
        }

        Ok(users_set)
    }
}