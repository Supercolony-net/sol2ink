#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod erc_20 {
	use ink::prelude::string::String;
	use ink_storage::Mapping;
	use brush::traits::AccountId;

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
	}
}
