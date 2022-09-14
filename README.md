# myvo-coin

My very own blockchain coin.

A simple student project to make sense of the blockchain with its encryption/decryption process and ownership proof.

(This is my first rust project so it is highly likely that there are multiple inefficient copies or strange loops/assignments/struct implementations/etc.)

The summary:

- A simple blockchain where coins can be transferred.
- With each new block 1 coin is generated.
- The hash of a block needs to have 1 leading zero by default (which can be changed through a constant).
- Not implemented
  - A p2p protocol to broadcast transactions and send the blockchain structure around is missing so it can only be used locally.
  - A timestamp to adjust the hash complexity dynamically.

## How to run it

1. Create a folder called `keys`. This is where the private/public keypairs are stored for proofing ownership.
2. Run the program with the argument 1: `cargo run 1`. This will create three new keypairs and create a new blockchain with a first transaction.
3. Run the program with the argument 2: `cargo run 2`. An interface is shown to show the current balance, create new keypairs and create new transactions.

### Sample Transaction Creation

After runninig the program with `cargo run 1` there should be three keys in the keys directory.
Running `cargo run 2` and entering `1` should then show something like

```
[Wallet] Loaded "keys/ecf41b24ba4c6730ccf3c41c803228cd8b917f701a9fdf27ac4fc3ee66cac597.pk"
[Wallet] Loaded "keys/fbc479bf3ffe1316667b121523f1df58d381989830ec04370de7d61b3339b6db.pk"
[Wallet] Loaded "keys/be026853dc025f29a3e9626aa344953da4dee133564a7a64c2157b065671d63e.pk"

      --- [ myvo-coin ] ---
.....What do you want to do?.....

[1] Print Cash Book information
[2] Create a new keypair and print the key hash
[3] Create a new transaction
[4] Quit

1
>>> [CashBook] <<<

 --- [Wallet] ('keys') ---
ecf41b24ba4c6730ccf3c41c803228cd8b917f701a9fdf27ac4fc3ee66cac597.pk
fbc479bf3ffe1316667b121523f1df58d381989830ec04370de7d61b3339b6db.pk
be026853dc025f29a3e9626aa344953da4dee133564a7a64c2157b065671d63e.pk

[Balance]: 2
 -> [Receipt]: 1 ([Keypair] [Public Key] ecf41b24ba4c6730ccf3c41c803228cd8b917f701a9fdf27ac4fc3ee66cac597)
 -> [Receipt]: 1 ([Keypair] [Public Key] be026853dc025f29a3e9626aa344953da4dee133564a7a64c2157b065671d63e)

<<< [CashBook] >>>
```

The first three rows indicate which keyfiles were found by the wallet.

The cashbook is then concerned with holding the wallet and looking the keys up in the blockchain to figure out which keys are still valid (i.e., haven't been used for transactions already) and how many coins are connected to each keypair.
Here, the balance is 2 coins with the two keypairs shown contributing 1 coin each.
This also means that the wallet key starting with `fbc...` was used in the first transaction is now worthless.

A new transaction can be created by first creating two new keypairs by entering `2` twice (one for receiving the transaction amount and one for receiving the block creation (incentive) amount).

```
Created a new keypair with this public key:
2e1ffd99ea4c4aab7d7d360be713e9631bd9986be0f58982ce06bbbf22ac3270
...
Created a new keypair with this public key:
d01966e4c6252e8663b4cb4ad3f002c5b2458e1007f209d3f6c3165c739a5bf2
```

Then, a new transaction can be created by entering `3`.
To fulfil a transaction inputs have to be defined first, i.e., who owns the money now.
These inputs are one or more of the keypair files that still have value attached to them in the balance.
Also, the receiver has to be specified which will be the first newly created keypair and the amount that this receiver should get and which should match the summed amount of the inputs.
Finally, the receiver of the creation amount will be the second newly created keypair.

Note, that the creation receiver will be connected to the block while the transaction receiver will be connected to the transaction in the block.
A block could contain multiple transaction.

```
3
-> Do you want to add an input?
[1] Yes
[2] No
1
-> Please input the name of the input keypair file.
keys/ecf41b24ba4c6730ccf3c41c803228cd8b917f701a9fdf27ac4fc3ee66cac597.pk
-> Do you want to add an input?
[1] Yes
[2] No
2
-> Please input the public key as hex of the recipient.
2e1ffd99ea4c4aab7d7d360be713e9631bd9986be0f58982ce06bbbf22ac3270
-> Please input the amount to be sent.
1
-> Please input the public key as hex of the creation recipient.
d01966e4c6252e8663b4cb4ad3f002c5b2458e1007f209d3f6c3165c739a5bf2

-> Is this correct?
> Receiver public key as hex: 2e1ffd99ea4c4aab7d7d360be713e9631bd9986be0f58982ce06bbbf22ac3270
> Amount:                     1
> Creation public key as hex: d01966e4c6252e8663b4cb4ad3f002c5b2458e1007f209d3f6c3165c739a5bf2

[1] Correct
[2] Change

1
360 tries in 48ms
The block starts with [0, 94, 108, 156, ...
```

The last two rows indicate that it took 48ms to hash the block 360 times until a hash was found that starts with one leading zero (which is the default number of leading zeros to only proof that it works) and then shows the first four byte values of the hash.

The cashbook now contains two more keys where one of the previously valid keys is now worthless, because the amount of this keys output was transferred to the `2e1...` keypair.
As one coin was just transferred to the same wallet but a different keypair and one coin was newly created, the overall balance is now `3`.

```
>>> [CashBook] <<<

 --- [Wallet] ('keys') ---
ecf41b24ba4c6730ccf3c41c803228cd8b917f701a9fdf27ac4fc3ee66cac597.pk
fbc479bf3ffe1316667b121523f1df58d381989830ec04370de7d61b3339b6db.pk
be026853dc025f29a3e9626aa344953da4dee133564a7a64c2157b065671d63e.pk
2e1ffd99ea4c4aab7d7d360be713e9631bd9986be0f58982ce06bbbf22ac3270.pk
d01966e4c6252e8663b4cb4ad3f002c5b2458e1007f209d3f6c3165c739a5bf2.pk

[Balance]: 3
 -> [Receipt]: 1 ([Keypair] [Public Key] be026853dc025f29a3e9626aa344953da4dee133564a7a64c2157b065671d63e)
 -> [Receipt]: 1 ([Keypair] [Public Key] 2e1ffd99ea4c4aab7d7d360be713e9631bd9986be0f58982ce06bbbf22ac3270)
 -> [Receipt]: 1 ([Keypair] [Public Key] d01966e4c6252e8663b4cb4ad3f002c5b2458e1007f209d3f6c3165c739a5bf2)

<<< [CashBook] >>>
```

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

How the parts of a transactions are put together.

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

### Validating Input

As the sender of a transaction has to proof ownership over the used input, there has to exist some validation mechanism.

This mechanism is a Validator that contains a signature created with the private key that was used to generate the previous outputs Public Key Hash and the public key itself.

The signature is the result of signing most of the new transaction with the private key except the Validator itself or anything that contains the full public key.
Hence, the transactions id of the previous output, its output index, the corresponding Public Key Hash of that output and the Public Key Hash and coin amount of the current transaction will be signed.
This way no tampering can be done while sending the transaction over a network.

Using the public key of the Validator, one can validate that the previous Public Key Hash is correct and that the signature was created with the private key of the owner proofing ownership of the output.
