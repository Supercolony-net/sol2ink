---
sidebar_position: 4
title: Parsing functions
---

At this point the parser knows all the members of the contract, and has the information about what the Solidity statements of each function and modifier look like, in the form of `Statement::Raw`, so we will parse them into correct form. For every statement we will check, which statement does it fit.

### _

This implies that at this point body of modifier should be executed. So the program parses it as `Statement::ModifierBody`.

### Return statement

Mission is simple - to return a value. The functions of generated contract will all return `Result<T, Error>`, where `T` is the return type of the function (`()` if the function has no return type). We wrap the output in a result because if we want to revert a call, we need to return error. And that is of course possible, if the return type is of result. The error type returned is declared in the final contract, but more on that later.

### Require

Require statements are not available in Rust and ink!, so Sol2Ink will parse them as an if statement, and returning an error. But require statement requires the condition to be true, so Sol2Ink will parse it as an inverted condition. Meaning `require(true)` will be parsed as 
```Rust
if(false) {
    return Err(Error(String::new()))
}
```
If the error message was defined in the Solidity contract, Sol2Ink will use this error message in the ink! contract as well, but if it was not provided, Sol2Ink will provide its own error message.

### Emit event

Here we just need to note, that events in ink! are structs with `ink[(event)]` attribute. So emitting an event is just calling of `emit_event` function, providing a new struct with the desired paramters.

### Ternary operator

Ternary operator does not exist in Rust, so they are parsed as an if/else block.

### Binary operation

Binary operations ++ and -- are not available in Rust, so we parse them as addition or subtraction of 1. Depending on if the operation was a prefix or suffix operation, we will do the incrementation/subtraction before or after reading of the value.

### Loops

- For loops are parsed to while loops, with the incrementation happenning at the end of the loop. 
- Do/while loops are parsed as loop with a check of the condition at the end of the block. 
- While loops are parsed normally as while loops 

### Unchecked blocks

A comment to check if everything is correct is inserted at the beginning and at the end of the unchecked block.

### Try/catch blocks

Try block is parsed as `if true` block, adding the original try statement as a comment. Catch blocks are parsed as `else if false` blocks, adding the original catch statement as a comment.

### Assembly blocks

Assembly blocks are fully parsed as comments.

All other statements are parsed as expected:
- declarations
- comments
- conditional blocks and one line conditions
- assignments
- function calls

If some specific statement is reached which Sol2Ink is unable to parse yet, it will be parsed as a comment with a notice `Sol2Ink Not Implemented yet`