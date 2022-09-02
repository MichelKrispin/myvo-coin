# myvo-coin

My very own blockchain coin with notes on how that works.

## How it works

Described in detail [here](https://developer.bitcoin.org/devguide/transactions.html) or even better [here](https://learnmeabitcoin.com/technical/transaction-data).

### Sending

The steps for a transaction from Alice to Bob are then the following:

1. Bob _(receiver)_ generates a public/private key pair using [ECDSA](/theory/signatures.html#ellipctic-curve-digital-signature).
2. Bob _(receiver)_ sends Alice (sender) a [hash of the public key](#public-key-hash) (Public Key Hash).
3. Alice _(sender)_ creates the transaction with Bob's Public Key Hash as the output and an input that Alice owns.
4. Alice _(sender)_ broadcasts the transaction to the network.
5. The transaction will be inserted into a block and added to the blockchain.

### Transactions

How the parts of a transactions are designed.

---

| Transaction                               |
| ----------------------------------------- |
| n Input (referencing to previous outputs) |
| Output (the new owner)                    |
| Change (maybe a second output for change) |
| Hash (the hash of all previous data)      |

---

| Input                                                     |
| --------------------------------------------------------- |
| Sequence Number (input counter)                           |
| Previous Output (the reference)                           |
| Validator (to proof ownership over the referenced output) |

---

| Output                                                      |
| ----------------------------------------------------------- |
| Amount                                                      |
| Public Key Hash (a hash of a public key to proof ownership) |

---

| Validator                                                                                         |
| ------------------------------------------------------------------------------------------------- |
| Signature (signature of the transaction with the private key used for the output public key hash) |
| Public Key (the public key to validate the signature)                                             |

---

####

### Validating Input

As the sender of a transaction has to proof ownership over the used input, there has to exist some validation mechanism.

This mechanism is a Validator that contains a signature created with the private key that was used to generate the previous outputs Public Key Hash and the public key itself.

The signature is the result of signing most of the new transaction with the private key except the Validator itself or anything that contains the full public key.
Hence, the transactions id of the previous output, its output index, the corresponding Public Key Hash of that output and the Public Key Hash and coin amount of the current transaction will be signed.
This way no tampering can be done while sending the transaction over a network.

Using the public key of the Validator, one can validate that the previous Public Key Hash is correct and that the signature was created with the private key of the owner proofing ownership of the output.
