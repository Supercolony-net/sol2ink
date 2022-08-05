#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
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
#[openbrush::contract]
pub mod erc_20 {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountIdExt,
            Storage,
            ZERO_ADDRESS,
        },
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
        First,
        Second,
    }

    /// This struct is added just to test struct parsing
    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Struct {
        field_1: u128,
        field_2: u128,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {
        pub balances: Mapping<AccountId, u128>,
        pub allowances: Mapping<(AccountId, AccountId), u128>,
        pub total_supply: u128,
        pub name: String,
        pub symbol: String,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC20 {
        #[storage_field]
        data: Data,
    }

    impl ERC20 {
        /// @dev Sets the values for {name} and {symbol}.
        /// The default value of {decimals} is 18. To select a different value for
        /// {decimals} you should overload it.
        /// All two of these values are immutable: they can only be set once during
        /// construction.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.name = name;
                instance.data.symbol = symbol;
            })
        }

        /// @dev Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> Result<String, Error> {
            return Ok(self.data.name)
        }

        /// @dev Returns the symbol of the token, usually a shorter version of the
        /// name.
        #[ink(message)]
        pub fn symbol(&self) -> Result<String, Error> {
            return Ok(self.data.symbol)
        }

        /// @dev Returns the number of decimals used to get its user representation.
        /// For example, if `decimals` equals `2`, a balance of `505` tokens should
        /// be displayed to a user as `5.05` (`505 / 10 ** 2`).
        /// Tokens usually opt for a value of 18, imitating the relationship between
        /// Ether and Wei. This is the value {ERC20} uses, unless this function is
        /// overridden;
        /// NOTE: This information is only used for _display_ purposes: it in
        /// no way affects any of the arithmetic of the contract, including
        /// {IERC20-balanceOf} and {IERC20-transfer}.
        #[ink(message)]
        pub fn decimals(&self) -> Result<u8, Error> {
            return Ok(18)
        }

        /// @dev See {IERC20-totalSupply}.
        #[ink(message)]
        pub fn total_supply(&self) -> Result<u128, Error> {
            return Ok(self.data.total_supply)
        }

        /// @dev See {IERC20-balanceOf}.
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Result<u128, Error> {
            return Ok(self.data.balances.get(&account).unwrap_or_default())
        }

        /// @dev See {IERC20-transfer}.
        /// Requirements:
        /// - `to` cannot be the zero address.
        /// - the caller must have a balance of at least `amount`.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            self._transfer(owner, to, amount)?;
            return Ok(true)
        }

        /// @dev See {IERC20-allowance}.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error> {
            return Ok(self
                .data
                .allowances
                .get(&(owner, spender))
                .unwrap_or_default())
        }

        /// @dev See {IERC20-approve}.
        /// NOTE: If `amount` is the maximum `uint256`, the allowance is not updated on
        /// `transferFrom`. This is semantically equivalent to an infinite approval.
        /// Requirements:
        /// - `spender` cannot be the zero address.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error> {
            let owner: AccountId = self.env().caller();
            self._approve(owner, spender, amount)?;
            return Ok(true)
        }

        /// @dev See {IERC20-transferFrom}.
        /// Emits an {Approval} event indicating the updated allowance. This is not
        /// required by the EIP. See the note at the beginning of {ERC20}.
        /// NOTE: Does not update the allowance if the current allowance
        /// is the maximum `uint256`.
        /// Requirements:
        /// - `from` and `to` cannot be the zero address.
        /// - `from` must have a balance of at least `amount`.
        /// - the caller must have allowance for ``from``'s tokens of at least
        /// `amount`.
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

        /// @dev Atomically increases the allowance granted to `spender` by the caller.
        /// This is an alternative to {approve} that can be used as a mitigation for
        /// problems described in {IERC20-approve}.
        /// Emits an {Approval} event indicating the updated allowance.
        /// Requirements:
        /// - `spender` cannot be the zero address.
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

        /// @dev Atomically decreases the allowance granted to `spender` by the caller.
        /// This is an alternative to {approve} that can be used as a mitigation for
        /// problems described in {IERC20-approve}.
        /// Emits an {Approval} event indicating the updated allowance.
        /// Requirements:
        /// - `spender` cannot be the zero address.
        /// - `spender` must have allowance for the caller of at least
        /// `subtractedValue`.
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

        /// @dev Moves `amount` of tokens from `from` to `to`.
        /// This internal function is equivalent to {transfer}, and can be used to
        /// e.g. implement automatic token fees, slashing mechanisms, etc.
        /// Emits a {Transfer} event.
        /// Requirements:
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `from` must have a balance of at least `amount`.
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
            let from_balance: u128 = self.data.balances.get(&from).unwrap_or_default();
            if from_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC20: transfer amount exceeds balance",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.data.balances.insert(&from, &(from_balance - amount));
            // <<< Please handle unchecked blocks manually
            self.data.balances.insert(
                &to,
                &(self.data.balances.get(&to).unwrap_or_default() + amount),
            );
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
            self._after_token_transfer(from, to, amount)?;
            Ok(())
        }

        /// @dev Creates `amount` tokens and assigns them to `account`, increasing
        /// the total supply.
        /// Emits a {Transfer} event with `from` set to the zero address.
        /// Requirements:
        /// - `account` cannot be the zero address.
        fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
            if account.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: mint to the zero address",
                )))
            }
            self._before_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
            self.data.total_supply += amount;
            self.data.balances.insert(
                &account,
                &(self.data.balances.get(&account).unwrap_or_default() + amount),
            );
            self.env().emit_event(Transfer {
                from: ZERO_ADDRESS.into(),
                to: account,
                value: amount,
            });
            self._after_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
            Ok(())
        }

        /// @dev Destroys `amount` tokens from `account`, reducing the
        /// total supply.
        /// Emits a {Transfer} event with `to` set to the zero address.
        /// Requirements:
        /// - `account` cannot be the zero address.
        /// - `account` must have at least `amount` tokens.
        fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
            if account.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC20: burn from the zero address",
                )))
            }
            self._before_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
            let account_balance: u128 = self.data.balances.get(&account).unwrap_or_default();
            if account_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC20: burn amount exceeds balance",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.data
                .balances
                .insert(&account, &(account_balance - amount));
            // <<< Please handle unchecked blocks manually
            self.data.total_supply -= amount;
            self.env().emit_event(Transfer {
                from: account,
                to: ZERO_ADDRESS.into(),
                value: amount,
            });
            self._after_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
            Ok(())
        }

        /// @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
        /// This internal function is equivalent to `approve`, and can be used to
        /// e.g. set automatic allowances for certain subsystems, etc.
        /// Emits an {Approval} event.
        /// Requirements:
        /// - `owner` cannot be the zero address.
        /// - `spender` cannot be the zero address.
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
            self.data.allowances.insert(&(owner, spender), &(amount));
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
            Ok(())
        }

        /// @dev Updates `owner` s allowance for `spender` based on spent `amount`.
        /// Does not update the allowance amount in case of infinite allowance.
        /// Revert if not enough allowance is available.
        /// Might emit an {Approval} event.
        fn _spend_allowance(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            let current_allowance: u128 = self.allowance(owner, spender)?;
            if current_allowance != u128.max {
                if current_allowance < amount {
                    return Err(Error::Custom(String::from("ERC20: insufficient allowance")))
                }
                // Please handle unchecked blocks manually >>>
                self._approve(owner, spender, current_allowance - amount)?;
                // <<< Please handle unchecked blocks manually
            }
            Ok(())
        }

        /// @dev Hook that is called before any transfer of tokens. This includes
        /// minting and burning.
        /// Calling conditions:
        /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
        /// will be transferred to `to`.
        /// - when `from` is zero, `amount` tokens will be minted for `to`.
        /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
        /// - `from` and `to` are never both zero.
        /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
        fn _before_token_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u128,
        ) -> Result<(), Error> {
            Ok(())
        }

        /// @dev Hook that is called after any transfer of tokens. This includes
        /// minting and burning.
        /// Calling conditions:
        /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
        /// has been transferred to `to`.
        /// - when `from` is zero, `amount` tokens have been minted for `to`.
        /// - when `to` is zero, `amount` of ``from``'s tokens have been burned.
        /// - `from` and `to` are never both zero.
        /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
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
