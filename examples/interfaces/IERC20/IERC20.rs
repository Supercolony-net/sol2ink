// Generated with Sol2Ink v0.3.0
// https://github.com/Supercolony-net/sol2ink

use brush::traits::AccountId;

///@dev Emitted when `value` tokens are moved from one account (`from`) to
///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`).
///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`).
///@dev Emitted when `value` tokens are moved from one account (`from`) to another (`to`). Note that `value` may be zero.
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    value: u128,
}

///@dev Emitted when the allowance of a `spender` for an `owner` is set by
///@dev Emitted when the allowance of a `spender` for an `owner` is set by a call to {approve}. `value` is the new allowance.
#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: u128,
}

#[brush::wrapper]
pub type ERC20Ref = dyn ERC20;

#[brush::trait_definition]
pub trait ERC20 {
    ///@dev Returns the amount of tokens in existence.
    #[ink(message)]
    fn total_supply(&self) -> u128;

    ///@dev Returns the amount of tokens owned by `account`.
    #[ink(message)]
    fn balance_of(&self, account: AccountId) -> u128;

    ///@dev Moves `amount` tokens from the caller's account to `to`.
    ///@dev Moves `amount` tokens from the caller's account to `to`.
    ///@dev Moves `amount` tokens from the caller's account to `to`. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Moves `amount` tokens from the caller's account to `to`. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Moves `amount` tokens from the caller's account to `to`. Returns a boolean value indicating whether the operation succeeded. Emits a {Transfer} event.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, amount: u128) -> bool;

    ///@dev Returns the remaining number of tokens that `spender` will be
    ///@dev Returns the remaining number of tokens that `spender` will be allowed to spend on behalf of `owner` through {transferFrom}. This is
    ///@dev Returns the remaining number of tokens that `spender` will be allowed to spend on behalf of `owner` through {transferFrom}. This is zero by default.
    ///@dev Returns the remaining number of tokens that `spender` will be allowed to spend on behalf of `owner` through {transferFrom}. This is zero by default.
    ///@dev Returns the remaining number of tokens that `spender` will be allowed to spend on behalf of `owner` through {transferFrom}. This is zero by default. This value changes when {approve} or {transferFrom} are called.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> u128;

    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens.
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens.
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race condition is to first reduce the spender's allowance to 0 and set the
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race condition is to first reduce the spender's allowance to 0 and set the desired value afterwards:
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race condition is to first reduce the spender's allowance to 0 and set the desired value afterwards: https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race condition is to first reduce the spender's allowance to 0 and set the desired value afterwards: https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
    ///@dev Sets `amount` as the allowance of `spender` over the caller's tokens. Returns a boolean value indicating whether the operation succeeded. IMPORTANT: Beware that changing an allowance with this method brings the risk that someone may use both the old and the new allowance by unfortunate transaction ordering. One possible solution to mitigate this race condition is to first reduce the spender's allowance to 0 and set the desired value afterwards: https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729 Emits an {Approval} event.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: u128) -> bool;

    ///@dev Moves `amount` tokens from `from` to `to` using the
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's allowance.
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's allowance.
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's allowance. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's allowance. Returns a boolean value indicating whether the operation succeeded.
    ///@dev Moves `amount` tokens from `from` to `to` using the allowance mechanism. `amount` is then deducted from the caller's allowance. Returns a boolean value indicating whether the operation succeeded. Emits a {Transfer} event.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool;
}
