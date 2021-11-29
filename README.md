# Carpooling-Chain

Carpooling Chain is a decentralized blockchain solution built with the objective of eliminating the need for a central authority to book cabs. The blockchain backend for Carpooling chain is built using Substrate. The substrate is a next-generation framework for blockchain innovation. It comes with everything you need to build your blockchain. 

## Setting up your environment

### Rustup.rs

Building this project requires [rustup](https://rustup.rs/), version 1.8.0 or more recent.
If you have an older version, run `rustup self update`.

To install on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To install on other systems, run:

```
curl https://sh.rustup.rs -sSf | sh
```

This will also download the current stable version of Rust, which this project won’t use.
To skip that step, run instead:

```
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none
```

## Building

### Normal Build

```
git clone https://github.com/knoldus/Carpooling-Chain
cd Carpooling-Chain
cargo build
```