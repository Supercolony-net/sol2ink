## Summary
**Sol2Ink is a tool for easy migration from Solidity to Ink! and Rust**

As we are the builders in the Dotsama ecosystem and experts in ink! smart contracts, we help companies with their path to the Dotsama ecosystem.
One of our many tasks is to help projects and teams migrate their smart contracts from popular Solidity to Polkadot's ink!. During this process,
we found out that the transition process may be unnecessarily long, and if we had a tool that would transpile a Solidity file to Rust and ink!, 
we would save much time. And that is how the idea of Sol2Ink was born.

### Capabilities

Sol2Ink in its current state is able to parse compilable Solidity interfaces into ink! traits and compilable Solidity contracts into ink! contracts, while leveraging the power of [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts). Currently, Sol2Ink supports only single file contract transpiling, not supporting inheritance. The output of Sol2Ink is a folder with the ink! smart contract and a Cargo.toml.

Some errors may occur in this version of Sol2Ink and will be fixed in upcoming versions.
With some statements, a parsing error can occur and cause the member to be parsed incorrectly. This needs to be corrected by the user.
The program may panic while parsing uncompilable code. Future versions should bring more user-friendly errors.
Some expressions may be parsed incorrectly, while still creating compilable code (one known example is `typeof(uint).max` is parsed as `u128.max` instead of `u128::MAX`.
And of course, as with all programs, there are probably some hidden unknown bugs as well :)

Read more about how Sol2Ink works under the hood here.

### Future development

- [X] Sol2Ink CLI
- [ ] User friendly errors when transpiling uncompilable contract
- [ ] Parsing libraries
- [ ] Implement currently incorrectly parsed statements and expressions
- [ ] Ability to parse a whole Solidity project into ink! project
- [ ] Parse inheritance
- [ ] Sol2Ink Web Application with interface

### How to use it?

To run the application you will need to have installed Rust and run the nightly toolchain. ​
You can run the application with `cargo run contract.sol`.
The result will be stored in `contract/lib.rs` and the Cargo.toml file in `contract/Cargo.toml`.

### Examples

Examples are stored in the example folder, where we have the input Solidity file and the output Rust and Ink! file.
By running `cargo test`, we will transpile all of the examples stored in this folder. We have several example contracts from OpenZeppelin and two example contracts from Solang. These original contracts were not modified (except the OpenZeppelin contracts, where we added missing enums, events, structs, etc. from the respective interface file), and the outputs of Sol2Ink are not modified either.
