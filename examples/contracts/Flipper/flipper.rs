#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

#[brush::contract]
pub mod flipper {

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }


    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct flipper {
        value: bool,
    }

    impl flipper {
        ///Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initvalue: bool) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.value = initvalue;
            })
        }

        ///A message that can be called on instantiated contracts.
        ///This one flips the value of the stored `bool` from `true`
        ///to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> Result<(), Error> {
            self.value = !value;
            Ok(())
        }

        ///Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Result<bool, Error> {
            return Ok(self.value)
        }

    }
}
