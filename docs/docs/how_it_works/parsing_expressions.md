---
sidebar_position: 5
title: Parsing expressions
---

Another step of parsing a statement is parsing each expression. Here the program will decide how to parse each expression inside a statement.

### Basics

- Literals are parsed without any modifications
- Specific expressions like `address(0)`, `msg.sender` or `msg.value` are parsed in their ink! form
- Solidity types are converted to Rust/ink! types

### Enclosed expressions

Parsing an enclosed expression like `((1+2)+3)+4` may be problematic. That is why Sol2Ink first removes the brackets by extracting expressions in parentheses, substituting them with a simple expression, which is then parsed on its own and returning them as `Expression::Enclosed`. Here is an example of how Sol2Ink would parse the before-mentioned expression:

```rust
let extracted = self.extract_parentheses("((1 + 2) + 3) + 4", false);
assert_eq!(extracted.0, String::from("___0___ + 4"));
assert_eq!(extracted.1, 1);
assert_eq!(extracted.2.get("___0___"), Some(
Expression::Arithmetic(Expression::Enclosed(Expression::Arithmetic(Expression::Arithmetic(1, 2, Operation::Add)), 3, Operation::Add), 4, Operation::Add)));
```

### Hex string

Expressions like `hex"0000_0000_0000_0000"` are converted to a call of `&hex::decode` function.

### type(T).f / type(T)

These expressions are parsed as expected, except `type` is changed to `type_of` since `type` is a keyword in rust. If the original expression were a cast, the `type_of` call would be omitted, and the expression will be parsed as a cast.

### Mapping/array manipulation

The only notable thing here is that Sol2Ink will not use an indexed approach to data; it will instead use `unwrap_or_default()` in case of reading and `insert` in case of writing from or to a mapping or array (which it parses as a vec).

All other expressions are parsed as expected:

- struct initializations
- function calls
- arithmetic operations
- logical operations

After Sol2Ink parses everything, it will assemble the final ink! contract.