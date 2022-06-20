use brush::traits::AccountId;

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

#[brush::wrapper]
pub type IERC20Ref = dyn IERC20;

#[brush::trait_definition]
pub trait IERC20 {
	#[ink(message)]
	fn total_supply(&self) -> u128;

	#[ink(message)]
	fn balance_of(&self, account: AccountId) -> u128;

	#[ink(message)]
	fn transfer(&mut self, to: AccountId, amount: u128) -> bool;

	#[ink(message)]
	fn allowance(&self, owner: AccountId, spender: AccountId) -> u128;

	#[ink(message)]
	fn approve(&mut self, spender: AccountId, amount: u128) -> bool;

	#[ink(message)]
	fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool;

}
