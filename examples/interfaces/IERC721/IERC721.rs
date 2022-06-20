// Generated with Sol2Ink v0.2.2
// https://github.com/Supercolony-net/sol2ink

use brush::traits::AccountId;
use ink::prelude::vec::Vec;

#[ink(event)]
pub struct Transfer {
	#[ink(topic)]
	from: AccountId,
	#[ink(topic)]
	to: AccountId,
	#[ink(topic)]
	tokenId: u128,
}

#[ink(event)]
pub struct Approval {
	#[ink(topic)]
	owner: AccountId,
	#[ink(topic)]
	approved: AccountId,
	#[ink(topic)]
	tokenId: u128,
}

#[ink(event)]
pub struct ApprovalForAll {
	#[ink(topic)]
	owner: AccountId,
	#[ink(topic)]
	operator: AccountId,
	approved: bool,
}

#[brush::wrapper]
pub type IERC721Ref = dyn IERC721;

#[brush::trait_definition]
pub trait IERC721 {
	#[ink(message)]
	fn balance_of(&self, owner: AccountId) -> u128;

	#[ink(message)]
	fn owner_of(&self, token_id: u128) -> AccountId;

	#[ink(message)]
	fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u128, data: Vec<u8>);

	#[ink(message)]
	fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u128);

	#[ink(message)]
	fn transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u128);

	#[ink(message)]
	fn approve(&mut self, to: AccountId, token_id: u128);

	#[ink(message)]
	fn set_approval_for_all(&mut self, operator: AccountId, approved: bool);

	#[ink(message)]
	fn get_approved(&self, token_id: u128) -> AccountId;

	#[ink(message)]
	fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;

}
