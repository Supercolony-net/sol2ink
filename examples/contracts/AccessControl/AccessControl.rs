#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v0.4.1
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (access/AccessControl.sol)
/// @dev Contract module that allows children to implement role-based access
/// control mechanisms. This is a lightweight version that doesn't allow enumerating role
/// members except through off-chain means by accessing the contract event logs. Some
/// applications may benefit from on-chain enumerability, for those cases see
/// {AccessControlEnumerable}.
/// Roles are referred to by their `bytes32` identifier. These should be exposed
/// in the external API and be unique. The best way to achieve this is by
/// using `public constant` hash digests:
/// ```
/// bytes32 public constant MY_ROLE = keccak256("MY_ROLE");
/// ```
/// Roles can be used to represent a set of permissions. To restrict access to a
/// function call, use {hasRole}:
/// ```
/// function foo() public {
/// require(hasRole(MY_ROLE, msg.sender));
/// ...
/// }
/// ```
/// Roles can be granted and revoked dynamically via the {grantRole} and
/// {revokeRole} functions. Each role has an associated admin role, and only
/// accounts that have a role's admin role can call {grantRole} and {revokeRole}.
/// By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which means
/// that only accounts with this role will be able to grant or revoke other
/// roles. More complex role relationships can be created by using
/// {_setRoleAdmin}.
/// WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission to
/// grant and revoke this role. Extra precautions should be taken to secure
/// accounts that have been granted it.
#[brush::contract]
pub mod access_control {
    use brush::{
        modifier_definition,
        modifiers,
        traits::AccountId,
    };
    use ink::prelude::string::String;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::Mapping;

    #[derive(Debug, Encode, Decode, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    pub const default_admin_role: [u8; 32] = 0x00;

    /// @dev Modifier that checks that an account has a specific role. Reverts
    /// with a standardized message including the required role.
    /// The format of the revert reason is given by the following regular expression:
    /// /^AccessControl: account (0x[0-9a-f]{40}) is missing role (0x[0-9a-f]{64})$/
    /// _Available since v4.1._
    ///The type of `T` should be the trait which implements the storage
    ///This will be implemented in Sol2Ink in upcoming version
    #[modifier_definition]
    pub fn only_role<T, F, R>(instance: &mut T, body: F, role: [u8; 32]) -> Result<R, Error>
    where
        T: AccessControl,
        F: FnOnce(&mut T) -> Result<R, Error>,
    {
        self._check_role(role)?;
        body(instance)
    }

    /// @dev Emitted when `newAdminRole` is set as ``role``'s admin role, replacing `previousAdminRole`
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// {RoleAdminChanged} not being emitted signaling this.
    /// _Available since v3.1._
    #[ink(event)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        previous_admin_role: [u8; 32],
        #[ink(topic)]
        new_admin_role: [u8; 32],
    }

    /// @dev Emitted when `account` is granted `role`.
    /// `sender` is the account that originated the contract call, an admin role
    /// bearer except when using {AccessControl-_setupRole}.
    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        sender: AccountId,
    }

    /// @dev Emitted when `account` is revoked `role`.
    /// `sender` is the account that originated the contract call:
    /// - if using `revokeRole`, it is the admin role bearer
    /// - if using `renounceRole`, it is the role bearer (i.e. `account`)
    #[ink(event)]
    pub struct RoleRevoked {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        sender: AccountId,
    }

    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RoleData {
        members: Mapping<AccountId, bool>,
        admin_role: [u8; 32],
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct AccessControl {
        roles: Mapping<[u8; 32], RoleData>,
    }

    impl AccessControl {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

        /// @dev See {IERC165-supportsInterface}.
        #[ink(message)]
        pub fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
            return Ok(interface_id == i_access_control.interface_id
                || super.supports_interface(interface_id)?)
        }

        /// @dev Returns `true` if `account` has been granted `role`.
        #[ink(message)]
        pub fn has_role(&self, role: [u8; 32], account: AccountId) -> Result<bool, Error> {
            return Ok(self
                .roles
                .get(&role)
                .unwrap()
                .members
                .get(&account)
                .unwrap())
        }

        /// @dev Revert with a standard message if `msg.sender` is missing `role`.
        /// Overriding this function changes the behavior of the {onlyRole} modifier.
        /// Format of the revert message is described in {_checkRole}.
        /// _Available since v4.6._
        fn _check_role(&self, role: [u8; 32]) -> Result<(), Error> {
            self._check_role(role, msg.sender)?;
            Ok(())
        }

        /// @dev Revert with a standard message if `account` is missing `role`.
        /// The format of the revert reason is given by the following regular expression:
        /// /^AccessControl: account (0x[0-9a-f]{40}) is missing role (0x[0-9a-f]{64})$/
        fn _check_role(&self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            if !self.has_role(role, account)? {
                self._revert(
                    (abi._encode_packed(
                        "AccessControl: account ",
                        strings._to_hex_string(account)?,
                        " is missing role ",
                        strings._to_hex_string((role as u128), 32)?,
                    )? as String),
                )?;
            }
            Ok(())
        }

        /// @dev Returns the admin role that controls `role`. See {grantRole} and
        /// {revokeRole}.
        /// To change a role's admin, use {_setRoleAdmin}.
        #[ink(message)]
        pub fn get_role_admin(&self, role: [u8; 32]) -> Result<[u8; 32], Error> {
            return Ok(self.roles.get(&role).unwrap().admin_role)
        }

        /// @dev Grants `role` to `account`.
        /// If `account` had not been already granted `role`, emits a {RoleGranted}
        /// event.
        /// Requirements:
        /// - the caller must have ``role``'s admin role.
        /// May emit a {RoleGranted} event.
        #[ink(message)]
        #[modifiers(_only_role(get_role_admin(role)))]
        pub fn grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            self._grant_role(role, account)?;
            Ok(())
        }

        /// @dev Revokes `role` from `account`.
        /// If `account` had been granted `role`, emits a {RoleRevoked} event.
        /// Requirements:
        /// - the caller must have ``role``'s admin role.
        /// May emit a {RoleRevoked} event.
        #[ink(message)]
        #[modifiers(_only_role(get_role_admin(role)))]
        pub fn revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            self._revoke_role(role, account)?;
            Ok(())
        }

        /// @dev Revokes `role` from the calling account.
        /// Roles are often managed via {grantRole} and {revokeRole}: this function's
        /// purpose is to provide a mechanism for accounts to lose their privileges
        /// if they are compromised (such as when a trusted device is misplaced).
        /// If the calling account had been revoked `role`, emits a {RoleRevoked}
        /// event.
        /// Requirements:
        /// - the caller must be `account`.
        /// May emit a {RoleRevoked} event.
        #[ink(message)]
        pub fn renounce_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            if account != self.env().caller() {
                return Err(Error::Custom(String::from(
                    "AccessControl: can only renounce roles for self",
                )))
            }
            self._revoke_role(role, account)?;
            Ok(())
        }

        /// @dev Grants `role` to `account`.
        /// If `account` had not been already granted `role`, emits a {RoleGranted}
        /// event. Note that unlike {grantRole}, this function doesn't perform any
        /// checks on the calling account.
        /// May emit a {RoleGranted} event.
        /// [WARNING]
        /// ====
        /// This function should only be called from the constructor when setting
        /// up the initial roles for the system.
        /// Using this function in any other way is effectively circumventing the admin
        /// system imposed by {AccessControl}.
        /// ====
        /// NOTE: This function is deprecated in favor of {_grantRole}.
        fn _setup_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            self._grant_role(role, account)?;
            Ok(())
        }

        /// @dev Sets `adminRole` as ``role``'s admin role.
        /// Emits a {RoleAdminChanged} event.
        fn _set_role_admin(&mut self, role: [u8; 32], admin_role: [u8; 32]) -> Result<(), Error> {
            let previous_admin_role: [u8; 32] = self.get_role_admin(role)?;
            self.roles.get(&role).unwrap().admin_role = admin_role;
            self.env().emit_event(RoleAdminChanged {
                role,
                previous_admin_role,
                new_admin_role: admin_role,
            });
            Ok(())
        }

        /// @dev Grants `role` to `account`.
        /// Internal function without access restriction.
        /// May emit a {RoleGranted} event.
        fn _grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            if !self.has_role(role, account)? {
                self.roles
                    .get(&role)
                    .unwrap()
                    .members
                    .get(&account)
                    .unwrap() = true;
                self.env().emit_event(RoleGranted {
                    role,
                    account,
                    sender: self.env().caller(),
                });
            }
            Ok(())
        }

        /// @dev Revokes `role` from `account`.
        /// Internal function without access restriction.
        /// May emit a {RoleRevoked} event.
        fn _revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
            if self.has_role(role, account)? {
                self.roles
                    .get(&role)
                    .unwrap()
                    .members
                    .get(&account)
                    .unwrap() = false;
                self.env().emit_event(RoleRevoked {
                    role,
                    account,
                    sender: self.env().caller(),
                });
            }
            Ok(())
        }

    }
}
