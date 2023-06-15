// lib.rs

use frame_support::{decl_module, decl_storage, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Ledger {
        LedgerData get(fn ledger_data): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn store_data(origin, key: Vec<u8>, value: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            // Ensuring the data does not already exist in the ledger
            ensure!(!LedgerData::contains_key(&key), "Data already exists in the ledger");

            // Storing the data in the ledger
            LedgerData::insert(&key, &value);

            // Emit an event to signify the data storage
            Self::deposit_event(RawEvent::DataStored(sender, key, value));

            Ok(())
        }
    }
}
