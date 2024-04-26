# Tuxedo Template Wallet

A cli wallet for the Tuxedo Node Template

## Overview

This wallet works with the Tuxedo Node Template and Tuxedo Template Runtime which is also contained in this repository.

Like many UTXO wallets, this one synchronizes a local-to-the-wallet database of UTXOs that exist on the current best chain.
The wallet does not sync the entire blockchain state.
Rather, it syncs a subset of the state that it considers "relevant".
Currently, the wallet syncs any UTXOs that contain tokens owned by a key in the wallet's keystore.
However, the wallet is designed so that this notion of "relevance" is generalizeable.
This design allows developers building chains with Tuxedo to extend the wallet for their own needs.
However, because this is a text- based wallet, it is likely not well-suited for end users of popular dapps.

## CLI Documentation

The node application has a thorough help page that you can access on the CLI. It also has help pages for all subcommands. Please explore and read these docs thoroughly.

```sh
# Show the wallet's main help page
$ tuxedo-template-wallet --help

A simple example / template wallet built for the tuxedo template runtime

Usage: tuxedo-template-wallet [OPTIONS] <COMMAND>

Commands:

...

# Show the help for a subcommand
$ tuxedo-template-wallet verify-coin --help
Verify that a particular coin exists.

Show its value and owner from both chain storage and the local database.

Usage: tuxedo-template-wallet verify-coin <OUTPUT_REF>

Arguments:
  <OUTPUT_REF>
          A hex-encoded output reference

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## Guided Tour

This guided tour shows off some of the most common and important wallet features. It can serve as a quickstart, but is not a substitute for reading the help pages mentioned above. (Seriously, please rtfm).

To follow this walkthrough, you should already have a fresh tuxedo template dev node running as described in the [main readme](../README.md). For example, `node-template --dev`.

### Syncing up an Initial Wallet

The wallet is not a long-running process.
The wallet starts up, syncs with the latest chain state, performs the action invoked, and exits.

Let's begin by just starting a new wallet and letting it sync.

```sh
$ tuxedo-template-wallet

[2023-04-11T17:44:40Z INFO  tuxedo_template_wallet::sync] Initializing fresh sync from genesis 0x12aba3510dc0918aec178a32927f145d22d62afe63392713cb65b85570206327
[2023-04-11T17:44:40Z INFO  tuxedo_template_wallet] Number of blocks in the db: 0
[2023-04-11T17:44:40Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 20
[2023-04-11T17:44:40Z INFO  tuxedo_template_wallet] No Wallet Command invoked. Exiting.
```

The logs indicate that a fresh database was created and had no blocks in it. Then, by communicating with the node, the wallet was able to sync 20 blocks. Finally it tells us that we didn't ask the wallet to tell us any specific information or send any transactions, so it just exits.

Let's run the same command again and see that the wallet persists state.

```sh
$ tuxedo-template-wallet

[2023-04-11T17:46:17Z INFO  tuxedo_template_wallet] Number of blocks in the db: 20
[2023-04-11T17:46:17Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 52
[2023-04-11T17:46:17Z INFO  tuxedo_template_wallet] No Wallet Command invoked. Exiting.
```

This time, it is not a fresh database. In fact it starts from block 20, where it left off previously, and syncs up to block 52. Again, we didn't tell the wallet any specific action to take, so it just exits.

We can also tell the wallet to skip the initial sync if we want to for any reason.
```sh
$ tuxedo-template-wallet --no-sync

[2023-04-11T17:47:48Z INFO  tuxedo_template_wallet] Number of blocks in the db: 52
[2023-04-11T17:47:48Z WARN  tuxedo_template_wallet] Skipping sync with node. Using previously synced information.
[2023-04-11T17:47:48Z INFO  tuxedo_template_wallet] No Wallet Command invoked. Exiting.
```

Now that we understand that the wallet syncs up with the node each time it starts, let's explore our first wallet command. Like most wallets, it will tell you how many tokens you own.

```sh
$ tuxedo-template-wallet show-balance

[2023-04-11T18:07:52Z INFO  tuxedo_template_wallet] Number of blocks in the db: 52
[2023-04-11T18:07:52Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 55
Balance Summary
0xd2bf…df67: 100
--------------------
total      : 100
```
The wallet begins by syncing the blockchain state, as usual.
Then it shows us that it knows about this `0xd2bf...` account.
This is the test account, or the "SHAWN" account.
The wallet already contains these keys so you can start learning quickly.
And it seems this account has some money.
Let's look further.

### Exploring the Genesis Coin

The chain begins with a single coin in storage.
We can confirm that the node and the wallet are familiar with the genesis coin using the `verify-coin` subcommand.

```sh
$ tuxedo-template-wallet verify-coin 000000000000000000000000000000000000000000000000000000000000000000000000

[2023-04-11T17:50:04Z INFO  tuxedo_template_wallet] Number of blocks in the db: 55
[2023-04-11T17:50:04Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 80
Details of coin 000000000000000000000000000000000000000000000000000000000000000000000000:
Found in storage.  Value: 100, owned by 0xd2bf…df67
Found in local db. Value: 100, owned by 0xd2bf…df67
```

After syncing, it tells us the status of the coin that we are asking about.
That number with all the `0`s is called an `OutputRef` and it is a unique way to refer to a utxo.
The wallet tells us that the coin is found in the chain's storage and in the wallet's own local db.
Both sources agree that the coin exists, is worth 100, and is owned by Shawn.

Let's "split" this coin by creating a transaction that spends it and creates two new coins worth 40 and 50, burning the remaining 10.

```sh
$ ./target/release/tuxedo-template-wallet spend-coins -r "40 50"

The args are:: SpendArgs { input: [], recipients: [(0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, [40, 50])] }
in Sync::get_arbitrary_unspent_set output_ref = OutputRef { tx_hash: 0xa95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf71, index: 0 }
[2024-04-24T14:00:28Z INFO  tuxedo_template_wallet::money] Node's response to spend transaction: Ok("0x7e0f3ad4103e6daaa12faa6b7ad76b70a1f520f48dc1ee4ed77404996cf8360c")
Created "792a03115790d50501cc0fa504d86c040c96a754dc633203f99505f74f61ae7000000000" worth 40. owned by 0xd2bf…df67
Created "792a03115790d50501cc0fa504d86c040c96a754dc633203f99505f74f61ae7001000000" worth 50. owned by 0xd2bf…df67

```

Our command told the wallet to create a transaction that spends some coins (in this case the genesis coin) and creates two new coins with the given amounts, burning the remaining 10.
It also tells us the `OutputRef`s of the new coins created.

A balance check reveals that our balance has decreased by the 10 burnt tokes as expected.

```sh
$ tuxedo-template-wallet show-balance

[2023-04-11T18:52:26Z INFO  tuxedo_template_wallet] Number of blocks in the db: 87
[2023-04-11T18:52:26Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 95

Balance Summary
0xd2bf…df67: 90
--------------------
total      : 90

```

In this case we didn't specify a recipient of the new outputs, so the same default address was used. Next let's explore using some other keys.

### Using Your Own Keys

Of course we can use other keys than the example Shawn key.
The wallet supports generating our own keys, or inserting pre-existing keys.
To follow this guide as closely as possible, you should insert the same key we generated.

```sh
# Generate a new key
$ tuxedo-template-wallet generate-key

  Generated public key is 0x1c0cf1c1cc741596cc115c4f2eabe1377fcd5d23774516c5dd521525516 
  (5HamRMAa...)
 Generated Phrase is path second heart chapter pilot artwork reward other surge energy deer tackle

# Or, to continue on with demo, insert the same generated key
$ tuxedo-template-wallet insert-key "decide city tattoo arrest jeans split main sad slam blame crack farm"

  The generated public key is f41a866782d45a4d2d8a623a097c62aee6955a9e580985e3910ba49eded9e06b (5HamRMAa...)
```

With our new keys in the keystore, let's send some coins from Shawn to our own key.

```sh
$ tuxedo-template-wallet spend-coins \
 -r "0x1c0cf1c1cc741596cc115c4f2eabe1377fcd5d23774516c5dd521525516 \
 20 \
 10"
 
[2024-04-24T14:06:42Z INFO  tuxedo_template_wallet::money] In the spend_coins_to_multiple_recipient The args are:: SpendArgs { input: [], recipients: [(0x1c0cf1c1cc741596cc115c4f2eabe1377fcd5d23774516c5dd521525516e2d08, [20, 10])] }
in Sync::get_arbitrary_unspent_set output_ref = OutputRef { tx_hash: 0x792a03115790d50501cc0fa504d86c040c96a754dc633203f99505f74f61ae70, index: 0 }
[2024-04-24T14:06:42Z INFO  tuxedo_template_wallet::money] Node's response to spend transaction: Ok("0x5f3e867662114157e7cf8333180f90e42a9f309320b64e0f8efa0328a507360c")
Created "5a13b5e7a8b0bd07ac76e5ef58351d1e9417387b4c0ceb7d8982eb0ba65ebd2c00000000" worth 20. owned by 0x1c0c…2d08
Created "5a13b5e7a8b0bd07ac76e5ef58351d1e9417387b4c0ceb7d8982eb0ba65ebd2c01000000" worth 10. owned by 0x1c0c…2d08
```

This command will consume one of the existing coins, and create two new ones owned by our key.
Our new coins will be worth 20 and 10 tokens.
Let's check the balance summary to confirm.

```sh
$ tuxedo-template-wallet show-balance

[2023-04-11T18:54:42Z INFO  tuxedo_template_wallet] Number of blocks in the db: 99
[2023-04-11T18:54:42Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 101

Balance Summary
0xd2bf…df67: 50
0xf41a…e06b: 30
--------------------
total      : 80
```
It is possible to create new coins using the wallet. Let's explore how to do it.

### Minting coins

We can optionally pass the amount and public key of the owner as arguments to mint_coins.
If optional arguments are not passed below are the default values:
Amount is `100` and Public key of owner is Shawn key.

```sh
$ tuxedo-template-wallet mint-coins \
 --owner 0xdeba7f5d5088cda3e32ccaf479056dd934d87fa8129987ca6db57c122bd73341 \
 --amount 200 \

[2024-01-18T14:22:19Z INFO  tuxedo_template_wallet] Number of blocks in the db: 6
[2024-01-18T14:22:19Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 14
[2024-01-18T14:22:19Z INFO  tuxedo_template_wallet::money] Node's response to mint-coin transaction: Ok("0xaff830b7755fee67c288afe18dfa6eabffe06286005b0fd6cb8e57b246c08df6")
Created "f76373909591d85f796c36ed4b265e46efabdf5b5c493b94246d590823cc42a500000000" worth 200. owned by 0xdeba…3341
```
It is possible to verify a newly minted coin exists in both chain storage and the local database using verify-coin command.

### Manually Selecting Inputs

So far, we have let the wallet select which inputs to spend on our behalf.
This is typically fine, but some users like to select specific inputs for their transactions.
The wallet supports this.
But before we can spend specific inputs, let's learn how to print the complete list of unspent outputs.

```sh
$ tuxedo-template-wallet show-all-outputs

[2023-04-11T18:55:23Z INFO  tuxedo_template_wallet] Number of blocks in the db: 101
[2023-04-11T18:55:23Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 104

###### Unspent outputs ###########
90695702dabcca93d2c5f84a45b07bf59626ddb49a9b5255e202777127a3323d00000000: owner 0xf41a866782d45a4d2d8a623a097c62aee6955a9e580985e3910ba49eded9e06b, amount 20
90695702dabcca93d2c5f84a45b07bf59626ddb49a9b5255e202777127a3323d01000000: owner 0xf41a866782d45a4d2d8a623a097c62aee6955a9e580985e3910ba49eded9e06b, amount 10
9b3b0d17ad5f7784e840c40089d4d0aa0de990c5c620d49a0729c3a45afa35bf01000000: owner 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, amount 50
```

Now that we know precisely which outputs exist in our chain, we can choose to spend a specific one.
Let's consume our 20 token input and send 15 of its coins to Shawn, burning the remaining 5.
Because we are sending to Shawn, and Shawn is the default recipient, we could leave off the `--recipient` flag, but I'll choose to include it anyway.

```sh
# The input value has to be copied from your own `show-all-outputs` results
$ tuxedo-template-wallet spend-coins \
  --input 90695702dabcca93d2c5f84a45b07bf59626ddb49a9b5255e202777127a3323d00000000 \
  -r "0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 15"

[2023-04-11T18:57:20Z INFO  tuxedo_template_wallet] Number of blocks in the db: 94
[2023-04-11T18:57:20Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 133
[2023-04-11T18:57:20Z INFO  tuxedo_template_wallet::money] Node's response to spend transaction: Ok("0x80018b868d1e29be5cb758e15618091da8185cd7256ae3338df4605732fcfe9f")

Created "4788fd9d517af94c2cfac80cb23fa6a63c41784b6fab01efd5d33b907af2550500000000" worth 15. owned by 0xd2bf…df67
```

You should confirm for yourself that both the balance summary and the complete list of UTXOs look as you expect.

### Multi recipients 
 we will demonstrate is its ability to construct transactions where we can send the monmey to multiple recipients.

```sh
# The input value has to be copied from your own `show-all-outputs` results
$ tuxedo-template-wallet spend-coins \
-i a95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf7100000000 \
-r "0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 30 40 10" \
-r "0xe0c99ddba50d55a82d1313eaf1897eefe6e599e5dcf0e9e14f56f5a736b6f933 5 3 12"

[2024-04-24T13:44:18Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(SpendCoins(SpendArgs { input: [OutputRef { tx_hash: 0xa95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf71, index: 0 }], recipients: [(0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, [30, 40, 10]), (0xe0c99ddba50d55a82d1313eaf1897eefe6e599e5dcf0e9e14f56f5a736b6f933, [5, 3, 12])] })) }
h256_from_string called
[2024-04-24T13:44:18Z INFO  tuxedo_template_wallet] Number of blocks in the db: 5

h256_from_string called
[2024-04-24T13:44:18Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 17
[2024-04-24T13:44:18Z INFO  tuxedo_template_wallet::money] In the spend_coins_to_multiple_recipient The args are:: SpendArgs { input: [OutputRef { tx_hash: 0xa95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf71, index: 0 }], recipients: [(0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, [30, 40, 10]), (0xe0c99ddba50d55a82d1313eaf1897eefe6e599e5dcf0e9e14f56f5a736b6f933, [5, 3, 12])] }
[2024-04-24T13:44:18Z INFO  tuxedo_template_wallet::money] Node's response to spend transaction: Ok("0xbcd1620519e11e25fc8e76fcf9747fae2be4cb26b66c51a9052031e8be05461e")
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a800000000" worth 30. owned by 0xd2bf…df67
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a801000000" worth 40. owned by 0xd2bf…df67
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a802000000" worth 10. owned by 0xd2bf…df67
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a803000000" worth 5. owned by 0xe0c9…f933
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a804000000" worth 3. owned by 0xe0c9…f933
Created "6db953f4c36a1218e0dcb92a30ac43e753310688a82ebf0e743d46c656fb98a805000000" worth 12. owned by 0xe0c9…f933

```
You should confirm for yourself that both the show all outputs and the complete list of UTXOs look as you expect.


### Multi Owner

The final wallet feature that we will demonstrate is its ability to construct transactions with inputs coming from multiple different owners.

Here we will create a transaction with a single output worth 70 units owned by some address that we'll call Jose, and we'll let the wallet select the inputs itself.
This will require inputs from both Shawn and us, and the wallet is able to handle this.

```sh
$ tuxedo-template-wallet spend-coins \
  -r "0x066ae8f6f5c3f04e7fc163555d6ef62f6f8878435a931ba7eaf02424a16afe62 70"

[2023-04-11T18:59:18Z INFO  tuxedo_template_wallet] Number of blocks in the db: 146
[2023-04-11T18:59:18Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 173
[2023-04-11T18:59:19Z INFO  tuxedo_template_wallet::money] Node's response to spend transaction: Ok("0x04efb1c55f4efacbe41d00d3c5fe554470328a37150df6053bd48088e73a023c")

Created "d0f722019e05863769e64ac6d33ad3ebeb359ce0469e93a9856bfcc236c4bad700000000" worth 70. owned by 0x066a…fe62
```

Now we check the balance summary and find it is empty.
That is because Jose's keys are not in the keystore, so the wallet does not track his tokens.

Now let's begin our journey for kitty management using this wallet 

### kitty creation
A new kitty can be created using below command 
```sh
$ ./target/release/tuxedo-template-wallet create-kitty --kitty-name amit
[2024-04-26T06:27:29Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(CreateKitty(CreateKittyArgs { kitty_name: "amit", owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 })) }
[2024-04-26T06:27:29Z INFO  tuxedo_template_wallet] Number of blocks in the db: 796
[2024-04-26T06:27:29Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1019
Node's response to spawn transaction: Ok("0xb3c848749029bce7fae68d81cba91dd60128a4cec7d038d97aafc16df3b6ea19")
Created "20fbaf62b7b9b95417744f6f6e3d6a275c74036e3e1520036c77539ebf965f4000000000" basic Kitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67

```

### Show all kitties 
We Can see all the kitties. It will show all the kitties owned by all the users :

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:28:02Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:28:02Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1019
in kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x20fbaf62b7b9b95417744f6f6e3d6a275c74036e3e1520036c77539ebf965f40, index: 0 }
[2024-04-26T06:28:02Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1030
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("amit") => KittyData { parent: Mom(RearinToGo), free_breedings: 2, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 3, name: [97, 109, 105, 116] } ->
--------------------------------------------------
```

### Update the name of the kitty 
We Can update the name of the kitty. We can update the name of the tradable kitty also.

```sh
$ ./target/release/tuxedo-template-wallet update-kitty-name --dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --new-name limi
[2024-04-26T06:29:18Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(UpdateKittyName(UpdateKittyNameArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", new_name: "limi", owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 })) }
[2024-04-26T06:29:19Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1030
[2024-04-26T06:29:19Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1056
[2024-04-26T06:29:19Z INFO  tuxedo_template_wallet::kitty] The set_kitty_property are:: UpdateKittyNameArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", new_name: "limi", owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 }
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0x20fbaf62b7b9b95417744f6f6e3d6a275c74036e3e1520036c77539ebf965f40, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0x20fbaf62b7b9b95417744f6f6e3d6a275c74036e3e1520036c77539ebf965f40, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(KittyData { parent: Mom(RearinToGo), free_breedings: 2, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 3, name: [97, 109, 105, 116] })
Node's response to spawn transaction: Ok("0x106eb13bc3be6aeb656e23ce559d87b9907d0016ad963c0d979cfc3ddb04721f")
Created "4e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab2700000000" basic Kitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67****
```

To confirm the updated name please execute the below :

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:30:31Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:30:31Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1056
in kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x4e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab27, index: 0 }
[2024-04-26T06:30:31Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1080
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("limi") => KittyData { parent: Mom(RearinToGo), free_breedings: 2, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 3, name: [108, 105, 109, 105] } ->
--------------------------------------------------e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab2700000000" basic Kitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67****
```

### Breed kitty
We can breed the kitties using mom and Dad kitty.
For this, we created one more kitty which is dad kitty.

```sh
$ amit@DESKTOP-TF687VE:~/OmBlockchain/OmTest/Tuxedo$ ./target/release/tuxedo-template-wallet breed-kitty --mom-dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --dad-dna 6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15
[2024-04-26T06:35:59Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(BreedKitty(BreedKittyArgs { mom_dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", dad_dna: "6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15", owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 })) }
[2024-04-26T06:35:59Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1174
[2024-04-26T06:35:59Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1189
[2024-04-26T06:35:59Z INFO  tuxedo_template_wallet::kitty] The Breed kittyArgs are:: BreedKittyArgs { mom_dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", dad_dna: "6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15", owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 }
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0x4e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab27, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0x4e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab27, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(KittyData { parent: Mom(RearinToGo), free_breedings: 2, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 3, name: [108, 105, 109, 105] })
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  -> output_ref OutputRef { tx_hash: 0x4e2dfcacdcd877b2fbcb521bf8961ccb60a51b876412463febc8bb7ff1faab27, index: 0 }
 Name : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  NOTmatched
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  -> output_ref OutputRef { tx_hash: 0x7f406d8605899c6f347bd4b60253c61e15b4b6edae06240fe0cd97573038b2f8, index: 0 }
 Name : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  NOTmatched
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  -> output_ref OutputRef { tx_hash: 0xca4280eb22231abefc7136ac8e0fa7ec981f28c63712fc4022d29a0a1a889700, index: 0 }
 Name : KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)  matched
output_ref = OutputRef { tx_hash: 0xca4280eb22231abefc7136ac8e0fa7ec981f28c63712fc4022d29a0a1a889700, index: 0 }
kitty dna  KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15) found_status = Some(KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 3, name: [100, 105, 110, 97] })
New mom Dna = KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)
New Dad Dna = KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15)
Child Dna = KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492)
Node's response to spawn transaction: Ok("0x8c4c77922e6be143ab028c5cda702b928fdf4b16badc63f6936f8b6933a6332a")
Created "05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db888000000000" basic Kitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67
Created "05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db888001000000" basic Kitty 0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15. owned by 0xd2bf…df67
Created "05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db888002000000" basic Kitty 0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492. owned by 0xd2bf…df67

```

Let's check if we have 3 kitties. 1 mom kitty, 1 dada kitty, and a newly created child kitty 

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:36:17Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:36:17Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1189
in kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db8880, index: 0 }
in kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db8880, index: 1 }
in kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db8880, index: 2 }
[2024-04-26T06:36:17Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1195
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("limi") => KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("dina") => KittyData { parent: Dad(Tired), free_breedings: 1, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 4, name: [100, 105, 110, 97] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("tomy") => KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492), num_breedings: 0, name: [116, 111, 109, 121] } ->

--------------------------------------------------

```
### list kitty for sale 
We make kitty available for sale using the below command.
After this command is executed the basic kitty is converted to tradable kitty.

```sh
$ ./target/release/tuxedo-template-wallet list-kitty-for-sale --dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --price 150 --owner d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
[2024-04-26T06:40:43Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ListKittyForSale(ListKittyForSaleArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", price: 150, owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 })) }
[2024-04-26T06:40:43Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1195
[2024-04-26T06:40:43Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1284
[2024-04-26T06:40:43Z INFO  tuxedo_template_wallet::kitty] The list_kitty_for_sale args : ListKittyForSaleArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", price: 150, owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 }
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0x05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db8880, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0x05d3cf55bbe68399c51ddaa9f0576eb72220a31d6d525cd3ce6e2f6312db8880, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] })
Node's response to spawn transaction: Ok("0x991de30918c2eb65bee49372a6ecbdd6a180830f202944f17c79c85f9cbe2dc4")
Created "8e5c56c7b267b6727d508b0a6ae740cd3b849a4f513295c06a504a77d75facc200000000" TradableKitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67

```
Now let's see if the basic kitty is converted to tradable kitty or not.

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:41:50Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:41:50Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1284
in Tradable kitty:apply_transaction output_ref = OutputRef { tx_hash: 0x8e5c56c7b267b6727d508b0a6ae740cd3b849a4f513295c06a504a77d75facc2, index: 0 }
[2024-04-26T06:41:50Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1306
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("dina") => KittyData { parent: Dad(Tired), free_breedings: 1, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 4, name: [100, 105, 110, 97] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("tomy") => KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492), num_breedings: 0, name: [116, 111, 109, 121] } ->
=-===================================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("limi") => TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 150 } ->
--------------------------------------------------
=-===================================================

```

### kitty price update 
We can update the price of the tradable kitty.
You can see the price of "limi" is 150.
After execution of the below command, it will be updated to 200.

```sh
$ ./target/release/tuxedo-template-wallet update-kitty-price --dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --price 200
[2024-04-26T06:45:14Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(UpdateKittyPrice(UpdateKittyPriceArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", price: 200, owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 })) }
[2024-04-26T06:45:14Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1306
[2024-04-26T06:45:14Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1374
[2024-04-26T06:45:14Z INFO  tuxedo_template_wallet::kitty] The set_kitty_property are:: UpdateKittyPriceArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", price: 200, owner: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 }
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0x8e5c56c7b267b6727d508b0a6ae740cd3b849a4f513295c06a504a77d75facc2, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0x8e5c56c7b267b6727d508b0a6ae740cd3b849a4f513295c06a504a77d75facc2, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 150 })
Node's response to spawn transaction: Ok("0xb618175d4f6e164f74f8c0e7eed6a21347d19b45edf63a3dd43dc5d6f58c2b71")
Created "b474c81bcac92ccb2f19d0fef2ae0b06606b1251b5ec8e53a931101c1f438eee00000000" TradableKitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xd2bf…df67

```

Let's see again if the price of the kitty is updated or not.

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:46:11Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:46:11Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1374
in Tradable kitty:apply_transaction output_ref = OutputRef { tx_hash: 0xb474c81bcac92ccb2f19d0fef2ae0b06606b1251b5ec8e53a931101c1f438eee, index: 0 }
[2024-04-26T06:46:11Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1393
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("dina") => KittyData { parent: Dad(Tired), free_breedings: 1, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 4, name: [100, 105, 110, 97] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("tomy") => KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492), num_breedings: 0, name: [116, 111, 109, 121] } ->

=-===================================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("limi") => TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 200 } ->
--------------------------------------------------
=-===================================================

```

### buy kitty 
We can buy the kitty from one user to another user.
This involves 2 things as below :
1. Kitty must be transferred from seller to buyer
2. money i.e. coins should be transferred from buyer to seller .

we have 2 users as below :


```sh
$
seller : 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Buyes : 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600

```
The buyer has the money. i.e oxf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600 has a coin with id "0f338d55b02b10cc5b6804c5c649b46ba9e47e27102449d1af373036dd5b4f4500000000"

```sh
$ ./target/release/tuxedo-template-wallet show-all-outputs
[2024-04-26T06:50:30Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllOutputs) }
[2024-04-26T06:50:30Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1477
[2024-04-26T06:50:30Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1480
###### Unspent outputs ###########
0398586cdbd3428bbf85d4a6662ee49043e3715cc53ac5c7e80bd74eaf1f244701000000: owner 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, amount 200
0f338d55b02b10cc5b6804c5c649b46ba9e47e27102449d1af373036dd5b4f4500000000: owner 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600, amount 200
a95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf7100000000: owner 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, amount 100

```
The seller has the kitty i.e. tradable kitty "limit". This is already shown in an earlier command.

Now lets execute the buy kitty operation:

```sh
$ ./target/release/tuxedo-template-wallet buy-kitty --dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --input 0f338d55b02b10cc5b6804c5c649b46ba9e47e27102449d1af373036dd5b4f4500000000 --owner 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600 --seller 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 --output-amount 200
[2024-04-26T06:56:15Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(BuyKitty(BuyKittyArgs { input: [OutputRef { tx_hash: 0x0f338d55b02b10cc5b6804c5c649b46ba9e47e27102449d1af373036dd5b4f45, index: 0 }], seller: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, owner: 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600, dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", output_amount: [200] })) }
[2024-04-26T06:56:15Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1580
[2024-04-26T06:56:15Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1595
[2024-04-26T06:56:15Z INFO  tuxedo_template_wallet::kitty] The Buy kittyArgs are:: BuyKittyArgs { input: [OutputRef { tx_hash: 0x0f338d55b02b10cc5b6804c5c649b46ba9e47e27102449d1af373036dd5b4f45, index: 0 }], seller: 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, owner: 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600, dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", output_amount: [200] }
Owner  = 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0xb474c81bcac92ccb2f19d0fef2ae0b06606b1251b5ec8e53a931101c1f438eee, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0xb474c81bcac92ccb2f19d0fef2ae0b06606b1251b5ec8e53a931101c1f438eee, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 200 })
Node's response to spawn transaction: Ok("0x05c105ffbcb423d3e6309ed43c4c73fc3623ef923f45b05767d64c8b58923c81")
Created "c1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca63272700000000" TradableKitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xf47c…a600
Created "c1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca63272701000000" worth 200. owned by 0xd2bf…df67

```

Now let's check if the kitty is transferred from seller to buyer or not.
We can see kitty transferred from seller i.e 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67 to buyer i.e 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600.

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T06:57:28Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T06:57:28Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1595
in Tradable kitty:apply_transaction output_ref = OutputRef { tx_hash: 0xc1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca632727, index: 0 }
[2024-04-26T06:57:28Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1619
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("dina") => KittyData { parent: Dad(Tired), free_breedings: 1, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 4, name: [100, 105, 110, 97] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("tomy") => KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492), num_breedings: 0, name: [116, 111, 109, 121] } ->

Owner -> 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600
Some("limi") => TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 200 } ->
--------------------------------------------------

```
Now let's check if the coin is transferred from the buyer to the seller or not.

```sh
$ ./target/release/tuxedo-template-wallet show-all-outputs
[2024-04-26T06:59:35Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllOutputs) }
[2024-04-26T06:59:35Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1619
[2024-04-26T06:59:35Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1661
###### Unspent outputs ###########
a95362504966abd5ee55223d09e860bb1dd60eba1425b80fb05e1cc3ad66bf7100000000: owner 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, amount 100
c1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca63272701000000: owner 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67, amount 200

```

###  Delist kitty from sale
When owner of the kitty decides not sell the kitty, he can de-list the kitty from sale making it unavailable for sale.
With this operation, tradable kitty is converted to basic kitty as below:

```sh
$ ./target/release/tuxedo-template-wallet delist-kitty-from-sale  --dna d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815 --owner 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600
[2024-04-26T07:03:29Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(DelistKittyFromSale(DelistKittyFromSaleArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", owner: 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600 })) }
[2024-04-26T07:03:29Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1739
[2024-04-26T07:03:29Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1739
[2024-04-26T07:03:29Z INFO  tuxedo_template_wallet::kitty] The list_kitty_for_sale args : DelistKittyFromSaleArgs { dna: "d3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815", owner: 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600 }
Owner  = 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600 Dna : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  -> output_ref OutputRef { tx_hash: 0xc1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca632727, index: 0 }
 Name : KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815)  matched
output_ref = OutputRef { tx_hash: 0xc1c7e6b18a4da9035a4f488198afb54cc0849d165dcce20b684701f5ca632727, index: 0 }
kitty dna  KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815) found_status = Some(TradableKittyData { kitty_basic_data: KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] }, price: 200 })
Node's response to spawn transaction: Ok("0x4bf53d5c8781508550801a2faf18738322146c6d41946fa38f2d797329f022c6")
Created "a058d2d3ecfadf74c58cab37c18c5ff4f05c36e39e8999fd31b3726498e4ed1800000000" basic Kitty 0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815. owned by 0xf47c…a600

```

Let's cross whether the kitty "limi" is converted from a tradable to a basic kitty or not.

```sh
$ ./target/release/tuxedo-template-wallet show-all-kitties
[2024-04-26T07:03:39Z INFO  tuxedo_template_wallet] cli from cmd args = Cli { endpoint: "http://localhost:9944", path: None, no_sync: false, tmp: false, dev: false, command: Some(ShowAllKitties) }
[2024-04-26T07:03:39Z INFO  tuxedo_template_wallet] Number of blocks in the db: 1741
[2024-04-26T07:03:39Z INFO  tuxedo_template_wallet] Wallet database synchronized with node to height 1743
Show All Kitty Summary
==========================================
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("dina") => KittyData { parent: Dad(Tired), free_breedings: 1, dna: KittyDNA(0x6966efbf9f73cfc36724994dea537f9818701c0a22b939097e2cce75b24d8c15), num_breedings: 4, name: [100, 105, 110, 97] } ->
--------------------------------------------------
Owner -> 0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67
Some("tomy") => KittyData { parent: Dad(RearinToGo), free_breedings: 2, dna: KittyDNA(0x8a6d25fe23328d4c47a9ecd13b91b5aca2747708af65d27e2fbc83ae72415492), num_breedings: 0, name: [116, 111, 109, 121] } ->
--------------------------------------------------
Owner -> 0xf47c90b21c22b6b9312fc5cd65583b759f80288abe04b052bd49d2d727c5a600
Some("limi") => KittyData { parent: Mom(HadBirthRecently), free_breedings: 1, dna: KittyDNA(0xd3471ef11bb1ca2871708e8adb8d8b4e6786fb433fb2cf8d26077132922f1815), num_breedings: 4, name: [108, 105, 109, 105] } ->

```

