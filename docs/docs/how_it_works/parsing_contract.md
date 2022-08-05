---
sidebar_position: 3
title: Parsing a contract
---

Another case is parsing of a contract, which will start if the program finds a contract definition. While parsing a contract, the program looks for the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments
- state variables
- constructor
- modifiers

### Parsing a function or a modifier

In this first step of parsing, the program parses the body of a function or a modifier to a raw statement, which is a statement which ends either with a curly bracket or with a semicolon. These raw statements will be then parsed into actual Rust and ink! code in the final step, and it is done this way, so the program knows when working with an expression, whether the expression is a constant, state variable, etc.

Once the program reaches the end of the contract, now it's time to parse the bodies of functions and modifiers.