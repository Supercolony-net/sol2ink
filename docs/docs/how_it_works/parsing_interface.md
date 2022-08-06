---
sidebar_position: 2
title: Parsing an interface
---

First we will describe parsing of interface, since less work is done in this case. Once the program finds interface definition, it will start parsing the interface and saves the comments which it parsed until this point. While parsing the interface, the program looks for the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments

Once the program reaches the end of the interface, it will move on to the assemble part, where it assembles an ink! trait from the parsed objects. The output file will contain the parsed interface in the form of trait and will contain the headers of all parsed members. Note that all functions return `Result` by default, but we will discuss this later when describing contract parsing.

We can then later create another file, where we implement this trait and then implement it for our contract.