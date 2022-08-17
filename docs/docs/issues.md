---
sidebar_position: 5
title: Known issues
---

Here is a list of known issues which you may face using Sol2Ink:

- inability to parse libraries
- inability to parse uncompilable contracts
- calling functions with a value
- occasional incorrect parsing of selectors within brackets
- incorrect rewriting of fields inside structs extracted from a mapping
- binary operation in a function only performs the reading of the value, not the updating
- incorrectly allowing modifiers to take functions as parameters
- inability to parse inheritation
- inability to parse multi-file projects

We will fix these issues in the upcoming versions of Sol2Ink. Every time you use Sol2Ink to transpile your contract from Solidity to ink!, run the generated code by a human brain to get the best results!