---
sidebar_position: 2
title: Capabilities
---

Sol2Ink, in its current state, can parse compilable Solidity interfaces into ink! traits and compilable Solidity contracts into ink! contracts while leveraging the power of [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts). Currently, Sol2Ink supports only single file contract transpiling, not supporting inheritance. The output of Sol2Ink is a folder with the ink! smart contract and a Cargo.toml.

Some errors may occur in this version of Sol2Ink, and we will fix them in upcoming versions.
With some statements, a parsing error can occur and cause the member to be parsed incorrectly. The member's name will indicate the error, which will be S2I_INCORRECTLY_PARSED_MEMBER, and the user needs to correct it.
The program may panic while parsing uncompilable code. Future versions should bring more user-friendly errors.
Some expressions may be parsed incorrectly while still creating compilable code (one known example is `typeof(uint).max` is parsed as `u128.max` instead of `u128::MAX`.
And of course, as with all programs, there are probably some hidden, unknown bugs as well :)