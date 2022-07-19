#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (token/ERC721/ERC721.sol)
/// @dev Implementation of https://eips.ethereum.org/EIPS/eip-721[ERC721] Non-Fungible Token Standard, including
/// the Metadata extension, but not including the Enumerable extension, which is available separately as
/// {ERC721Enumerable}.
#[brush::contract]
pub mod erc_721 {
    use brush::traits::{
        AccountId,
        AcountIdExt,
        ZERO_ADDRESS,
    };
    use ink::prelude::{
        string::String,
        vec::Vec,
    };
    use ink_lang::codegen::EmitEvent;
    use ink_storage::Mapping;

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    ///using Address for address;
    ///using Strings for uint256;
    ///Token name
    ///Token symbol
    ///Mapping from token ID to owner address
    ///Mapping owner address to token count
    ///Mapping from token ID to approved address
    ///Mapping from owner to operator approvals
    /// @dev Emitted when `tokenId` token is transferred from `from` to `to`.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        token_id: u128,
    }

    /// @dev Emitted when `owner` enables `approved` to manage the `tokenId` token.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        approved: AccountId,
        #[ink(topic)]
        token_id: u128,
    }

    /// @dev Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct ERC721 {
        name: String,
        symbol: String,
        owners: Mapping<u128, AccountId>,
        balances: Mapping<AccountId, u128>,
        token_approvals: Mapping<u128, AccountId>,
        operator_approvals: Mapping<(AccountId, AccountId), bool>,
    }

    impl ERC721 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.name = name;
                instance.symbol = symbol;
            })
        }

        #[ink(message)]
        pub fn supports_interface(&self, interface_id: bytes4) -> Result<bool, Error> {
            return Ok(interface_id == type_of(ierc_721).interface_id
                || interface_id == type_of(ierc_721_metadata).interface_id
                || self.supports_interface(interface_id)?)
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Result<u128, Error> {
            if owner.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC721: address zero is not a valid owner",
                )))
            }
            return Ok(self.balances.get(&owner).unwrap())
        }

        #[ink(message)]
        pub fn owner_of(&self, token_id: u128) -> Result<AccountId, Error> {
            let owner: AccountId = self.owners.get(&token_id).unwrap();
            if owner.is_zero() {
                return Err(Error::Custom(String::from("ERC721: invalid token ID")))
            }
            return Ok(owner)
        }

        #[ink(message)]
        pub fn name(&self) -> Result<String, Error> {
            return Ok(self.name)
        }

        #[ink(message)]
        pub fn symbol(&self) -> Result<String, Error> {
            return Ok(self.symbol)
        }

        #[ink(message)]
        pub fn token_uri(&self, token_id: u128) -> Result<String, Error> {
            self._require_minted(token_id)?;
            let base_uri: String = base_uri();
            return Ok(bytes(base_uri).length > 0)
        }

        fn _base_uri(&self) -> Result<String, Error> {
            return Ok("")
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
            let owner: AccountId = erc_721.owner_of(token_id);
            if to == owner {
                return Err(Error::Custom(String::from(
                    "ERC721: approval to current owner",
                )))
            }
            if msg_sender() != owner {
                return Err(Error::Custom(String::from(
                    "ERC721: approve caller is not token owner nor approved for all",
                )))
            }
            self._approve(to, token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get_approved(&self, token_id: u128) -> Result<AccountId, Error> {
            self._require_minted(token_id)?;
            return Ok(self.token_approvals.get(&token_id).unwrap())
        }

        #[ink(message)]
        pub fn set_approval_for_all(
            &mut self,
            operator: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            self._set_approval_for_all(msg_sender(), operator, approved)?;
            Ok(())
        }

        #[ink(message)]
        pub fn is_approved_for_all(
            &self,
            owner: AccountId,
            operator: AccountId,
        ) -> Result<bool, Error> {
            return Ok(self.operator_approvals.get(&(owner, operator)).unwrap())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
        ) -> Result<(), Error> {
            // solhint-disable-next-line max-line-length
            if !self._is_approved_or_owner(msg_sender(), token_id)? {
                return Err(Error::Custom(String::from(
                    "ERC721: caller is not token owner nor approved",
                )))
            }
            self._transfer(from, to, token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
        ) -> Result<(), Error> {
            self.safe_transfer_from(from, to, token_id, "")?;
            Ok(())
        }

        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            if !self._is_approved_or_owner(msg_sender(), token_id)? {
                return Err(Error::Custom(String::from(
                    "ERC721: caller is not token owner nor approved",
                )))
            }
            self._safe_transfer(from, to, token_id, data)?;
            Ok(())
        }

        fn _safe_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            self._transfer(from, to, token_id)?;
            if !self._check_on_erc_721_received(from, to, token_id, data)? {
                return Err(Error::Custom(String::from(
                    "ERC721: transfer to non ERC721Receiver implementer",
                )))
            }
            Ok(())
        }

        fn _exists(&self, token_id: u128) -> Result<bool, Error> {
            return Ok(!self.owners.get(&token_id).unwrap().is_zero())
        }

        fn _is_approved_or_owner(&self, spender: AccountId, token_id: u128) -> Result<bool, Error> {
            let owner: AccountId = erc_721.owner_of(token_id);
            return Ok((spender == owner
                || self.is_approved_for_all(owner, spender)
                || self.get_approved(token_id)? == spender)?)
        }

        fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
            self._safe_mint(to, token_id, "")?;
            Ok(())
        }

        fn _safe_mint(
            &mut self,
            to: AccountId,
            token_id: u128,
            data: Vec<u8>,
        ) -> Result<(), Error> {
            self._mint(to, token_id)?;
            if !self._check_on_erc_721_received(ZERO_ADDRESS.into(), to, token_id, data)? {
                return Err(Error::Custom(String::from(
                    "ERC721: transfer to non ERC721Receiver implementer",
                )))
            }
            Ok(())
        }

        fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC721: mint to the zero address",
                )))
            }
            if self._exists(token_id)? {
                return Err(Error::Custom(String::from("ERC721: token already minted")))
            }
            self._before_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
            self.balances
                .insert(&to, self.balances.get(&to).unwrap() + 1);
            self.owners.insert(&token_id, to);
            self.env().emit_event(Transfer {
                from: ZERO_ADDRESS.into(),
                to,
                tokenId: token_id,
            });
            self._after_token_transfer(ZERO_ADDRESS.into(), to, token_id)?;
            Ok(())
        }

        fn _burn(&mut self, token_id: u128) -> Result<(), Error> {
            let owner: AccountId = erc_721.owner_of(token_id);
            self._before_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
            // Clear approvals
            delete_token_approvals.get(&token_id).unwrap();
            self.balances
                .insert(&owner, self.balances.get(&owner).unwrap() - 1);
            delete_owners.get(&token_id).unwrap();
            self.env().emit_event(Transfer {
                from: owner,
                to: ZERO_ADDRESS.into(),
                tokenId: token_id,
            });
            self._after_token_transfer(owner, ZERO_ADDRESS.into(), token_id)?;
            Ok(())
        }

        fn _transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
        ) -> Result<(), Error> {
            if erc_721.owner_of(token_id) != from {
                return Err(Error::Custom(String::from(
                    "ERC721: transfer from incorrect owner",
                )))
            }
            if to.is_zero() {
                return Err(Error::Custom(String::from(
                    "ERC721: transfer to the zero address",
                )))
            }
            self._before_token_transfer(from, to, token_id)?;
            // Clear approvals from the previous owner
            delete_token_approvals.get(&token_id).unwrap();
            self.balances
                .insert(&from, self.balances.get(&from).unwrap() - 1);
            self.balances
                .insert(&to, self.balances.get(&to).unwrap() + 1);
            self.owners.insert(&token_id, to);
            self.env().emit_event(Transfer {
                from,
                to,
                tokenId: token_id,
            });
            self._after_token_transfer(from, to, token_id)?;
            Ok(())
        }

        fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
            self.token_approvals.insert(&token_id, to);
            self.env().emit_event(Approval {
                owner: erc_721.owner_of(token_id),
                approved: to,
                tokenId: token_id,
            });
            Ok(())
        }

        fn _set_approval_for_all(
            &mut self,
            owner: AccountId,
            operator: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            if owner == operator {
                return Err(Error::Custom(String::from("ERC721: approve to caller")))
            }
            self.operator_approvals.insert(&(owner, operator), approved);
            self.env().emit_event(ApprovalForAll {
                owner,
                operator,
                approved,
            });
            Ok(())
        }

        fn _require_minted(&self, token_id: u128) -> Result<(), Error> {
            if !self._exists(token_id)? {
                return Err(Error::Custom(String::from("ERC721: invalid token ID")))
            }
            Ok(())
        }

        fn _check_on_erc_721_received(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
            data: Vec<u8>,
        ) -> Result<bool, Error> {
            if to.is_contract() {
                // Please handle try/catch blocks manually >>>
                if true {
                    // try IERC721Receiver(to).onERC721Received(_msgSender(), from, tokenId, data) returns (bytes4 retval) {
                    return Ok(retval == ierc_721_receiver.on_erc_721_received.selector)
                } else {
                    // catch (bytes reason) {
                    if reason.length == 0 {
                        self._revert("ERC721: transfer to non ERC721Receiver implementer")?;
                    } else {
                        // @solidity memory-safe-assembly
                        // Please handle assembly blocks manually >>>
                        // revert(add(32, reason), mload(reason))
                        // <<< Please handle assembly blocks manually
                    }
                }
                // <<< Please handle try/catch blocks manually
            } else {
                return Ok(true)
            }
        }

        fn _before_token_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
        ) -> Result<(), Error> {
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: u128,
        ) -> Result<(), Error> {
            Ok(())
        }

    }
}
