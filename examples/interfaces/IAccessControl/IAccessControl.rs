// Generated with Sol2Ink v0.2.1
// https://github.com/Supercolony-net/sol2ink

use brush::traits::AccountId;

#[ink(event)]
pub struct RoleAdminChanged {
	#[ink(topic)]
	role: [u8; 32],
	#[ink(topic)]
	previousAdminRole: [u8; 32],
	#[ink(topic)]
	newAdminRole: [u8; 32],
}

#[ink(event)]
pub struct RoleGranted {
	#[ink(topic)]
	role: [u8; 32],
	#[ink(topic)]
	account: AccountId,
	#[ink(topic)]
	sender: AccountId,
}

#[ink(event)]
pub struct RoleRevoked {
	#[ink(topic)]
	role: [u8; 32],
	#[ink(topic)]
	account: AccountId,
	#[ink(topic)]
	sender: AccountId,
}

#[brush::wrapper]
pub type IAccessControlRef = dyn IAccessControl;

#[brush::trait_definition]
pub trait IAccessControl {
	#[ink(message)]
	fn has_role(&self, role: [u8; 32], account: AccountId) -> bool;

	#[ink(message)]
	fn get_role_admin(&self, role: [u8; 32]) -> [u8; 32];

	#[ink(message)]
	fn grant_role(&mut self, role: [u8; 32], account: AccountId);

	#[ink(message)]
	fn revoke_role(&mut self, role: [u8; 32], account: AccountId);

	#[ink(message)]
	fn renounce_role(&mut self, role: [u8; 32], account: AccountId);

}
