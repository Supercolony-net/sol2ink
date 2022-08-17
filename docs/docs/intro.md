---
sidebar_position: 1
slug: /
title: Sol2Ink Documentation
sidebar_label: Getting started
---

# Sol2Ink Documentation

Welcome to Sol2Ink documentation. In this documentation, we will describe the capabilities of Sol2Ink, how the process works under the hood,
what issues you may face while using Sol2Ink, and you will see some examples of usage of Sol2Ink.

## What is Sol2Ink

Sol2Ink is a tool developed to ease the developers' transition from Solidity to ink!. Since we are the builders in the Dotsama ecosystem, we recognized a problem when some team wanted to develop their existing Solidity dapp in ink! smart contract language, the most annoying and time-consuming part of the development will be rewriting the Solidity code into Rust and ink!. Sol2Ink aims to decrease this time by transpiling the existing Solidity code into Rust and ink! code. So the dirty part of the job is automated, and now it is up to the developers to fix some language-specific issues while teaching how stuff works in ink!. Sol2Ink will save time!

### What you'll need

Sol2Ink is written in Rust, so that you will need Rust installed with the nightly toolchain. If this is satisfied, you will also need Sol2Ink, which you can get here. Another thing you will need is the Solidity file you want to transpile. And that's it! We can start transpiling now!
