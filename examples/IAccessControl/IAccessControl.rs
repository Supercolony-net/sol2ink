use brush::traits::AccountId;
#[brush::wrapper] pub type AccessControlRef = dyn AccessControl;
#[brush::trait_definition] pub trait AccessControl { #[ink(message)] fn has_role (&self,role : [u8; 32],
account : AccountId) -> bool;
#[ink(message)] fn get_role_admin (&self,role : [u8; 32]
) -> [u8; 32]
;
#[ink(message)] fn grant_role (&mut self,role : [u8; 32],
account : AccountId);
#[ink(message)] fn revoke_role (&mut self,role : [u8; 32],
account : AccountId);
#[ink(message)] fn renounce_role (&mut self,role : [u8; 32],
account : AccountId);
}