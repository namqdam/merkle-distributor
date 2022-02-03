Merkle Distributor
===================

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/namqdam/merkle-distributor)

A program for distributing tokens efficiently via uploading a [Merkle root](https://en.wikipedia.org/wiki/Merkle_tree).

This program is largely based off of [Uniswap's Merkle Distributor](https://github.com/Uniswap/merkle-distributor).

Prerequisites
=============

If you're using Gitpod, you can skip this step.

* Make sure Rust is installed per the prerequisites in [`near-sdk-rs`](https://github.com/near/near-sdk-rs).
* Make sure [near-cli](https://github.com/near/near-cli) is installed.

Explore this contract
=====================

The source for this contract is in `contracts/merkle-distributor/lib.rs`.

Building this contract
======================

Run the following, and we'll build our rust project up via cargo. This will generate our WASM binaries into our `target/` directory. This is the smart contract we'll be deploying onto the NEAR blockchain later.

```bash
cargo build --target wasm32-unknown-unknown --release
```

Testing this contract
=====================

We have some tests that you can run. For example, the following will run our simple tests to verify that our contract code is working.

```bash
cargo test -- --nocapture
```

Using this contract
===================

### Quickest deploy

You can build and deploy this smart contract to a development account. [Dev Accounts](https://docs.near.org/docs/concepts/account#dev-accounts) are auto-generated accounts to assist in developing and testing smart contracts. Please see the [Standard deploy](#standard-deploy) section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/merkle_distributor.wasm
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-xxx

In this instance, the account is `dev-xxx`. A file has been created containing a key pair to
the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an
environment variable containing this development account id and use that when copy/pasting commands.
Run this command to set the environment variable:

```bash
source neardev/dev-account.env
export ACCOUNT_ID=aabbcc
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:

```bash
echo $CONTRACT_NAME
echo $ACCOUNT_ID
```

The next command will initialize the contract using the `new` method:

```bash
near call $CONTRACT_NAME initialize '{"owner_id": "aaa", "token_id": "bbb", "merkle_root": "bbb"}' --accountId $CONTRACT_NAME
```

To claim:

```bash
near call $CONTRACT_NAME claim '{"index": 0, "amount": 100, "proof": ["xxx"]}' --accountId $ACCOUNT_ID
```

Notes
=====

* To generate merkle_tree, using command

```bash
yarn generate-merkle-root -i scripts/example.json -o merkle.json
```

* The output tree will be located in merkle.json
* Using `merkleRoot` and `proof` for calling contract using near-cli
