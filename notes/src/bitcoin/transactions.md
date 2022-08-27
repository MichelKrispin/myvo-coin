## Transactions

Described in detail [here](https://developer.bitcoin.org/devguide/transactions.html).

And even better [here](https://learnmeabitcoin.com/technical/transaction-data).

The idea is that the receiver gives the destination address of the transaction to the sender, who then creates the transaction.

### Sending

The steps for a transaction from Alice to Bob are then the following:

1. Bob _(receiver)_ generates a public/private key pair using [ECDSA](/theory/signatures.html#ellipctic-curve-digital-signature).
2. Bob _(receiver)_ sends Alice (sender) a [hash of the public key](#public-key-hash) (_Public Key Hash_).
3. Alice _(sender)_ creates the transaction with Bob's _Public Key Hash_ as the output and an input that Alice owns.
4. Alice _(sender)_ broadcasts the transaction to the network.
5. The transaction will be inserted into a block and added to the blockchain.

### Validating Input

As the sender of a transaction has to proof ownership over the used input, there has to exist some validation mechanism.

This mechanism is a _Signature Script_ that contains a signature created with the private key that was used to generate the previous outputs _Public Key Hash_ and the public key itself.

The signature is the result of signing most of the new transaction with the private key except the _Signature Script_ itself or anything that contains the full public key.
Hence, the transactions id of the previous output, its output index, the corresponding _Public Key Hash_ of that output and the _Public Key Hash_ and coin amount of the current transaction will be signed.
This way no tampering can be done while sending the transaction over a network.

Using the public key of the _Signature Script_, one can validate that the previous _Public Key Hash_ is correct and that the signature was created with the private key of the owner proofing ownership of the output.

### Architecture

How each described element is structured.

---

| Transaction    |
| -------------- |
| Input          |
| Output         |
| Transaction ID |

---

| Input            |
| ---------------- |
| Sequence Number  |
| Previous Output  |
| Signature Script |

---

| Output            |
| ----------------- |
| Amount            |
| _Public Key Hash_ |

---

| Signature Script |
| ---------------- |
| Signature        |
| Public Key       |

---
