#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod erc_20 {
	use brush::traits::AccountId;
	use ink_storage::Mapping;
	use ink::prelude::string::String;

	#[ink(event)]
	pub struct Transfer {
		#[ink(topic)]
		Transfer: AccountId,
		#[ink(topic)]
		Transfer: AccountId,
		Transfer: u128,
	}

	#[ink(event)]
	pub struct Approval {
		#[ink(topic)]
		Approval: AccountId,
		#[ink(topic)]
		Approval: AccountId,
		Approval: u128,
	}

	pub enum Enum {
		FIRST, 
		SECOND, 
	}

	#[derive(Default, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
	pub struct Struct {
		pub field_1: u128,
		pub field_2: u128,
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
				// _name = name_;
				// _symbol = symbol_;
			})
		}

		#[ink(message)]
		pub fn name(&self) -> String {
			// return _name;
			todo!()
		}
	
		#[ink(message)]
		pub fn symbol(&self) -> String {
			// return _symbol;
			todo!()
		}
	
		#[ink(message)]
		pub fn decimals(&self) -> u8 {
			// return 18;
			todo!()
		}
	
		#[ink(message)]
		pub fn total_supply(&self) -> u128 {
			// return _totalSupply;
			todo!()
		}
	
		#[ink(message)]
		pub fn balance_of(&self, account: AccountId) -> u128 {
			// return _balances[account];
			todo!()
		}
	
		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, amount: u128) -> bool {
			// address owner = msg.sender;
			// _transfer(owner, to, amount);
			// return true;
			todo!()
		}
	
		#[ink(message)]
		pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
			// return _allowances[owner][spender];
			todo!()
		}
	
		#[ink(message)]
		pub fn approve(&mut self, spender: AccountId, amount: u128) -> bool {
			// address owner = msg.sender;
			// _approve(owner, spender, amount);
			// return true;
			todo!()
		}
	
		#[ink(message)]
		pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool {
			// address spender = msg.sender;
			// _spendAllowance(from, spender, amount);
			// _transfer(from, to, amount);
			// return true;
			todo!()
		}
	
		#[ink(message)]
		pub fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> bool {
			// address owner = msg.sender;
			// _approve(owner, spender, allowance(owner, spender) + addedValue);
			// return true;
			todo!()
		}
	
		#[ink(message)]
		pub fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: u128) -> bool {
			// address owner = msg.sender;
			// uint256 currentAllowance = allowance(owner, spender);
			// require(currentAllowance >= subtractedValue, "ERC20: decreased allowance below zero");
			// unchecked {
			// _approve(owner, spender, currentAllowance - subtractedValue);
			// }
			// return true;
			todo!()
		}
	
		fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
			// require(from != address(0), "ERC20: transfer from the zero address");
			// require(to != address(0), "ERC20: transfer to the zero address");
			// _beforeTokenTransfer(from, to, amount);
			// uint256 fromBalance = _balances[from];
			// require(fromBalance >= amount, "ERC20: transfer amount exceeds balance");
			// unchecked {
			// _balances[from] = fromBalance - amount;
			// }
			// _balances[to] += amount;
			// emit Transfer(from, to, amount);
			// _afterTokenTransfer(from, to, amount);
			todo!()
		}
	
		fn _mint(&mut self, account: AccountId, amount: u128) {
			// require(account != address(0), "ERC20: mint to the zero address");
			// _beforeTokenTransfer(address(0), account, amount);
			// _totalSupply += amount;
			// _balances[account] += amount;
			// emit Transfer(address(0), account, amount);
			// _afterTokenTransfer(address(0), account, amount);
			todo!()
		}
	
		fn _burn(&mut self, account: AccountId, amount: u128) {
			// require(account != address(0), "ERC20: burn from the zero address");
			// _beforeTokenTransfer(account, address(0), amount);
			// uint256 accountBalance = _balances[account];
			// require(accountBalance >= amount, "ERC20: burn amount exceeds balance");
			// unchecked {
			// _balances[account] = accountBalance - amount;
			// }
			// _totalSupply -= amount;
			// emit Transfer(account, address(0), amount);
			// _afterTokenTransfer(account, address(0), amount);
			todo!()
		}
	
		fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
			// require(owner != address(0), "ERC20: approve from the zero address");
			// require(spender != address(0), "ERC20: approve to the zero address");
			// _allowances[owner][spender] = amount;
			// emit Approval(owner, spender, amount);
			todo!()
		}
	
		fn _spend_allowance(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
			// uint256 currentAllowance = allowance(owner, spender);
			// if (currentAllowance != type(uint256).max) {
			// require(currentAllowance >= amount, "ERC20: insufficient allowance");
			// unchecked {
			// _approve(owner, spender, currentAllowance - amount);
			// }
			// }
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
