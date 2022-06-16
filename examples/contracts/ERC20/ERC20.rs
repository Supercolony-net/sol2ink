#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod erc_20 {
	use ink::prelude::string::String;
	use brush::traits::AccountId;
	use ink_storage::Mapping;

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
		pub fn new() -> Self {
			ink_lang::codegen::initialize_contract(|instance: &mut Self| {
			})
		}

		#[ink(message)]
		pub fn name(&self) -> String {
			todo!()
		}
	
		#[ink(message)]
		pub fn symbol(&self) -> String {
			todo!()
		}
	
		#[ink(message)]
		pub fn decimals(&self) -> u8 {
			todo!()
		}
	
		#[ink(message)]
		pub fn total_supply(&self) -> u128 {
			todo!()
		}
	
		#[ink(message)]
		pub fn balance_of(&self, account: AccountId) -> u128 {
			todo!()
		}
	
		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, amount: u128) -> bool {
			todo!()
		}
	
		#[ink(message)]
		pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
			todo!()
		}
	
		#[ink(message)]
		pub fn approve(&mut self, spender: AccountId, amount: u128) -> bool {
			todo!()
		}
	
		#[ink(message)]
		pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool {
			todo!()
		}
	
		#[ink(message)]
		pub fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> bool {
			todo!()
		}
	
		#[ink(message)]
		pub fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: u128) -> bool {
			todo!()
		}
	
		fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
			todo!()
		}
	
		fn _mint(&mut self, account: AccountId, amount: u128) {
			todo!()
		}
	
		fn _burn(&mut self, account: AccountId, amount: u128) {
			todo!()
		}
	
		fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
			todo!()
		}
	
		fn _spend_allowance(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
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
