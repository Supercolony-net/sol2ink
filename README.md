## Summary
**Sol2Ink is a tool for easy migration from Solidity to Ink! and Rust**

As we are the builders in the Dotsama ecosystem and experts in ink! smart contracts, we help companies with their path to the Dotsama ecosystem.
One of our many tasks is to help projects and teams migrate their smart contracts from popular Solidity to Polkadot's ink!. During this process,
we found out that the process of transition may be unnecesarilly long, and if we had a tool which would transpile a Solidity file to Rust and ink!, 
we would save a lot of time. And that is how the idea of Sol2Ink was born. 

### Capabilities

Sol2Ink in its current state is able to parse Solidity interfaces into ink! traits, while also leveraging the power of 
[OpenBrush](https://github.com/Supercolony-net/openbrush-contracts).

### Future development

In the future Sol2Ink will be able to parse whole Solidity smart contract codebases and even projects into Rust and Ink! The complexity of parsable
inputs will grow by time, beginning with interfaces through regular smart contracts up to inheritance, storage manipulation, delegate calls, etc.

### Roadmap

- Sol2Ink cli tool for simple smart contract parsing
- Documentation and a website with guides on how to use Sol2Ink
- Inheritance parsing
- Delegate calls and storage manipulation parsing
- Dependencies parsing and PSP usage
- Sol2Ink	web application interface
- Maintenance	and integration of new ecosystem standards and updates

### How to use it?

We run the application with

`cargo run input_file_name.sol`

And the result file will be saved in `output.rs` file.

### Examples

Examples are stored in the example folder, where we have the input Solidity file and the output Rust and Ink! file.
Here we can find a folder with interface transpiling example and a folder with contracts transpiling example.
All examples are taken from OpenZeppelin repository, and then transpiled to ink! by Sol2Ink. 
Sol2Ink can at the time handle simple smart contracts parsing without inheritance, that is why in contracts we removed
inheritance, and changed the calls to the Context functions to the basic retrieving of this data.
