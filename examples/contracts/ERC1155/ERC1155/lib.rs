#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (token/ERC1155/ERC1155.sol)
/// @dev Implementation of the basic standard multi-token.
/// See https://eips.ethereum.org/EIPS/eip-1155
/// Originally based on code by Enjin: https://github.com/enjin/erc-1155
/// _Available since v3.1._
#[openbrush::contract]
pub mod erc_1155 {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountIdExt,
            Storage,
            ZERO_ADDRESS,
        },
    };
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }


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

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {
        ///Mapping from token ID to account balances
        pub balances: Mapping<(u128, AccountId), u128>,
        ///Mapping from account to operator approvals
        pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
        ///Used as the URI for all token types by relying on ID substitution, e.g. https://token-cdn-domain/{id}.json
        pub uri: String,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC1155 {
        #[storage_field]
        data: Data,
    }

    impl ERC1155 {
        /// @dev See {_setURI}.
        #[ink(constructor)]
        pub fn new(uri: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_uri(uri)?;
            })
        }

        /// @dev See {IERC165-supportsInterface}.
        #[ink(message)]
        pub fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
            return Ok(interface_id == ierc_1155.interface_id
                || interface_id == ierc_1155_metadata_uri.interface_id
                || super.supports_interface(interface_id)?)
        }

        /// @dev See {IERC1155MetadataURI-uri}.
        /// This implementation returns the same URI for *all* token types. It relies
        /// on the token type ID substitution mechanism
        /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
        /// Clients calling this function must replace the `\{id\}` substring with the
        /// actual token type ID.
        #[ink(message)]
        pub fn uri(&self) -> Result<String, Error> {
            return Ok(self.data.uri)
        }

        /// @dev See {IERC1155-balanceOf}.
        /// Requirements:
        /// - `account` cannot be the zero address.
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error> {
            if account.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: address zero is not a valid owner",
                )))
            }
            return Ok(self.data.balances.get(&(id, account)).unwrap_or_default())
        }

        /// @dev See {IERC1155-balanceOfBatch}.
        /// Requirements:
        /// - `accounts` and `ids` must have the same length.
        #[ink(message)]
        pub fn balance_of_batch(
            &self,
            accounts: Vec<AccountId>,
            ids: Vec<u128>,
        ) -> Result<Vec<u128>, Error> {
            if accounts.length != ids.length {
                return Err(Error::Custom(String::from(
                    "ERC1155: accounts and ids length mismatch",
                )))
            }
            let batch_balances: Vec<u128> = vec![u128::default(); accounts.length];
            let i: u128 = 0;
            while i < accounts.length {
                batch_balances.insert(
                    &i,
                    &(self.balance_of(
                        accounts.get(&i).unwrap_or_default(),
                        ids.get(&i).unwrap_or_default(),
                    )?),
                );
                i += 1;
            }
            return Ok(batch_balances)
        }

        /// @dev See {IERC1155-setApprovalForAll}.
        #[ink(message)]
        pub fn set_approval_for_all(
            &mut self,
            operator: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            self._set_approval_for_all(self.env().caller(), operator, approved)?;
            Ok(())
        }

        /// @dev See {IERC1155-isApprovedForAll}.
        #[ink(message)]
        pub fn is_approved_for_all(
            &self,
            account: AccountId,
            operator: AccountId,
        ) -> Result<bool, Error> {
            return Ok(self
                .data
                .operator_approvals
                .get(&(account, operator))
                .unwrap_or_default())
        }

        /// @dev See {IERC1155-safeTransferFrom}.
        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: u128,
            amount: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if from != self.env().caller() || self.is_approved_for_all(from, msg.sender)? {
                return Err(Error::Custom(String::from(
                    "ERC1155: caller is not token owner nor approved",
                )))
            }
            self._safe_transfer_from(from, to, id, amount, data)?;
            Ok(())
        }

        /// @dev See {IERC1155-safeBatchTransferFrom}.
        #[ink(message)]
        pub fn safe_batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if from != self.env().caller() || self.is_approved_for_all(from, msg.sender)? {
                return Err(Error::Custom(String::from(
                    "ERC1155: caller is not token owner nor approved",
                )))
            }
            self._safe_batch_transfer_from(from, to, ids, amounts, data)?;
            Ok(())
        }

        /// @dev Transfers `amount` tokens of token type `id` from `from` to `to`.
        /// Emits a {TransferSingle} event.
        /// Requirements:
        /// - `to` cannot be the zero address.
        /// - `from` must have a balance of tokens of type `id` of at least `amount`.
        /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
        /// acceptance magic value.
        fn _safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: u128,
            amount: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: transfer to the zero address",
                )))
            }
            let operator: AccountId = self.env().caller();
            let ids: Vec<u128> = self._as_singleton_array(id)?;
            let amounts: Vec<u128> = self._as_singleton_array(amount)?;
            self._before_token_transfer(operator, from, to, ids, amounts, data)?;
            let from_balance: u128 = self.data.balances.get(&(id, from)).unwrap_or_default();
            if from_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC1155: insufficient balance for transfer",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.data
                .balances
                .insert(&(id, from), &(from_balance - amount));
            // <<< Please handle unchecked blocks manually
            self.data.balances.insert(
                &(id, to),
                &(self.data.balances.get(&(id, to)).unwrap_or_default() + amount),
            );
            self.env().emit_event(TransferSingle {
                operator,
                from,
                to,
                id,
                value: amount,
            });
            self._after_token_transfer(operator, from, to, ids, amounts, data)?;
            self._do_safe_transfer_acceptance_check(operator, from, to, id, amount, data)?;
            Ok(())
        }

        /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_safeTransferFrom}.
        /// Emits a {TransferBatch} event.
        /// Requirements:
        /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
        /// acceptance magic value.
        fn _safe_batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if ids.length != amounts.length {
                return Err(Error::Custom(String::from(
                    "ERC1155: ids and amounts length mismatch",
                )))
            }
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: transfer to the zero address",
                )))
            }
            let operator: AccountId = self.env().caller();
            self._before_token_transfer(operator, from, to, ids, amounts, data)?;
            let i: u128 = 0;
            while i < ids.length {
                let id: u128 = ids.get(&i).unwrap_or_default();
                let amount: u128 = amounts.get(&i).unwrap_or_default();
                let from_balance: u128 = self.data.balances.get(&(id, from)).unwrap_or_default();
                if from_balance < amount {
                    return Err(Error::Custom(String::from(
                        "ERC1155: insufficient balance for transfer",
                    )))
                }
                // Please handle unchecked blocks manually >>>
                self.data
                    .balances
                    .insert(&(id, from), &(from_balance - amount));
                // <<< Please handle unchecked blocks manually
                self.data.balances.insert(
                    &(id, to),
                    &(self.data.balances.get(&(id, to)).unwrap_or_default() + amount),
                );
                i += 1;
            }
            self.env().emit_event(TransferBatch {
                operator,
                from,
                to,
                ids,
                values: amounts,
            });
            self._after_token_transfer(operator, from, to, ids, amounts, data)?;
            self._do_safe_batch_transfer_acceptance_check(operator, from, to, ids, amounts, data)?;
            Ok(())
        }

        /// @dev Sets a new URI for all token types, by relying on the token type ID
        /// substitution mechanism
        /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
        /// By this mechanism, any occurrence of the `\{id\}` substring in either the
        /// URI or any of the amounts in the JSON file at said URI will be replaced by
        /// clients with the token type ID.
        /// For example, the `https://token-cdn-domain/\{id\}.json` URI would be
        /// interpreted by clients as
        /// `https://token-cdn-domain/000000000000000000000000000000000000000000000000000000000004cce0.json`
        /// for token type ID 0x4cce0.
        /// See {uri}.
        /// Because these URIs cannot be meaningfully represented by the {URI} event,
        /// this function emits no events.
        fn _set_uri(&mut self, newuri: String) -> Result<(), Error> {
            self.data.uri = newuri;
            Ok(())
        }

        /// @dev Creates `amount` tokens of token type `id`, and assigns them to `to`.
        /// Emits a {TransferSingle} event.
        /// Requirements:
        /// - `to` cannot be the zero address.
        /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
        /// acceptance magic value.
        fn _mint(
            &mut self,
            to: AccountId,
            id: u128,
            amount: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: mint to the zero address",
                )))
            }
            let operator: AccountId = self.env().caller();
            let ids: Vec<u128> = self._as_singleton_array(id)?;
            let amounts: Vec<u128> = self._as_singleton_array(amount)?;
            self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
            self.data.balances.insert(
                &(id, to),
                &(self.data.balances.get(&(id, to)).unwrap_or_default() + amount),
            );
            self.env().emit_event(TransferSingle {
                operator,
                from: ZERO_ADDRESS.into(),
                to,
                id,
                value: amount,
            });
            self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
            self._do_safe_transfer_acceptance_check(
                operator,
                ZERO_ADDRESS.into(),
                to,
                id,
                amount,
                data,
            )?;
            Ok(())
        }

        /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_mint}.
        /// Emits a {TransferBatch} event.
        /// Requirements:
        /// - `ids` and `amounts` must have the same length.
        /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
        /// acceptance magic value.
        fn _mint_batch(
            &mut self,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: mint to the zero address",
                )))
            }
            if ids.length != amounts.length {
                return Err(Error::Custom(String::from(
                    "ERC1155: ids and amounts length mismatch",
                )))
            }
            let operator: AccountId = self.env().caller();
            self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
            let i: u128 = 0;
            while i < ids.length {
                self.data.balances.insert(
                    &(ids.get(&i).unwrap_or_default(), to),
                    &(self
                        .data
                        .balances
                        .get(&(ids.get(&i).unwrap_or_default(), to))
                        .unwrap_or_default()
                        + amounts.get(&i).unwrap_or_default()),
                );
                i += 1;
            }
            self.env().emit_event(TransferBatch {
                operator,
                from: ZERO_ADDRESS.into(),
                to,
                ids,
                values: amounts,
            });
            self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
            self._do_safe_batch_transfer_acceptance_check(
                operator,
                ZERO_ADDRESS.into(),
                to,
                ids,
                amounts,
                data,
            )?;
            Ok(())
        }

        /// @dev Destroys `amount` tokens of token type `id` from `from`
        /// Emits a {TransferSingle} event.
        /// Requirements:
        /// - `from` cannot be the zero address.
        /// - `from` must have at least `amount` tokens of token type `id`.
        fn _burn(&mut self, from: AccountId, id: u128, amount: u128) -> Result<(), Error> {
            if from.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: burn from the zero address",
                )))
            }
            let operator: AccountId = self.env().caller();
            let ids: Vec<u128> = self._as_singleton_array(id)?;
            let amounts: Vec<u128> = self._as_singleton_array(amount)?;
            self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
            let from_balance: u128 = self.data.balances.get(&(id, from)).unwrap_or_default();
            if from_balance < amount {
                return Err(Error::Custom(String::from(
                    "ERC1155: burn amount exceeds balance",
                )))
            }
            // Please handle unchecked blocks manually >>>
            self.data
                .balances
                .insert(&(id, from), &(from_balance - amount));
            // <<< Please handle unchecked blocks manually
            self.env().emit_event(TransferSingle {
                operator,
                from,
                to: ZERO_ADDRESS.into(),
                id,
                value: amount,
            });
            self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
            Ok(())
        }

        /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_burn}.
        /// Emits a {TransferBatch} event.
        /// Requirements:
        /// - `ids` and `amounts` must have the same length.
        fn _burn_batch(
            &mut self,
            from: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
        ) -> Result<(), Error> {
            if from.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC1155: burn from the zero address",
                )))
            }
            if ids.length != amounts.length {
                return Err(Error::Custom(String::from(
                    "ERC1155: ids and amounts length mismatch",
                )))
            }
            let operator: AccountId = self.env().caller();
            self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
            let i: u128 = 0;
            while i < ids.length {
                let id: u128 = ids.get(&i).unwrap_or_default();
                let amount: u128 = amounts.get(&i).unwrap_or_default();
                let from_balance: u128 = self.data.balances.get(&(id, from)).unwrap_or_default();
                if from_balance < amount {
                    return Err(Error::Custom(String::from(
                        "ERC1155: burn amount exceeds balance",
                    )))
                }
                // Please handle unchecked blocks manually >>>
                self.data
                    .balances
                    .insert(&(id, from), &(from_balance - amount));
                // <<< Please handle unchecked blocks manually
                i += 1;
            }
            self.env().emit_event(TransferBatch {
                operator,
                from,
                to: ZERO_ADDRESS.into(),
                ids,
                values: amounts,
            });
            self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
            Ok(())
        }

        /// @dev Approve `operator` to operate on all of `owner` tokens
        /// Emits an {ApprovalForAll} event.
        fn _set_approval_for_all(
            &mut self,
            owner: AccountId,
            operator: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            if owner == operator {
                return Err(Error::Custom(String::from(
                    "ERC1155: setting approval status for self",
                )))
            }
            self.data
                .operator_approvals
                .insert(&(owner, operator), &(approved));
            self.env().emit_event(ApprovalForAll {
                account: owner,
                operator,
                approved,
            });
            Ok(())
        }

        /// @dev Hook that is called before any token transfer. This includes minting
        /// and burning, as well as batched variants.
        /// The same hook is called on both single and batched variants. For single
        /// transfers, the length of the `ids` and `amounts` arrays will be 1.
        /// Calling conditions (for each `id` and `amount` pair):
        /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
        /// of token type `id` will be  transferred to `to`.
        /// - When `from` is zero, `amount` tokens of token type `id` will be minted
        /// for `to`.
        /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
        /// will be burned.
        /// - `from` and `to` are never both zero.
        /// - `ids` and `amounts` have the same, non-zero length.
        /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
        fn _before_token_transfer(
            &mut self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            Ok(())
        }

        /// @dev Hook that is called after any token transfer. This includes minting
        /// and burning, as well as batched variants.
        /// The same hook is called on both single and batched variants. For single
        /// transfers, the length of the `id` and `amount` arrays will be 1.
        /// Calling conditions (for each `id` and `amount` pair):
        /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
        /// of token type `id` will be  transferred to `to`.
        /// - When `from` is zero, `amount` tokens of token type `id` will be minted
        /// for `to`.
        /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
        /// will be burned.
        /// - `from` and `to` are never both zero.
        /// - `ids` and `amounts` have the same, non-zero length.
        /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
        fn _after_token_transfer(
            &mut self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            Ok(())
        }

        fn _do_safe_transfer_acceptance_check(
            &mut self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            id: u128,
            amount: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if to.is_contract()? {
                // Please handle try/catch blocks manually >>>
                if true {
                    // try IERC1155Receiver(to).onERC1155Received(operator, from, id, amount, data) returns (bytes4 response) {
                    if response != ierc_1155_receiver.on_erc_1155_received.selector {
                        revert("ERC1155: ERC1155Receiver rejected tokens")?;
                    }
                } else if false {
                    // catch Error(string reason) {
                    revert(reason)?;
                    // <<< Please handle try/catch blocks manually
                } else if false {
                    // catch {
                    revert("ERC1155: transfer to non-ERC1155Receiver implementer")?;
                    // <<< Please handle try/catch blocks manually
                }
            }
            Ok(())
        }

        fn _do_safe_batch_transfer_acceptance_check(
            &mut self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            amounts: Vec<u128>,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if to.is_contract()? {
                // Please handle try/catch blocks manually >>>
                if true {
                    // try IERC1155Receiver(to).onERC1155BatchReceived(operator, from, ids, amounts, data) returns ( bytes4 response ) {
                    if response != ierc_1155_receiver.on_erc_1155_batch_received.selector {
                        revert("ERC1155: ERC1155Receiver rejected tokens")?;
                    }
                } else if false {
                    // catch Error(string reason) {
                    revert(reason)?;
                    // <<< Please handle try/catch blocks manually
                } else if false {
                    // catch {
                    revert("ERC1155: transfer to non-ERC1155Receiver implementer")?;
                    // <<< Please handle try/catch blocks manually
                }
            }
            Ok(())
        }

        fn _as_singleton_array(&self, element: u128) -> Result<Vec<u128>, Error> {
            let array: Vec<u128> = vec![u128::default(); 1];
            array.insert(&0, &(element));
            return Ok(array)
        }

    }
}
