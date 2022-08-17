---
sidebar_position: 2
title: Parsing an interface
---

First, we will describe the parsing of the interface since less work is done in this case. Once the program finds the interface definition, it will start parsing the interface and saves the comments that it parsed until this point. While parsing the interface, the program looks for the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments

Once the program reaches the end of the interface, it will move on to the assemble part, where it assembles an ink! trait from the parsed objects. The output file will contain the parsed interface as a trait and include the headers of all parsed members. Note that all functions return `Result` by default, but we will discuss this later when describing contract parsing.

We can then later create another file where we implement this trait and then implement it for our contract.