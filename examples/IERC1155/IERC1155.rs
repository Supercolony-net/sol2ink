use brush::traits::AccountId;
use ink::prelude::vec::Vec;
#[brush::wrapper] pub type ERC1155Ref = dyn ERC1155;
#[brush::trait_definition] pub trait ERC1155 { #[ink(message)] fn balance_of (&self,account : AccountId,id : u128) -> u128;
#[ink(message)] fn balance_of_batch (&self,accounts : Vec<AccountId>,ids : Vec<u128>) -> Vec<u128>;
#[ink(message)] fn set_approval_for_all (&mut self,operator : AccountId,approved : bool);
#[ink(message)] fn is_approved_for_all (&self,account : AccountId,operator : AccountId) -> bool;
#[ink(message)] fn safe_transfer_from (&mut self,from : AccountId,to : AccountId,id : u128,amount : u128,data : Vec<u8>);
#[ink(message)] fn safe_batch_transfer_from (&mut self,from : AccountId,to : AccountId,ids : Vec<u128>,amounts : Vec<u128>,data : Vec<u8>);
}