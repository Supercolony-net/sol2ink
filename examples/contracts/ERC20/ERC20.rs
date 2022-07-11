#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.6.0) (token/ERC20/ERC20.sol)
/// @dev Implementation of the {IERC20} interface.
/// This implementation is agnostic to the way tokens are created. This means
/// that a supply mechanism has to be added in a derived contract using {_mint}.
/// For a generic mechanism see {ERC20PresetMinterPauser}.
/// TIP: For a detailed writeup see our guide
/// https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How
/// to implement supply mechanisms].
/// We have followed general OpenZeppelin Contracts guidelines: functions revert
/// instead returning `false` on failure. This behavior is nonetheless
/// conventional and does not conflict with the expectations of ERC20
/// applications.
/// Additionally, an {Approval} event is emitted on calls to {transferFrom}.
/// This allows applications to reconstruct the allowance for all accounts just
/// by listening to said events. Other implementations of the EIP may not emit
/// these events, as it isn't required by the specification.
/// Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
/// functions have been added to mitigate the well-known issues around setting
/// allowances. See {IERC20-approve}.
#[brush::contract]
pub mod erc_20 {
    use brush::traits::{
        AccountId,
        AcountIdExt,
        ZERO_ADDRESS,
    };
    use ink::prelude::string::String;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::Mapping;

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    /// @dev Emitted when `value` tokens are moved from one account (`from`) to
    /// another (`to`).
    /// Note that `value` may be zero.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    /// @dev Emitted when the allowance of a `spender` for an `owner` is set by
    /// a call to {approve}. `value` is the new allowance.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    /// This enum is added just to test enum parsing
    pub enum Enum {
        FIRST,
        SECOND,
    }

    /// This struct is added just to test struct parsing
    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Struct {
        field_1: u128,
        field_2: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct ERC20 {
        balances: Mapping<AccountId, u128>,
        allowances: Mapping<(AccountId, AccountId), u128>,
        total_supply: u128,
        name: String,
        symbol: String,
    }

    impl ERC20 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.name = name;
                instance.symbol = symbol;
            })
        }

        #[ink(message)]
        pub fn name(&self) -> Result<String, Error> {
            return Ok(self.name)
        }

        #[ink(message)]
        pub fn symbol(&self) -> Result<String, Error> {
            return Ok(self.symbol)
        }

        #[ink(message)]
        pub fn decimals(&self) -> Result<u8, Error> {
            return Ok(18)
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Result<u128, Error> {
            return Ok(self.total_supply)
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Result<u128, Error> {
            return Ok(self.balances.get(&account).unwrap())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            self._transfer(owner, to, amount)?;
            return Ok(true)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error> {
            return Ok(self.allowances.get(&(owner, spender)).unwrap())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            self._approve(owner, spender, amount)?;
            return Ok(true)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u128,
        ) -> Result<bool, Error> {
            let spender: AccountId = self.env().caller();
            self._spend_allowance(from, spender, amount)?;
            self._transfer(from, to, amount)?;
            return Ok(true)
        }

        #[ink(message)]
        pub fn increase_allowance(
            &mut self,
            spender: AccountId,
            added_value: u128,
        ) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            self._approve(
                owner,
                spender,
                self.allowance(owner, spender)? + added_value,
            )?;
            return Ok(true)
        }

        #[ink(message)]
        pub fn decrease_allowance(
            &mut self,
            spender: AccountId,
            subtracted_value: u128,
        ) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            let current_allowance: u128 = self.allowance(owner, spender)?;
            if current_allowance < subtracted_value {
                return Err(Error::Custom(String::from(
                    "ERC20: decreased allowance below zero",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self._approve(owner, spender, current_allowance - subtracted_value)?;
            // <<< Please handle unchecked blocks manually
            return Ok(true)
        }

        fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) -> Result<(), Error> {
            if from.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: transfer from the zero address",
                )))
            }
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: transfer to the zero address",
                )))
            }
            self._before_token_transfer(from, to, amount)?;
            let from_balance: u128 = self.balances.get(&from).unwrap();
            if from_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC20: transfer amount exceeds balance",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.balances.insert(&from, from_balance - amount);
            // <<< Please handle unchecked blocks manually
            self.balances
                .insert(&to, self.balances.get(&to).unwrap() + amount);
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
            self._after_token_transfer(from, to, amount)?;
            Ok(())
        }

        fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
            if account.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: mint to the zero address",
                )))
            }
            self._before_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
            self.total_supply += amount;
            self.balances
                .insert(&account, self.balances.get(&account).unwrap() + amount);
            self.env().emit_event(Transfer {
                from: ZERO_ADDRESS.into(),
                to: account,
                value: amount,
            });
            self._after_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
            Ok(())
        }

        fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
            if account.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: burn from the zero address",
                )))
            }
            self._before_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
            let account_balance: u128 = self.balances.get(&account).unwrap();
            if account_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC20: burn amount exceeds balance",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.balances.insert(&account, account_balance - amount);
            // <<< Please handle unchecked blocks manually
            self.total_supply -= amount;
            self.env().emit_event(Transfer {
                from: account,
                to: ZERO_ADDRESS.into(),
                value: amount,
            });
            self._after_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
            Ok(())
        }

        fn _approve(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            if owner.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: approve from the zero address",
                )))
            }
            if spender.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: approve to the zero address",
                )))
            }
            self.allowances.insert(&(owner, spender), amount);
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
            Ok(())
        }

        fn _spend_allowance(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            let current_allowance: u128 = self.allowance(owner, spender)?;
            if current_allowance != u128::MAX {
                if current_allowance < amount {
                    return Err(Error::Custom(String::from("ERC20: insufficient allowance")))
                }
                // Please handle unchecked blocks manually >>>
                self._approve(owner, spender, current_allowance - amount)?;
                // <<< Please handle unchecked blocks manually
            }
            Ok(())
        }

        fn _before_token_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            Ok(())
        }

    }
}
