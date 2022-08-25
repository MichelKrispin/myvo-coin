# Bitcoin

Very much a reproduction of the bitcoin paper.

## Introduction

Introduced by Satoshi Nakamoto.

## Transactions

An electronic coin is defined to be a chain of digital signatures.
By signing the hash of a previous transaction block with a private key and adding the signature and the public key of the receiver to the next block, one can transfer a coin.
Double-spending can't occur here, because there is only one long list of transactions.
Knowing this complete list, one can check that the coin is only transfered once.
And of course, a lot of participants must have the complete list, so each transactions is publicly anounced.

## Timestamp Server

A timestamp server works by adding a timestamp to each block that must be hashed.
Hence, all later appended blocks must have a later timestamp to prove that the previous data has existed earlier.

## Proof-of-Work

Every block contains a nonce field, that changes the hash for the complete block.
This nonce is changed until a hash is found that has `$n$` leading zeros and only hashes with the defined number of leading zeros will be accepted.
As Satoshi Nakamoto said:

> Proof-of-work is essentially one-CPU-one-vote.

This also means that the number of CPUs have to be equally distributed between the participants.
If one party controls more than half of the CPU power, then the network can't be trusted anymore.

The proof-of-work concept is completely based upon randomness (if the hash function is well designed) and the number of leading zeros increase or decrease the amount of work that has to be done.

## Network

The steps to run the network are:

1. New transactions are broadcast to all nodes.
2. Each node collects new transactions into a block.
3. Each node works on finding a difficult proof-of-work for its block.
4. When a node finds a proof-of-work, it broadcasts the block to all nodes.
5. Nodes accept the block only if all transactions in it are valid and not already spent.
6. Nodes express their acceptance of the block by working on creating the next block in the chain, using the hash of the accepted block as the previous hash

Nodes always accept the longest chain to be the correct one, i.e., the one to be trusted, and work on extending this longest chain.
Note, that there are multiple transactions in one block and that not all nodes have necessarily access to all transactions.
If they reach some nodes, at some point they will be inserted into a block and appended to the blockchain.
Also, if transactions are in a block that belongs to the shorter blockchains, they will be captured and used for computing another block (at least if they are still valid).

## Incentive

The Bitcoin convection is to add one transaction per block, the first one, to create a new coin owned by the creator of the block.
This way nodes have an incentive to compute new blocks as they receive new coins by adding blocks to the chain.
Additionally, this is the only possible way to distribute new coins.

Once a predetermined number of coins have entered the system, the creation is stopped and the incentive will transition completely to transaction fees.
Transaction fees are the difference between the output and input value of a transaction and will be added to the incentive value.

## Reclaiming Disk Space

As there are a lot of transactions in one block, the number of hashes will take a lot of disk space over time.
If a transaction is contained in a block down the road, one only wants to check whether it was valid, without saving every hash individually.
This is done by making use of a [Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree).

## Simplified Payment Verification

It is possible to verify payments without a copy of the full network node, but with a copy the block ehaders only.
Linking the transaction to a place in the chain, one can verify that the hashes up until now are correct.

## Combining and Splitting Value

Transaction have multiple inputs and at most two outputs.
This way, the change of the transaction can get back to the sender and can be made up of multiple received transactions.

## Privacy

All transactions have to be publicly anounced.
So the only way to maintain privacy is by keeping the public keys anonymous and using new keys for every transaction.

## Calculations

New coins can never be created out of thin air.
They always have to belong to someone and have to be transferred to a new owner.
This means that the only thing an attacker can achieve is to take back previously spent money by generating a faster block of transactions than all the other nodes.

This race can be modelled by a [simple random walk](/theory/probability.html#simple-random-walk), where the honest chain is extended by one block, is modelled by `$+1$`, and the other case, the attacker extending its chain by another block, is modelled by `$-1$`.

The problem setting is analogous to a Gambler's Ruin problem where a gambler with unlimited credit starts at a deficit and plays potentially an infinite number of trials to try to reach breakeven.
The probability is then

```math
q_z = \begin{cases}
    1,               & p \leq q, \\
    (\frac{q}{p})^z, & p \gt q,
\end{cases}
```

where `$p \in [0, 1], q = 1 - p$`, is the probability that an honest node finds the next block and the probability that the attacker finds the next block, respectively.
`$q_z$` is then the probability that the attacker will ever catch up after being `$z \in \mathbb{Z}$` blocks behind.
This indicates, that the probability of catchin up drops exponentially.
