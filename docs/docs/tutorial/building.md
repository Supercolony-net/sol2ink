---
sidebar_position: 2
title: Building the ink! smart contract
---

To build the ink! smart contract we will need [cargo-contract](https://github.com/paritytech/cargo-contract). So if we satisfy this condition, we will navigate to the generated folder `ERC20` and call cargo contract build. The contract will start building, we will wait for a while and...

It fails! So let's look at the issue.

First issue looks like this:

///

This issue was described before, Solidity expression is written as `type(uint256).max` and gets parsed as `u128.max`. The correct form is `u128::MAX`, so now with no issues, we will try to build it again.

And we failed again.

///

These issues have the same reason, we want to return String, which can not be copied. Fixing both of these issues is simple, we will return the clones of these strings by calling `.clone()` on them. Now when we build, everything works! Congratulations!

### Warnings

You could have noticed some warnings. The cause of these warnings is that two of the functions have parameters, which are unused inside that function. This is not an issue, but if we want to remove these warnings, we will simply add `_` to the front of the names of these parameters, which will imply that those parameters are unused. 

### More things to notice

There are still some things, which are not implemented in Sol2Ink (but definitely on the radar!). Let's have a look at what was not parsed right in our ERC-20 file.
First thing we notice is a comment on line 240 saying `Please handle unchecked blocks manually`. Two lines above we see the same comment, but with inversed arrows, meaning that everything between these two comments is originally in the unchecked block. We don't really need to care about this, so we can just remove the comments. We can find the same comment on lines 270, 298, 331 and 392 where we do the same thing. And that's it! Now it is up to the developer to optimize the contract for Rust and ink!, but the dirty work is already done!