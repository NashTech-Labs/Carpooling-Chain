# Carpooling-Chain

Carpooling Chain is a decentralized blockchain solution built with the objective of eliminating the need for a central authority to book cabs. The blockchain backend for Carpooling chain is built using Substrate. The substrate is a next-generation framework for blockchain innovation. It comes with everything you need to build your blockchain. 

## Setting up your environment

### Installation of Rust

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
### Installation of Node and npm

To install node js we need following commands:

```
sudo apt update
```
Install Node.js from the repositories:

```
sudo apt install nodejs
```
Install npm:
```
sudo apt install npm
```
To check which version of Node.js you have installed after these initial steps, type:

```
nodejs -v
```
### Install express js

```
npm install
```
### Install yarn
```
npm install -g yarn
```
### Install Polkadot js API
```
yarn add @polkadot/api
```
## Building

```
git clone https://github.com/knoldus/Carpooling-Chain
cd Carpooling-Chain
cargo build --release

```
## To start local substrate node

```
./target/release/node-template --dev --tmp
```
## Run the App

```
cd app/src/
node index.js
```
Now open this link to add a new customer:

http://localhost:6069/add-customer

Now you can check whether the new customer has been added or not by going on the polkadot.js portal.

https://polkadot.js.org/apps/#/chainstate