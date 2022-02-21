Xpanse Smart Contract
==================

A auto-compouding strategy for ETH-AURORA farm on ref finance.

![Xpanse](https://user-images.githubusercontent.com/85037852/154997062-ef1e6ebe-c057-4584-a610-5f02955feb0b.jpg)

Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]

Commands for using this strategy from near-cli
===========

- Deposit token to strategy
```near call exchange.ref-dev.testnet mft_transfer_call '{"token_id": ":107", "receiver_id": "xpanse-strategy.testnet","amount":"1000000000000000000", "msg":""}' --account_id <sender-id>.testnet --depositYocto '1'  --gas '300000000000000'```

- Withdraw token from strategy
```near call xpanse-strategy.testnet '{"amount": "1000000000000000000"}' --account_id <sender-id> --depositYocto '1'  --gas '300000000000000'```

- Harvesting calls
```
near call xpanse-strategy.testnet harvesting_step_1 --account-id <sender-id>.testnet --gas '300000000000000'
near call xpanse-strategy.testnet harvesting_step_2 --account-id <sender-id>.testnet --gas '300000000000000'
near call xpanse-strategy.testnet harvesting_step_3 --account-id <sender-id>.testnet --gas '300000000000000'
near call xpanse-strategy.testnet harvesting_step_4 --account-id <sender-id>.testnet --gas '300000000000000'
near call xpanse-strategy.testnet harvesting_step_5 --account-id <sender-id>.testnet --gas '300000000000000'
near call xpanse-strategy.testnet harvesting_step_6 --account-id <sender-id>.testnet --gas '300000000000000'
```

Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
