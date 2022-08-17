---
sidebar_position: 4
title: Parsing functions
---

Now the parser knows every contract member and has the information about what the Solidity statements of functions and modifiers look like, in the form of `Statement::Raw`, so we will parse them into the correct format. For every statement, we will check which statement does it fit.

### _

`_` implies that at this point body of the modifier should be executed. So the program parses it as `Statement::ModifierBody`.

### Return statement

The mission is simple - to return a value. The functions of the generated contract will always return `Result<T, Error>`, where `T` is the return type of the function (`()` if the function has no return type). We wrap the output in a result because if we want to revert a call, we need to return an error. And that is, of course, possible if we return `Result`. The error type returned in the Result is declared in the final contract, but more on that later.

### Require

Require statements are not available in Rust and ink!, so Sol2Ink will parse them as an if statement and return an error. But the require statement requires the condition to be true so that Sol2Ink will parse it as an inverted condition. Meaning `require(true)` will be parsed as 
```Rust
if(false) {
    return Err(Error(String::new()))
}
```
If the error message were defined in the Solidity contract, Sol2Ink would use this error message in the ink! contract as well, but if it were not provided, Sol2Ink would provide its error message.

### Emit event

Here we need to note that events in ink! are structs with the `ink[(event)]` attribute. So emitting an event is just calling of `emit_event` function, providing a new struct with the desired parameters.

### Ternary operator

The ternary operator does not exist in Rust, so they are parsed as an if/else block.

### Binary operation

Binary operations ++ and -- are not available in Rust, so we parse them as addition or subtraction of 1. Depending on if the operation were a prefix or suffix operation, we would do the incrementation/subtraction before or after reading the value.

### Loops

- For loops are parsed to while loops, with the incrementation happening at the end of the loop. 
- Do/while loops are parsed as a loop with a condition check at the end of the block. 
- While loops are parsed as while loops 

### Unchecked blocks

A comment to check if everything is correct is inserted at the unchecked block's beginning and end.

### Try/catch blocks

Try block is parsed as an `if true` block, adding the original try statement as a comment. Catch blocks are parsed as `else if false` blocks, adding the original catch statement as a comment.

### Assembly blocks

Sol2Ink parses assembly blocks as comments.

All other statements are parsed as expected:
- declarations
- comments
- conditional blocks and one-line conditions
- assignments
- function calls

If So2Ink reaches some specific statement that it cannot parse yet, it will parse it as a comment with a notice `Sol2Ink Not Implemented yet`.