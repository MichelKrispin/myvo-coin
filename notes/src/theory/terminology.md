# Terminology

## Distributed Ledger

- An "append-only" transaction store distributed across machines (immutability).
- A new transaction might reverse a previous transaction, but both remain part of the ledger.

## Blockchain

- A distributed ledger structured into a linked list of blocks.
- Each block contains an ordered set of transactions.
- Use cryptographic hashes to secure the link from a block to its predecessor.

## Blockchain System

Consists of

- A blockchain network of nodes.
- A blockchain data structure.
  - For the ledger replicated across the blockchain network.
  - Full nodes hold a full replica of the ledger.

## Public Blockchain

A blockchain system that

- Has an open network.
  - Nodes can join and leave without requiring permissions from anyone.
- All full nodes can verify new transactions and blocks.
- Incentive mechanism to ensure the correct operation.
  - Valid transactions are processed and included in the ledger and invalid transactions are rejected.

## Smart Contracts

- Programs deployed as data and executed in transactions on the blockchain.
- Blockchain can be a computational platform (more than a simple distributed database).
  - Code is deterministic and immutable once deployed.
- Can invoke other smart contracts.
- Can hold and transfer digital assets.

## Cryptocurrencies

- Base currency of blockchains.
- Symbiotic relationship.
  - Cryptocurrency enables the incentive mechanism for blockchain operations.
  - Blockchain keeps track of the ownership of portions of that currency.

## Digital Tokens

- Created and exchanged using smart contracts.
- Represent assets

## (Non-Fungible) Fungible Tokens

|                | Fungible        | Non-Fungible |
| -------------- | --------------- | ------------ |
| Digital Tokens | Interchangeable | Immutable    |
| Main Concern   | How many?       | Which ones?  |
