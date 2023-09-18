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

  [Goerli](https://goerli.net/) is testnet for testing validating and staking. The Goerli network is open for users wanting to run a testnet validator. Stakers wanting to test protocol upgrades before they are deployed to mainnet should therefore use Goerli. However, the Goerli testnet is deprecated and will be replaced by [Holesovice](https://github.com/eth-clients/holesky) soon.
  
  Recently, Holesovice should be attempted to launch at Sep, 15th but unfortunately it [failed](https://twitter.com/protolambda/status/1702691543629328474). 


So in this blog, I will introduce how to run a validator on Goerli testnet step by step. Let's get started!  


## Pre-requisites

To run an ETH 2.0  node one needs:

- **Validator client**
Responsible for producing new blocks and attestations in the beacon chain and shard chains
In this time, I will use [Lighthouse]([https://github.com/paradigmxyz/reth](https://github.com/sigp/lighthouse/tree/stable)).

- **Beacon chain client**
Responsible for managing the state of the beacon chain, validator shuffling, and more.

- ETH 1 node
RethSupplies incoming validator deposits from the eth1 chain to the beacon chain client.
In this time, I will use [Reth](https://github.com/paradigmxyz/reth).


- ETH balance
Goerli ETH and some ETH for deposit transaction fees.


- Wallet
[Metamask](https://metamask.io/) installed.


### Machine Requirements

- Operating System: 64-bit Linux, Mac OS X, Windows
- Processor: Intel Core i7-4770 or AMD FX-8310
- Memory: 8GB
- Storage: 100 GB
- Internet: Broadband internet connection (10 Mbps)
- Power: Uninterruptible power supply

#### Digital Ocean Equivalent (cloud provider)

DigitalOcean Droplets are Linux-based virtual machines (VMs) that run on top of virtualized hardware. Each Droplet you create is a new server you can use, either standalone or as part of a larger, cloud-based infrastructure. 

- Operating System: 64-bit Linux, Mac OS X, Windows
- Processor: Intel Core i7-4770 or AMD FX-8310
- Memory: 8GB
- Storage: 160 GB
- Internet: Broadband internet connection (10 Mbps)
- Power: Uninterruptible power supply


## Run a node

### 1. Connect to Droplet
You'll need to open a terminal.

Once the terminal is open, enter the following SSH command. Substitute in your Droplet's IP address after the @.

![Droplet](https://github.com/aoikurokawa/blog/assets/62386689/96884732-83d3-4753-ab3f-af4d5451bee6)


```bash

ssh root@ip address

```

### 2. Install Dependencies
First, install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After Rust installation completes, try running to check whether rust installed successfully

```bash
cargo --verison

# return cargo 1.68.2
```

With Rust installed, install following dependencies relevant to your operating system:

**For Reth:**

```bash
apt-get install libclang-dev pkg-config build-essential
```

**For Lighthouse**

```bash
apt install -y git gcc g++ make cmake pkg-config llvm-dev libclang-dev clang
```
   
### 3. Build Reth
Visit [reth github](https://github.com/paradigmxyz/reth), then clone it. 

```bash
git clone https://github.com/paradigmxyz/reth

cd reth
```

Build Reth!!

```bash
cargo build --release
```

Compilation may take around 10 ~ 15 min. Installation was successful if `reth --help` displays the [command-line documentation](https://paradigmxyz.github.io/reth/cli/cli.html). 


### 4. Build Lighthouse
Visit [lighthouse github](https://github.com/sigp/lighthouse), then clone it.

```bash
git clone https://github.com/sigp/lighthouse.git

cd lighthouse
```

Build lighthouse!!

```bash
git checkout stable

make
```

### 5. Create a JWT secret file
A JWT secret file is used to secure the communication between the execution client and the consensus client. In this step, we will create a JWT secret file which will be used in later steps.

```bash
sudo mkdir -p /secrets
openssl rand -hex 32 | tr -d "\n" | sudo tee /secrets/jwt.hex
```


### 6. Running the Reth Node
Run this following command to run Reth node. 

```bash
RUST_LOG=info reth node \
    --authrpc.jwtsecret /secrets/jwt.hex \
    --authrpc.addr 127.0.0.1 \
    --authrpc.port 8551 \
    --chain goerli
```


### 7. Set up a beacon node using Lighthouse
In this step, we will set up a beacon node. Use the following command to start a beacon node that connects to the execution node.


```bash
lighthouse bn \
  --network goerli \
  --execution-endpoint http://localhost:8551 \
  --execution-jwt /secrets/jwt.hex \
  --checkpoint-sync-url https://sync-goerli.beaconcha.in/ \
  --http
```

### 8. Download staking-deposit-cli to create validator keys
The Etereum Foundation procides the staking-deposit-cli for creating validator keys. Download and run the staking-deposit-cli with the command:

```bash
wget -c https://github.com/ethereum/staking-deposit-cli/releases/download/v2.6.0/staking_deposit-cli-33cdafe-linux-amd64.tar.gz -O - | tar -xz

cd taking_deposit-cli-33cdafe-linux-amd64

./deposit new-mnemonic
```

### 9. Import validator keys to Lighthouse
Run the following command to import validator keys.

```bash
lighthouse --network goerli account validator import --directory $HOME/staking_deposit-cli-33cdafe-linux-amd64/validator_keys
```


### 10. Start Lighthouse validator client
After the keys are imported, the user can start performing their validator duties by starting the Lighthouse validator client.

```bash
lighthouse vc --network goerli --suggested-fee-recipient YourFeeRecipientAddress
```

### 11. Get some GöETH to submit deposit
  
  If you have not installed [metamask](https://metamask.io/), or other wallet, you need to install it first. 

  After installing the wallet, you have to aquire some GOETH. so visit one of these faucet website. 

- [QuickNode Goerli Faucet](https://faucet.quicknode.com/drip)
- []()

Visit [this website,](https://goerli.launchpad.ethereum.org/en/) then deposit GOETH to become a validator.


## References
