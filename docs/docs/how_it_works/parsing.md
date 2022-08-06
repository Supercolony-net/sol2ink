---
sidebar_position: 1
title: Parsing
---

In this section we will take a look at how Sol2Ink works under the hood.

### Parsing

First thing the program will do after running it is parse the original file. In the first phase it will look for the contract or interface definition. It will also parse all comments which it finds until finding the definition of the contract. Imports and pragma statement are not needed in the ink! file (altough support for multi-file projects will be implemented in later versions), so they are skipped. Once we find the contract or interface definition, we start parsing it.

### Note the following
- library parsing is not implemented yet
- inheritance parsing is not implemented yet, so everything after `is` keyword will be skipped
- if the parser fails to find contract or interface definition, it will fail