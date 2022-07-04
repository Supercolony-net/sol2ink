// Generated with Sol2Ink v0.4.0
// https://github.com/Supercolony-net/sol2ink

use brush::traits::AccountId;

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

#[brush::wrapper]
pub type AccessControlRef = dyn AccessControl;

#[brush::trait_definition]
pub trait AccessControl {
    /// @dev Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    fn has_role(&self, role: [u8; 32], account: AccountId) -> Result<bool, Error>;

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    /// To change a role's admin, use {AccessControl-_setRoleAdmin}.
    #[ink(message)]
    fn get_role_admin(&self, role: [u8; 32]) -> Result<[u8; 32], Error>;

    /// @dev Grants `role` to `account`.
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    /// Requirements:
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    fn grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Revokes `role` from `account`.
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    /// Requirements:
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    fn revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Revokes `role` from the calling account.
    /// Roles are often managed via {grantRole} and {revokeRole}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    /// If the calling account had been granted `role`, emits a {RoleRevoked}
    /// event.
    /// Requirements:
    /// - the caller must be `account`.
    #[ink(message)]
    fn renounce_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;
}
