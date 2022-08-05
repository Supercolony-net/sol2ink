#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

///example.sol
#[openbrush::contract]
pub mod example {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountIdExt,
        Storage,
        ZERO_ADDRESS,
    };
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    ///Constants
    pub const BAD_STATE: State = state.zombie;
    pub const FIRST_PID: i32 = 1;

    ///Process state
    pub enum State {
        Running,
        Sleeping,
        Waiting,
        Stopped,
        Zombie,
        StateCount,
    }

    ///cards
    pub enum Suit {
        Club,
        Diamonds,
        Hearts,
        Spades,
    }

    pub enum Value {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct card {
        v: value,
        s: suit,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {
        ///Variables in contract storage
        pub state: State,
        pub pid: i32,
        pub reaped: u32,
        pub card_1: card,
        pub card_2: card,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct example {
        #[storage_field]
        data: Data,
    }

    impl example {
        ///Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                // Set contract storage
                instance.data.pid = pid;
                self.reaped = 3;
                self.card_1 = card(value.two, suit.club)?;
                self.card_2 = Card {
                    s: suit.club,
                    v: value.two,
                };
            })
        }

        ///Reading but not writing contract storage means function
        ///can be declared view
        #[ink(message)]
        pub fn is_zombie_reaper(&self) -> Result<bool, Error> {
            // must be pid 1 and not zombie ourselves *
            return Ok((self.data.pid == FIRST_PID && self.data.state != state.zombie))
        }

        ///Returning a constant does not access storage at all, so
        ///function can be declared pure
        #[ink(message)]
        pub fn systemd_pid(&self) -> Result<u32, Error> {
            // Note that cast is required to change sign from
            // int32 to uint32
            return Ok((FIRST_PID as u32))
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
            let mut count = Default::default();
            count = 0;
            while n != 0 {
                n &= (n - 1);
                count += 1;
            }
            Ok(count)
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
            let mut out = Default::default();
            out = ((input << 56) & &hex::decode("ff00_0000_0000_0000"))
                | ((input << 40) & &hex::decode("00ff_0000_0000_0000"))
                | ((input << 24) & &hex::decode("0000_ff00_0000_0000"))
                | ((input << 8) & &hex::decode("0000_00ff_0000_0000"))
                | ((input >> 8) & &hex::decode("0000_0000_ff00_0000"))
                | ((input >> 24) & &hex::decode("0000_0000_00ff_0000"))
                | ((input >> 40) & &hex::decode("0000_0000_0000_ff00"))
                | ((input >> 56) & &hex::decode("0000_0000_0000_00ff"));
            Ok(out)
        }

        ///This mocks a pid state
        fn _get_pid_state(&self, pid: u64) -> Result<State, Error> {
            let n: u64 = 8;
            let i: u16 = 1;
            while i < 10 {
                if (i % 3) == 0 {
                    n *= pid / (i as u64);
                } else {
                    n /= 3;
                }
                i += 1;
            }
            return Ok(state(n % (state.state_count as u64))?)
        }

        ///Overloaded function with different return value!
        fn _get_pid_state(&self) -> Result<u32, Error> {
            return Ok(self.data.reaped)
        }

        #[ink(message)]
        pub fn reap_processes(&mut self) -> Result<(), Error> {
            let n: u32 = 0;
            while n < 100 {
                if self._get_pid_state(n)? == state.zombie {
                    // reap!
                    self.data.reaped += 1;
                }
                n += 1;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn run_queue(&self) -> Result<u16, Error> {
            let count: u16 = 0;
            // no initializer means its 0.
            let n: u32 = 0;
            loop {
                if self._get_pid_state(n)? == state.waiting {
                    count += 1;
                }
                if n < 1000 {
                    break
                }
            }
            return Ok(count)
        }

        ///This function does a lot of copying
        #[ink(message)]
        pub fn set_card_1(&mut self, c: card) -> Result<card, Error> {
            let mut previous = Default::default();
            previous = self.data.card_1;
            self.data.card_1 = c;
            Ok(previous)
        }

        ///return the ace of spades
        #[ink(message)]
        pub fn ace_of_spaces(&self) -> Result<card, Error> {
            return Ok(Card {
                s: suit.spades,
                v: value.ace,
            })
        }

        ///score card
        #[ink(message)]
        pub fn score_card(&self, c: card) -> Result<u32, Error> {
            let mut score = Default::default();
            if c.s == suit.hearts {
                if c.v == value.ace {
                    score = 14;
                }
                if c.v == value.king {
                    score = 13;
                }
                if c.v == value.queen {
                    score = 12;
                }
                if c.v == value.jack {
                    score = 11;
                }
            }
            // all others score 0
            Ok(score)
        }

    }
}
