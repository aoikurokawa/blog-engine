# Run an ethereum validator node

## Introduction

  Proof-of-stake (PoS) underlies Ethereum's [consensus mechanism](https://ethereum.org/en/developers/docs/consensus-mechanisms/). Since [the merge](https://ethereum.org/en/roadmap/merge/), Ethereum transitioned from [Proof of Work](https://ethereum.org/en/developers/docs/consensus-mechanisms/pow/) to [Proof of Stake](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/). 
  
  In a proof of stake system, staking serves a similar function to proof of work's mining, in that it's the process by which a network participant gets selected to add the latest batch of transactions to the blockchain and earn some crypto in exchange. In general proof of stake blockchains employ a network of "validators" who contribute - or "stake" - their own crypto in exchange for a chance of getting to validate new transaction, update the blockchain, and earn a reward. The validator takes on the duty of verifying that the new blocks spread across the network are legitimate, and at times, they also produce and disseminate new blocks. Should they attempt to deceive the network (like suggesting several blocks when they should only propose one, or by sending contradicting attestations), a portion or all of their staked ETH may be forfeited. 

  To become a validator in mainnet, you need to deposit 32 ETH. However we don't have any amount of that, instead we will try to launch a validator on testnet. 

### Ethereum Mainnet

  Mainnet is the primary public Ethereum production blockchain, where actual-value transactions occur on the distributed ledger.

  When people and exchanges discuss ETH prices, they're talking about Mainnet ETH.

### Ethereum Testnets

  Beyond Mainnet, public testnets exist. These networks are utilized by both protocol and smart contract developers to trial protocol changes and potential smart contracts in an environment similar to Mainnet prior to actual deployment. This can be likened to the distinction between production and staging servers. Historically, many testnets employed a permissioned proof-of-authority consensus method. In this setup, a select group of nodes are tasked with validating transactions and producing new blocks, effectively pledging their identity. On the other hand, some testnets use an open proof-of-stake consensus system, allowing anyone to experiment with validator operations, mirroring the Ethereum Mainnet approach.

  Currently, there are two public testnets that client developers are currently maintaining.

#### Sepolia

  [Sepolia](https://sepolia.dev/) is the advised primary testnet for app development. This network operates with a permissioned set of validators. Being relatively recent, it has a concise state and history.


#### Goerli

  Goerli is testnet for testing validating and staking. The Goerli network is open for users wanting to run a testnet validator. Stakers wanting to test protocol upgrades before they are deployed to mainnet should therefore use Goerli. However, the Goerli testnet is deprecated and will be replaced by [Holesovice](https://github.com/eth-clients/holesky) soon.
  
  Recently, Holesovice should be attempted to launch at Sep, 15th but unfortunately it [failed](https://twitter.com/protolambda/status/1702691543629328474). 


So in this blog, I will introduce how to run a validator on Goerli testnet step by step. Let's get started!  

## Pre-requisites


### Home Staker

- Operating System: 64-bit Linux, Mac OS X, Windows
- Processor: Intel Core i7-4770 or AMD FX-8310
- Memory: 8GB
- Storage: 100 GB
- Internet: Broadband internet connection (10 Mbps)
- Power: Uninterruptible power supply

### Digital Ocean Equivalent (cloud provider)

DigitalOcean Droplets are Linux-based virtual machines (VMs) that run on top of virtualized hardware. Each Droplet you create is a new server you can use, either standalone or as part of a larger, cloud-based infrastructure. 

- Operating System: 64-bit Linux, Mac OS X, Windows
- Processor: Intel Core i7-4770 or AMD FX-8310
- Memory: 8GB
- Storage: 160 GB
- Internet: Broadband internet connection (10 Mbps)
- Power: Uninterruptible power supply

To connect to your Droplet, you'll need to open a terminal.


Once the terminal is open, enter the following SSH command. Substitute in your Droplet's IP address after the @

```bash

ssh root@ip address

```

## Become a validator

### Download staking-deposit-cli

```bash
wget -c https://github.com/ethereum/staking-deposit-cli/releases/download/v2.6.0/staking_deposit-cli-33cdafe-linux-amd64.tar.gz -O - | tar -xz
```

## Execution client
- Reth
- https://paradigmxyz.github.io/reth/installation/source.html


## Consensus client
- Lighthouse

## References
- https://goerli.launchpad.ethstaker.cc/en/
- https://eth-clients.github.io/checkpoint-sync-endpoints/#goerli
- https://ethereum.org/en/developers/docs/networks/
