// Generated with Sol2Ink v0.3.0
// https://github.com/Supercolony-net/sol2ink

#[brush::contract]
pub mod erc_20 {
    use brush::traits::AccountId;
    use ink::prelude::string::String;
    use ink_storage::Mapping;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    pub enum Enum {
        FIRST,
        SECOND,
    }

    #[derive(Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Struct {
        field_1: u128,
        field_2: u128,
    }
    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct erc_20 {
        balances: Mapping<AccountId, u128>,
        allowances: Mapping<(AccountId, AccountId), u128>,
        total_supply: u128,
        name: String,
        symbol: String,
    }

    impl erc_20 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            todo!()
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            todo!()
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            todo!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u128 {
            todo!()
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u128 {
            todo!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u128) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            todo!()
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u128) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u128) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn decrease_allowance(&mut self, spender: AccountId, subtracted_value: u128) -> bool {
            todo!()
        }

        fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            todo!()
        }

        fn _mint(&mut self, account: AccountId, amount: u128) {
            todo!()
        }

        fn _burn(&mut self, account: AccountId, amount: u128) {
            todo!()
        }

        fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
            todo!()
        }

        fn _spend_allowance(&mut self, owner: AccountId, spender: AccountId, amount: u128) {
            todo!()
        }

        fn _before_token_transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            todo!()
        }

        fn _after_token_transfer(&mut self, from: AccountId, to: AccountId, amount: u128) {
            todo!()
        }
    }
}
