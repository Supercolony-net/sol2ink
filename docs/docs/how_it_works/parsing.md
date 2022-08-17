---
sidebar_position: 1
title: Parsing
---

This section will look at how Sol2Ink works under the hood.

### Parsing

After running it, the program will first parse the original file. The first phase will look for the contract or interface definition and parse all comments until finding the contract's definition. The ink! file does not need imports or pragma statements (although we will implement support for multi-file projects in later versions), so Sol2Ink skips them. Once Sol2Ink finds the contract or interface definition, we start parsing it.

### Note the following
- library parsing is not implemented yet
- inheritance parsing is not implemented yet, so Sol2Ink will skip everything after the `is` keyword
- if the parser fails to find a contract or interface definition, it will fail