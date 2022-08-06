---
sidebar_position: 5
title: Known issues
---

Here is a list of known issues, which you may face using Sol2Ink:

- unability to parse libraries
- unability to parse uncompilable contracts
- calling functions with a value
- ocasional incorrect parsing of selectors within brackets
- incorrect rewriting of fields inside structs extracted from a mapping
- binary operation in a function call only counts with reading of the value, not with updating
- incorrectly allowing modifiers to take functions as parameters
- unability to parse inheritation
- unability to parse multi-file projects

These issues will be fixed in the upcoming versions of Sol2Ink. Every time you use Sol2Ink to transpile your contract from Solidity to ink!, run the generated code by a human brain to get the best results!