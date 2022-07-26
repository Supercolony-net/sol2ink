// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

use brush::traits::AccountId;
use ink::prelude::{
    string::String,
    vec::Vec,
};

/// @dev Emitted when `value` tokens of token type `id` are transferred from `from` to `to` by `operator`.
#[ink(event)]
pub struct TransferSingle {
    #[ink(topic)]
    operator: AccountId,
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    id: u128,
    value: u128,
}

/// @dev Equivalent to multiple {TransferSingle} events, where `operator`, `from` and `to` are the same for all
/// transfers.
#[ink(event)]
pub struct TransferBatch {
    #[ink(topic)]
    operator: AccountId,
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    ids: Vec<u128>,
    values: Vec<u128>,
}

/// @dev Emitted when `account` grants or revokes permission to `operator` to transfer their tokens, according to
/// `approved`.
#[ink(event)]
pub struct ApprovalForAll {
    #[ink(topic)]
    account: AccountId,
    #[ink(topic)]
    operator: AccountId,
    approved: bool,
}

/// @dev Emitted when the URI for token type `id` changes to `value`, if it is a non-programmatic URI.
/// If an {URI} event was emitted for `id`, the standard
/// https://eips.ethereum.org/EIPS/eip-1155#metadata-extensions[guarantees] that `value` will equal the value
/// returned by {IERC1155MetadataURI-uri}.
#[ink(event)]
pub struct URI {
    value: String,
    #[ink(topic)]
    id: u128,
}

#[brush::wrapper]
pub type ERC1155Ref = dyn ERC1155;

#[brush::trait_definition]
pub trait ERC1155 {
    /// @dev Returns the amount of tokens of token type `id` owned by `account`.
    /// Requirements:
    /// - `account` cannot be the zero address.
    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error>;

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {balanceOf}.
    /// Requirements:
    /// - `accounts` and `ids` must have the same length.
    #[ink(message)]
    fn balance_of_batch(
        &self,
        accounts: Vec<AccountId>,
        ids: Vec<u128>,
    ) -> Result<Vec<u128>, Error>;

    /// @dev Grants or revokes permission to `operator` to transfer the caller's tokens, according to `approved`,
    /// Emits an {ApprovalForAll} event.
    /// Requirements:
    /// - `operator` cannot be the caller.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error>;

    /// @dev Returns true if `operator` is approved to transfer ``account``'s tokens.
    /// See {setApprovalForAll}.
    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> Result<bool, Error>;

    /// @dev Transfers `amount` tokens of token type `id` from `from` to `to`.
    /// Emits a {TransferSingle} event.
    /// Requirements:
    /// - `to` cannot be the zero address.
    /// - If the caller is not `from`, it must have been approved to spend ``from``'s tokens via {setApprovalForAll}.
    /// - `from` must have a balance of tokens of type `id` of at least `amount`.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {safeTransferFrom}.
    /// Emits a {TransferBatch} event.
    /// Requirements:
    /// - `ids` and `amounts` must have the same length.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

}
