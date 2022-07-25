#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

///example.sol
#[brush::contract]
pub mod example {

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
            // Sol2Ink Not Implemented yet: /* must be pid 1 and not zombie ourselves */ return (pid == first_pid && state != State.Zombie)
        }

    }
}
