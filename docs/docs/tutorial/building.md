---
sidebar_position: 2
title: Building the ink! smart contract
---

To build the ink! smart contract we will need [cargo-contract](https://github.com/paritytech/cargo-contract). So if we satisfy this condition, we will navigate to the generated folder `ERC20` and call cargo contract build. The contract will start building; we will wait for a while and...

It fails! So let's look at the issue.

The first issue looks like this:

![issue1](https://user-images.githubusercontent.com/43150707/183226415-17fa4232-9b38-4302-b8b8-4357c64ab740.png)

We described this issue before; the Solidity expression `type(uint256).max` is parsed as `u128.max`. The correct form is `u128::MAX`, so now with no issues, we will try to build it again.

And we failed again.

![issue2](https://user-images.githubusercontent.com/43150707/183226417-73ec8940-0800-4e70-9bee-91421def1ba2.png)

These issues have the same reason; we want to return a String which is the type that can not be copied. Fixing both of these issues is simple; we will return the clones of these strings by calling `.clone()` on them. Now when we build, everything works! Congratulations!

### Warnings

You could have noticed some warnings. The cause of these warnings is that two functions have parameters that are unused inside that function. It is not an issue, but if we want to remove these warnings, we will add `_` to the front of the names of these parameters, implying that those parameters are unused. 

### More things to notice

Some things are still not implemented in Sol2Ink (but definitely on the radar!). Let's look at what was not parsed right in our ERC-20 file.
We notice a comment on line 240 saying, `Please handle unchecked blocks manually`. Two lines beneath, we see the same comment but with inversed arrows, meaning that everything between these two comments is initially in the unchecked block. We don't need to care about this and can remove the comments. We can find the same comment on lines 270, 298, 331, and 392, where we do the same thing. And that's it! Now it is the developer's job to optimize the contract for Rust and ink!, but the dirty work is already behind us!