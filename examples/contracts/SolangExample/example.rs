#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

///example.sol
#[brush::contract]
pub mod example {
    use brush::traits::{
        AccountId,
        AcountIdExt,
        ZERO_ADDRESS,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    ///Constants
    pub const bad_state: State = State.Zombie;
    pub const first_pid: i32 = 1;

    ///Process state
    pub enum State {
        Running,
        Sleeping,
        Waiting,
        Stopped,
        Zombie,
        StateCount,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct example {
        ///Variables in contract storage
        state: State,
        pid: i32,
        reaped: u32,
    }

    impl example {
        ///Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                // Set contract storage
                instance.pid = pid;
                self.reaped = 3;
            })
        }

        ///Reading but not writing contract storage means function
        ///can be declared view
        #[ink(message)]
        pub fn is_zombie_reaper(&self) -> Result<bool, Error> {
            // must be pid 1 and not zombie ourselves *
            return Ok((pid == self.first_pid && self.state != state.zombie))
        }

        ///Returning a constant does not access storage at all, so
        ///function can be declared pure
        #[ink(message)]
        pub fn systemd_pid(&self) -> Result<u32, Error> {
            // Note that cast is required to change sign from
            // int32 to uint32
            return Ok(self.first_pid as u32)
        }

        ///Convert celcius to fahrenheit
        #[ink(message)]
        pub fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error> {
            let fahrenheit: i32 = celcius * 9 / 5 + 32;
            return Ok(fahrenheit)
        }

        ///Convert fahrenheit to celcius
        #[ink(message)]
        pub fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error> {
            return Ok((fahrenheit - 32) * 5 / 9)
        }

        ///is this number a power-of-two
        #[ink(message)]
        pub fn is_power_of_2(&self, n: u128) -> Result<bool, Error> {
            return Ok(n != 0 && (n & (n - 1)) == 0)
        }

        ///calculate the population count (number of set bits) using Brian Kerningham's way
        #[ink(message)]
        pub fn population_count(&self, n: u128) -> Result<u128, Error> {
            // Sol2Ink Not Implemented yet: for(count = 0 n != 0 count++){
            n &= (n - 1);
            // Sol2Ink Not Implemented yet End Block here
        }

        ///calculate the power of base to exp
        #[ink(message)]
        pub fn power(&self, base: u128, exp: u128) -> Result<u128, Error> {
            return Ok(base.pow(exp as u32))
        }

        ///returns true if the address is 0
        #[ink(message)]
        pub fn is_address_zero(&self, a: AccountId) -> Result<bool, Error> {
            return Ok(a.is_zero())
        }

        ///reverse the bytes in an array of 8 (endian swap)
        #[ink(message)]
        pub fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error> {
            out = ((input << 56) & self._hex("ff_00_0000_0000_0000"))?
                | ((input << 40) & self._hex("00_ff_0000_0000_0000"))?
                | ((input << 24) & self._hex("0000_ff_00_0000_0000"))?
                | ((input << 8) & self._hex("0000_00_ff_0000_0000"))?
                | ((input >> 8) & self._hex("0000_0000_ff_00_0000"))?
                | ((input >> 24) & self._hex("0000_0000_00_ff_0000"))?
                | ((input >> 40) & self._hex("0000_0000_0000_ff_00"))?
                | ((input >> 56) & self._hex("0000_0000_0000_00_ff"))?;
        }

    }
}
