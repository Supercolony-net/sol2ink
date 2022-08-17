---
sidebar_position: 3
title: Parsing a contract
---

Another case is parsing a contract, which will start if the program finds a contract definition. While parsing a contract, the program looks for the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments
- state variables
- constructor
- modifiers

### Parsing a function or a modifier

First, the program parses the function or a modifier body to a raw statement, which is a statement ending either with a curly bracket or with a semicolon. Sol2Ink will then parse these raw statements into actual Rust and ink! code in the final step, done this way, so the program knows when working with an expression, whether the expression is a constant, state variable, etc.

Once the program reaches the end of the contract, now it's time to parse the bodies of functions and modifiers.