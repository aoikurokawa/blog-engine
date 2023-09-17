# Run an ethereum validator node

## Introduction
Since [the merge](https://ethereum.org/en/roadmap/merge/), Ethereum transitioned from Proof of Work to Proof of Stake consensus algorithm. Recently, one testnet called Holesky should be launched at Sep, 15th but unfortunately it [failed](https://twitter.com/protolambda/status/1702691543629328474). So instead I will introduce how to run a validator on Goerli testnet step by step. Let's get started!  

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
