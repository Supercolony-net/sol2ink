#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

#[openbrush::contract]
pub mod primitives {
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


    pub enum Oper {
        Add,
        Sub,
        Mul,
        Div,
        Modulo,
        Pow,
        Shl,
        Shr,
        Or,
        And,
        Xor,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {}

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct primitives {
        #[storage_field]
        data: Data,
    }

    impl primitives {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn is_mul(&self, op: oper) -> Result<bool, Error> {
            return Ok(op == oper.mul)
        }

        #[ink(message)]
        pub fn return_div(&self) -> Result<oper, Error> {
            return Ok(oper.div)
        }

        #[ink(message)]
        pub fn op_i_64(&self, op: oper, a: i64, b: i64) -> Result<i64, Error> {
            if op == oper.add {
                return Ok(a + b)
            } else if op == oper.sub {
                return Ok(a - b)
            } else if op == oper.mul {
                return Ok(a * b)
            } else if op == oper.div {
                return Ok(a / b)
            } else if op == oper.modulo {
                return Ok(a % b)
            } else if op == oper.shl {
                return Ok(a << b)
            } else if op == oper.shr {
                return Ok(a >> b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_u_64(&self, op: oper, a: u64, b: u64) -> Result<u64, Error> {
            if op == oper.add {
                return Ok(a + b)
            } else if op == oper.sub {
                return Ok(a - b)
            } else if op == oper.mul {
                return Ok(a * b)
            } else if op == oper.div {
                return Ok(a / b)
            } else if op == oper.modulo {
                return Ok(a % b)
            } else if op == oper.pow {
                return Ok(a.pow(b as u32))
            } else if op == oper.shl {
                return Ok(a << b)
            } else if op == oper.shr {
                return Ok(a >> b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_u_256(&self, op: oper, a: u128, b: u128) -> Result<u128, Error> {
            if op == oper.add {
                return Ok(a + b)
            } else if op == oper.sub {
                return Ok(a - b)
            } else if op == oper.mul {
                return Ok(a * b)
            } else if op == oper.div {
                return Ok(a / b)
            } else if op == oper.modulo {
                return Ok(a % b)
            } else if op == oper.pow {
                return Ok(a.pow((b as u128) as u32))
            } else if op == oper.shl {
                return Ok(a << b)
            } else if op == oper.shr {
                return Ok(a >> b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_i_256(&self, op: oper, a: i128, b: i128) -> Result<i128, Error> {
            if op == oper.add {
                return Ok(a + b)
            } else if op == oper.sub {
                return Ok(a - b)
            } else if op == oper.mul {
                return Ok(a * b)
            } else if op == oper.div {
                return Ok(a / b)
            } else if op == oper.modulo {
                return Ok(a % b)
            } else if op == oper.shl {
                return Ok(a << b)
            } else if op == oper.shr {
                return Ok(a >> b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn return_u_8_6(&self) -> Result<[u8; 6], Error> {
            return Ok("ABCDEF")
        }

        #[ink(message)]
        pub fn op_u_8_5_shift(&self, op: oper, a: [u8; 5], r: u64) -> Result<[u8; 5], Error> {
            if op == oper.shl {
                return Ok(a << r)
            } else if op == oper.shr {
                return Ok(a >> r)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_u_8_5(&self, op: oper, a: [u8; 5], b: [u8; 5]) -> Result<[u8; 5], Error> {
            if op == oper.or {
                return Ok(a | b)
            } else if op == oper.and {
                return Ok(a & b)
            } else if op == oper.xor {
                return Ok(a ^ b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_u_8_14_shift(&self, op: oper, a: [u8; 14], r: u64) -> Result<[u8; 14], Error> {
            if op == oper.shl {
                return Ok(a << r)
            } else if op == oper.shr {
                return Ok(a >> r)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn op_u_8_14(&self, op: oper, a: [u8; 14], b: [u8; 14]) -> Result<[u8; 14], Error> {
            if op == oper.or {
                return Ok(a | b)
            } else if op == oper.and {
                return Ok(a & b)
            } else if op == oper.xor {
                return Ok(a ^ b)
            } else {
                revert()?;
            }
        }

        #[ink(message)]
        pub fn address_passthrough(&self, a: AccountId) -> Result<AccountId, Error> {
            return Ok(a)
        }

    }
}
