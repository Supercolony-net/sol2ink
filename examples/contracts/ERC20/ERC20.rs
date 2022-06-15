#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod erc_20 {

	#[ink(storage)]
	#[derive(Default, SpreadAllocate)]
	pub struct ERC20{
	}
	
	impl ERC20 {
		#[ink(constructor)]
		pub fn new(	) -> Self {
			ink_lang::codegen::initialize_contract(|instance: &mut Self| {
			})
		}
	}
}
