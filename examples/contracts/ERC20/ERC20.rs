#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.3.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.6.0) (token/ERC20/ERC20.sol)
///@dev Implementation of the {IERC20} interface.
///@dev Implementation of the {IERC20} interface.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms].
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms].
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit these events, as it isn't required by the specification.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit these events, as it isn't required by the specification.
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit these events, as it isn't required by the specification. Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit these events, as it isn't required by the specification. Finally, the non-standard {decreaseAllowance} and {increaseAllowance} functions have been added to mitigate the well-known issues around setting
///@dev Implementation of the {IERC20} interface. This implementation is agnostic to the way tokens are created. This means that a supply mechanism has to be added in a derived contract using {_mint}. For a generic mechanism see {ERC20PresetMinterPauser}. TIP: For a detailed writeup see our guide https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How to implement supply mechanisms]. We have followed general OpenZeppelin Contracts guidelines: functions revert instead returning `false` on failure. This behavior is nonetheless conventional and does not conflict with the expectations of ERC20 applications. Additionally, an {Approval} event is emitted on calls to {transferFrom}. This allows applications to reconstruct the allowance for all accounts just by listening to said events. Other implementations of the EIP may not emit these events, as it isn't required by the specification. Finally, the non-standard {decreaseAllowance} and {increaseAllowance} functions have been added to mitigate the well-known issues around setting allowances. See {IERC20-approve}.
#[brush::contract]
pub mod erc_20 {
    use brush::traits::AccountId;
    use ink::prelude::string::String;
    use ink_storage::Mapping;

    ///@dev Emitted when `value` tokens are moved from one account (`from`) to
    ///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`).
    ///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`).
    ///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`). Note that `value` may be zero.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    ///@dev Emitted when the allowance of a `spender` for an `owner` is set by
    ///@dev Emitted when the allowance of a `spender` for an `owner` is set by a call to {approve}. `value` is the new allowance.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    ///This enum is added just to test enum parsing
    pub enum Enum {
        FIRST,
        SECOND,
    }

    ///This struct is added just to test struct parsing
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

    impl erc_20 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            // return _name
            todo!()
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            // return _symbol
            todo!()
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            // return 18
            todo!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u128 {
            // return _totalSupply
            todo!()
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u128 {
            // return _balances[account]
            todo!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u128) -> bool {
            // address owner = msg.sender
            // _transfer(owner, to, amount)
            // return true
            todo!()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            // return _allowances[owner][spender]
            todo!()
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u128) -> bool {
            // address owner = msg.sender
            // _approve(owner, spender, amount)
            // return true
            todo!()
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool {
            // address spender = msg.sender
            // _spendAllowance(from, spender, amount)
            // _transfer(from, to, amount)
            // return true
            todo!()
        }

        #[ink(message)]
        pub fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> bool {
            // address owner = msg.sender
            // _approve(owner, spender, allowance(owner, spender) + addedValue)
            // return true
            todo!()
        }

        #[ink(message)]
        pub fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: u128) -> bool {
            // address owner = msg.sender
            // uint256 currentAllowance = allowance(owner, spender)
            // require(currentAllowance >= subtractedValue, "ERC20: decreased allowance below zero")
            // unchecked
            // _approve(owner, spender, currentAllowance - subtractedValue)
            //
            // return true
            todo!()
        }

        fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            // require(from != address(0), "ERC20: transfer from the zero address")
            // require(to != address(0), "ERC20: transfer to the zero address")
            // _beforeTokenTransfer(from, to, amount)
            // uint256 fromBalance = _balances[from]
            // require(fromBalance >= amount, "ERC20: transfer amount exceeds balance")
            // unchecked
            // _balances[from] = fromBalance - amount
            //
            // _balances[to] += amount
            // emit Transfer(from, to, amount)
            // _afterTokenTransfer(from, to, amount)
            todo!()
        }

        fn _mint(&mut self, account: AccountId, amount: u128) {
            // require(account != address(0), "ERC20: mint to the zero address")
            // _beforeTokenTransfer(address(0), account, amount)
            // _totalSupply += amount
            // _balances[account] += amount
            // emit Transfer(address(0), account, amount)
            // _afterTokenTransfer(address(0), account, amount)
            todo!()
        }

        fn _burn(&mut self, account: AccountId, amount: u128) {
            // require(account != address(0), "ERC20: burn from the zero address")
            // _beforeTokenTransfer(account, address(0), amount)
            // uint256 accountBalance = _balances[account]
            // require(accountBalance >= amount, "ERC20: burn amount exceeds balance")
            // unchecked
            // _balances[account] = accountBalance - amount
            //
            // _totalSupply -= amount
            // emit Transfer(account, address(0), amount)
            // _afterTokenTransfer(account, address(0), amount)
            todo!()
        }

        fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
            // require(owner != address(0), "ERC20: approve from the zero address")
            // require(spender != address(0), "ERC20: approve to the zero address")
            // _allowances[owner][spender] = amount
            // emit Approval(owner, spender, amount)
            todo!()
        }

        fn _spend_allowance(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
            // uint256 currentAllowance = allowance(owner, spender)
            // if (currentAllowance != type(uint256).max)
            // require(currentAllowance >= amount, "ERC20: insufficient allowance")
            // unchecked
            // _approve(owner, spender, currentAllowance - amount)
            //
            //
            todo!()
        }

        fn _before_token_transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            todo!()
        }

        fn _after_token_transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            todo!()
        }
    }
}
