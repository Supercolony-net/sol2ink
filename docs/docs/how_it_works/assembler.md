---
sidebar_position: 6
title: Assembling a contract
---

Sol2Ink has everything it needs; now, it needs to mix it. Here we will clarify what may not be obvious.

### Error

Each contract will contain the following error definition: 
```rust
#[derive(Debug, Encode, Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}
```
This error will be used as the error type when returning results from the contract functions.

### Storage

Openbrush simplifies the work with storage and allows the upgradeability of the storage; that is why we use the following approach. This approach will also streamline our future development when our contract uses multiple traits, etc. For now, we define a storage key inside the contract, the state variables in a struct which will use this storage key, and this struct itself is the member of the contract storage. The whole storage will look something like this:

```rust
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub value: u128,
}

#[ink(storage)]
#[derive(Default, SpreadAllocate, Storage)]
pub struct Contract {
    #[storage_field]
    data: Data,
}
```
Accessing the `value` state variables inside the contract looks like `self.data.value`. 

Sol2Ink will generate the functions of the contract inside the impl section. Note the following:

- the constructor will be called new and will have the `#[ink(constructor)]` attribute
- the constructor will be generated even if it is empty or does not exist in the original contract
- public/external messages will have the `#[ink(message)]` attribute
- private/internal functions will be prefixed with `_`
