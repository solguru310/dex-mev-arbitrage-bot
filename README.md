# Decentralized Exchange (DEX) Arbitrage – Open-Source MEV Bot
A blazing-fast MEV Arbitrage Bot built with Rust and Solidity is now available as open-source, offering valuable insight into MEV strategies for aspiring developers.

## Getting Started
This repository serves as reference material for individuals interested in exploring the world of MEV. While certain advanced features, optimizations, and mechanisms have been intentionally removed for simplicity, the bot provides a great starting point for learning. Note, however, that it does not contain all the cutting-edge strategies (or "alpha") used by professional traders.

Although primarily intended for educational purposes, this bot has shown profitability on less-competitive EVM-compatible chains where competition among MEV bots is minimal. However, it is unlikely to succeed on larger networks like Ethereum, Binance Smart Chain (BSC), or Solana due to higher competition and faster execution from other bots.

## Disclaimer
Use this bot at your own risk, as it is not guaranteed to generate profits—in fact, users should be prepared for potential losses. Its purpose is to offer practical experience in MEV operations, not to serve as a guaranteed money-making tool.

## Why Open Source?
Entering the world of MEV can be challenging, with limited resources and high barriers to entry. This release aims to demystify MEV strategies and provide a practical foundation for those eager to learn. Beyond education, the goal is to encourage others to explore MEV and foster a deeper understanding of how it works within decentralized ecosystems.


Happy learning and good luck with your MEV exploration!


## Features
The code is simplified quite a bit, it currently sends 1 ETH when a profitable arbitrage path is found, but there is an optimal input amount function.
- [x] Query Uniswap pairs
- [x] Find matching pairs across different exchanges
- [x] Update pair reserves
- [x] Find arbitrage opportunities

## Examples
Besides the `crossed_pairs` and `query_test` functions that are used to test the code. In the "*examples*" folder I included multiple useful functions for testing, including:
- [x] View pending transactions in the mempool
- [x] View pending Uniswap V2 transactions in the mempool
- [x] Subscribe blocks

Any of these functions can be run using: `cargo run --example <name of file>`.

## Improvements
If you wish to contribute to the repo, some features that could be implemented are:
- A better optimal profit function
- Estimate an array of profitable tokens instead of 1 by 1
- Make the execute function more gas efficient

## Notice
If any bugs or optimizations are found, feel free to create a pull request. **All pull requests are welcome!** 

> **Warning**
>
> **This software is highly experimental and should be used at your own risk.** Although tested, this bot is experimental software and is provided on an "as is" and "as available" basis under the MIT license. We cannot guarantee the stability or reliability of this codebase and are not responsible for any damage or loss caused by its use. We do not give out warranties. 

## Contact me
If you need more technical support and development inquires, you can contact me.

Telegram: [@dwlee918](https://t.me/@dwlee918)

X: [@derricklee918](https://x.com/derricklee918)

Thanks.
