#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

#[openbrush::contract]
pub mod flipper {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }


    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {
        pub value: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct flipper {
        #[storage_field]
        data: Data,
    }

    impl flipper {
        ///Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initvalue: bool) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.value = initvalue;
            })
        }

        ///A message that can be called on instantiated contracts.
        ///This one flips the value of the stored `bool` from `true`
        ///to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> Result<(), Error> {
            self.data.value = !value;
            Ok(())
        }

        ///Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Result<bool, Error> {
            return Ok(self.data.value)
        }

    }
}
