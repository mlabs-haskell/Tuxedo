# Tuxedo web service functionality

A REST API for communicating with Tuxedo node template.

## Overview

This is a service built on Axum to support the decentralized application (DApp) built on the Tuxedo Blockchain that allows users to create, trade, breed, and manage virtual cats known as "Kitties". This README provides an overview of the available operations and REST APIs for interacting with the Cryptokitties platform.

Like many UTXO wallets, this web service synchronizes a local-to-the-wallet database of UTXOs that exist on the current best chain.Let's call this as Indexer from now on.
The Indexer does not sync the entire blockchain state.
Rather, it syncs a subset of the state that it considers "relevant".
Currently, the Indexer syncs all relevant UTXOs i.e. Coins, KittyData, TradableKittyData, Timestamps. 
However, the Indexer is designed so that this notion of "relevance" is generalizable.
This design allows developers building chains with Tuxedo to extend the Indexer for their own needs.
However, because this is a rest API-based web service, it is likely to be used by DApps which will leverage the REST API to achieve results.

The overall idea behind the web service architecture: https://github.com/mlabs-haskell/TuxedoDapp/issues/35

Links :
**Sequence dig for API flow:** https://github.com/mlabs-haskell/TuxedoDapp/issues/35#issuecomment-2020211287

**Algorithm to create the redeemer:** https://github.com/mlabs-haskell/TuxedoDapp/issues/35#issuecomment-2015171702

**The overall procedure required from DApp**: https://github.com/mlabs-haskell/TuxedoDapp/issues/35#issuecomment-2011277263

**Difference between signed transaction and unsigned transaction example:** https://github.com/mlabs-haskell/TuxedoDapp/issues/35#issuecomment-2020399526

## REST Documentation

Webservice can be run by using 

```sh
$ cargo run
```

Webservice tests can be run using:

```sh
$ cargo test
```


When we trigger the below REST APIs, we will get the below error as there will be a mismatch between the current Genesis block and the previously stored Genesis block in the local sled db.
This usually happens when we restart the blockchain node which causes the mismatch between genesis and genesis in our local sled db.

```sh
Node's Genesis block::0xbd8b9a1d322444332727ca249a5bec1bffeb488fcd16eb5570e7af5bb76de2bf
thread 'tokio-runtime-worker' panicked at webservice-wallet/src/service_handlers/money_handler/money_servicehandler.rs:65:38:
Error: Node reports a different genesis block than wallet. Wallet: 0x6259f26d866c56e2034257aef53c96d28631c4f0be662863c506a54d0df9449f. Expected: 0xbd8b9a1d322444332727ca249a5bec1bffeb488fcd16eb5570e7af5bb76de2bf. Aborting all operations
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```
## Solution to above problem
Execute below command in the machine where web service is running 
```sh
$ cd /tmp/tuxedo-wallet/
$ rm -rf wallet_database

```


## Guided tour for REST APIS usage 

This guided tour shows REST apis usage and curl command used to hit the endpoints :

### Minting coins 

Rest apis for minting coins

**end point:** post-mint-coin

**Amount to mint:** 6000

**Public_key of owner:** d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"amount": 6000,"owner_public_key":"d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}' http://localhost:3000/post-mint-coin

```

### Get all coins 

Rest apis for getting all the coins stored in the web service. Basically web service stores all the coin UTXO which are synced from the genesis block to the current height.

**end point:** get-all-coins

```sh
$ curl -X GET -H "Content-Type: application/json" http://localhost:3000/get-all-coins

```

### Get all owned coins 

Rest API for getting all the coins owned by a particular user or public key  in the web service. Web service stores all the coin utxos which are synced from the genesis block to the current height. Webservice will filter the coin UTXO filtered by the supplied public jey.

**end point:** get-owned-coins

**Public_key of owner:** Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

```sh
$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-owned-coins

```

### Create kitty:

Rest API for creating the kitty 

**end point:** post-create-kitty

**Name of kitty to be created:** amit

**Public_key of owner of kitty:** Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**Returns:** Created kitty.

```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"name": "amit","owner_public_key":"d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}' http://localhost:3000/post-create-kitty
```

### Get all kitties/Tradable kitties:

Rest API forgetting all the kitties stored in the local db. It returns all the kitties irrespective of onwer.

**end point:** get-all-kitty-list

**end point:** get-all-tradable-kitty-list

**Returns:** All basic kitties/tradableKitties irrespective of owner.

```sh
$ curl -X GET -H "Content-Type: application/json"  http://localhost:3000/get-all-kitty-list

$ curl -X GET -H "Content-Type: application/json"  http://localhost:3000/get-all-tradable-kitty-list

```

### Get owned kitties/Tradable kitties:

Rest API forgetting all the owned kitties/tradable kitties  by any particular owner i.e. public key stored in the local db.

**end point for kitty :** get-owned-kitty-list

**end point for tradable kitty :** get-owned-tradable-kitty-list

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example : d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**Returns:** All the kitties/tradable-owned by the user i.e public key.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-owned-kitty-list

$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-owned-tradable-kitty-list

```
### Get kitty/Tradable kitty details by DNA :

Rest API for getting all the details of the kitty by DNA.

**end point for basic kitty :** get-kitty-by-dna

**end point for tradable kitty :** get-tradable-kitty-by-dna

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Returns:** The kitty whose DNA matches, else None.


```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de" http://localhost:3000/get-kitty-by-dna

$ curl -X GET -H "Content-Type: application/json" -H "td-kitty-dna: 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de" http://localhost:3000/get-tradable-kitty-by-dna

```
## From now on all the below APIS will have two API Calls in Sequence for one operation: 

**1. Get Transaction and Input UTXO List:**
 Retrieves the transaction and input list required for generating the Redeemer by the web DApp. This call is not routed to the blockchain but is handled entirely by the web service.

 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**
 Sends the signed transaction to the blockchain via web service for verification and validation using the verifier and constraint checker, respectively.


### List kitty for sale :
Rest API is used for listing a Kitty for sale, converting it into a TradableKitty with an associated price.

**1. Get Transaction and Input UTXO List for list kitty for sale:**

**end point:** get-txn-and-inpututxolist-for-listkitty-forsale

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example : d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-price:**  Price of the kitty

**Returns:** Transaction for listing a kitty for sale without redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 394bd079207af3e0b1a9b1eb1dc40d5d5694bd1fd904d56b96d6fad0039b1f7c" -H "kitty-price: 100" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-listkitty-forsale

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:** put-listkitty-for-sale

**signed_transaction:**: Send the signed transaction. i.e all inputs should have redeemer to prove the ownership of spending or usage.

**Returns:** Tradable kitty .

 ```sh
$ curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x0652486c2774dfedfbf60bb7b07c3d44382d816ee40e866deedab59c18e95bf9","index":0},"redeemer":[220, 1, 218, 165, 148, 2, 133, 212, 30, 175, 14, 76, 51, 187, 44, 197, 113, 58, 200, 237, 68, 119, 107, 136, 229, 141, 249, 120, 213, 127, 55, 96, 2, 94, 7, 28, 157, 133, 123, 181, 178, 120, 16, 103, 95, 48, 29, 9, 121, 198, 237, 60, 1, 64, 184, 179, 241, 21, 207, 96, 18, 17, 176, 132]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"ListKittiesForSale"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/put-listkitty-for-sale

```


### Tradable kitty name update :
Rest API is used for updating the name of tradable kitty.

**1. Get Transaction and Input UTXO List for name update of tradable kitty:**

**end point:**:get-txn-and-inpututxolist-for-td-kitty-name-update

**DNA of tradable kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of tradable kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-new-name:**  New name of the kitty

**Returns:** Transaction with tradable kitty name update without redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90" -H "kitty-new-name: jbbl" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-td-kitty-name-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:patch-update-td-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 **Returns:** Tradable kitty with updated name.

 ```sh
$ curl -X PATCH \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0xd876e07116639ccc2e1aff0e99de2843f28b682d8ed2f72748060cff673dfa4e","index":0},"redeemer":[76, 152, 233, 6, 117, 88, 222, 255, 204, 185, 124, 203, 32, 71, 28, 54, 188, 201, 83, 161, 48, 127, 47, 122, 175, 140, 180, 245, 241, 5, 118, 53, 125, 174, 99, 78, 231, 115, 73, 243, 48, 94, 62, 50, 16, 20, 19, 121, 178, 190, 248, 53, 59, 125, 11, 82, 145, 96, 122, 90, 36, 232, 176, 133]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"UpdateKittiesName"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/patch-update-td-kitty-name

```

### Tradable kitty price update :
Rest API is used for updating the price of tradable kitty.

**1. Get Transaction and Input UTXO List for price update of tradable kitty:**

**end point:**:get-txn-and-inpututxolist-for-td-kitty-price-update

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-price:**  New price of kitty

**Returns:** Transaction with tradable kitty price update without redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90" -H "kitty-price: 101" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-td-kitty-price-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:patch-update-td-kitty-price

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 **Returns:** Tradable kitty with updated price.

 ```sh
$ curl -X PATCH \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x0c107ce6272fb5bd2da225dbbc8c60f0a94eb49c674f7808cc32721da7b8f751","index":0},"redeemer":[156, 45, 121, 232, 235, 7, 200, 99, 35, 110, 163, 45, 124, 107, 60, 60, 98, 164, 31, 54, 132, 137, 79, 129, 219, 57, 59, 58, 144, 44, 16, 97, 162, 76, 25, 115, 178, 188, 35, 161, 149, 71, 51, 109, 192, 187, 145, 144, 52, 49, 196, 123, 138, 115, 164, 93, 189, 78, 250, 1, 242, 127, 1, 129]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,101,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"UpdateKittiesPrice"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/patch-update-td-kitty-price
```


### De-List kitty from sale :
Rest API is used for removing a tradable Kitty from the sale, converting it back to a Basic Kitty without an associated price.

**1. Get Transaction and Input UTXO List for delist-kitty-from-sale:**

**end point:**:get-txn-and-inpututxolist-for-delist-kitty-from-sale

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

 **Returns:** Transaction with a delisted  kitty without redeemer..

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna:e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-delist-kitty-from-sale

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:put-delist-kitty-from-sale

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 **Returns:** Basic kitty.

 ```sh
$ curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x102e8575f9fc33061fbfa5a3787f264b934d2f05ab7714324a27d73b3c9aeb13","index":0},"redeemer":[240, 72, 218, 57, 203, 151, 58, 22, 2, 159, 101, 110, 99, 3, 80, 129, 188, 215, 41, 165, 48, 204, 137, 39, 107, 119, 121, 206, 47, 193, 165, 77, 134, 8, 19, 205, 32, 43, 52, 165, 120, 119, 235, 13, 48, 213, 122, 13, 62, 36, 113, 72, 12, 243, 53, 236, 166, 165, 209, 2, 199, 102, 101, 142]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"DelistKittiesFromSale"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,101,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/put-delist-kitty-from-sale

```

### kitty name update :
Rest API is used for updating the name of basic kitty.

**1. Get Transaction and Input UTXO List for name update of kitty:**

**end point:**:get-txn-and-inpututxolist-for-kitty-name-update

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-new-name:**  New name of the kitty

**Returns:** Transaction with kitty name update without redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90" -H "kitty-new-name: jram" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-kitty-name-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:patch-update-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

**Returns:** Kitty with an updated name.

 ```sh
$ curl -X PATCH \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x49ae6667d5f89d2a20da1704e751e206fb93ea109585c30da307f2832f96d808","index":0},"redeemer":[62, 143, 195, 20, 176, 220, 208, 33, 255, 203, 197, 239, 59, 88, 129, 185, 18, 132, 21, 4, 138, 49, 85, 117, 192, 68, 136, 216, 16, 215, 220, 66, 221, 217, 182, 236, 154, 160, 75, 212, 8, 120, 234, 43, 222, 58, 13, 50, 58, 175, 129, 241, 141, 234, 212, 222, 57, 203, 165, 198, 183, 220, 74, 137]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,114,97,109],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"FreeKitty":"UpdateKittiesName"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/patch-update-kitty-name
```

### Breed kitty :
Rest API is used for breeding a new Kitty from two parent Kitties, creating a child DNA based on both 

**1. Get Transaction and Input UTXO List for breed kitty:**

**end point:**:get-txn-and-inpututxolist-for-breed-kitty

**DNA of mom kitty:**  Input the DNA of kitty. Note it should start without 0X. Example  e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90

**DNA of dad kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 1c0d2c1f1b5055581414d781c162962ff37d7abc2e3c4580cd3321723eba2415

**Public_key of the owner of kitties:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**"child-kitty-name**  Name of child kitty

**Returns:** Transaction with breeding info such as mom, dad, child i.e. new family without a redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "mom-dna: e3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90" -H "dad-dna: 1c0d2c1f1b5055581414d781c162962ff37d7abc2e3c4580cd3321723eba2415" -H "child-kitty-name: jram" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-breed-kitty

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:post-breed-kitty

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

**Returns:** New family. I.e Mom kitty, Dad kitty and Child kitty. The mom and dad will have breeding status updated EX: From raringToGo to Tired or hadRecentBirth.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0xd3d22fba6dc77681382597dd1cf211a4224f3f9b61d243211ef3e568a53a8b4d","index":0},"redeemer":[130, 105, 164, 215, 55, 250, 158, 123, 55, 224, 1, 254, 2, 202, 3, 8, 130, 150, 121, 22, 173, 84, 207, 98, 207, 193, 175, 158, 127, 203, 173, 115, 68, 251, 206, 161, 116, 204, 125, 82, 211, 196, 175, 251, 3, 252, 157, 179, 97, 20, 97, 202, 231, 98, 29, 42, 213, 30, 243, 100, 214, 250, 105, 136]},{"output_ref":{"tx_hash":"0x2fff23770c669bb7d8f3be701c9c13f2a537e35a306df7d413376713b845c91e","index":0},"redeemer":[240, 39, 153, 183, 73, 88, 183, 29, 85, 199, 155, 119, 221, 131, 86, 240, 114, 216, 11, 145, 73, 76, 141, 93, 106, 218, 51, 73, 52, 183, 2, 32, 232, 176, 205, 136, 168, 165, 235, 37, 186, 218, 75, 110, 37, 223, 188, 72, 131, 187, 100, 50, 28, 211, 37, 78, 92, 51, 43, 40, 19, 114, 209, 137]}],"peeks":[],"outputs":[{"payload":{"data":[0,1,1,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,114,97,109],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}},{"payload":{"data":[1,1,1,0,0,0,0,0,0,0,28,13,44,31,27,80,85,88,20,20,215,129,193,98,150,47,243,125,122,188,46,60,69,128,205,51,33,114,62,186,36,21,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}},{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,198,109,250,226,133,108,197,53,184,120,210,139,76,35,109,156,100,214,242,113,227,227,174,38,22,37,65,198,241,250,122,72,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,114,97,109],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"FreeKitty":"Breed"}},"input_utxo_list":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,227,167,195,47,34,151,3,45,4,175,132,148,100,203,249,211,0,227,38,96,123,158,213,70,253,157,221,43,111,128,60,144,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,114,97,109],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}},{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,28,13,44,31,27,80,85,88,20,20,215,129,193,98,150,47,243,125,122,188,46,60,69,128,205,51,33,114,62,186,36,21,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/post-breed-kitty

```
**The output message of breed looks like the below :**
 ```sh
$O/P message :
{"message":"Kitty breeding done successfully","mom_kitty":{"parent":{"Mom":"HadBirthRecently"},"free_breedings":1,"dna":"0xe3a7c32f2297032d04af849464cbf9d300e326607b9ed546fd9ddd2b6f803c90","num_breedings":1,"name":[106,114,97,109]},"dad_kitty":{"parent":{"Dad":"Tired"},"free_breedings":1,"dna":"0x1c0d2c1f1b5055581414d781c162962ff37d7abc2e3c4580cd3321723eba2415","num_breedings":1,"name":[97,109,105,116]},"child_kitty":{"parent":{"Mom":"RearinToGo"},"free_breedings":2,"dna":"0xc66dfae2856cc535b878d28b4c236d9c64d6f271e3e3ae26162541c6f1fa7a48","num_breedings":0,"name":[106,114,97,109]}}

```

### Buy tradable kitty :
Rest API that allows buying a Tradable Kitty from a seller using cryptocurrency i.e money/coin

**1. Get Transaction and Input UTXO List for buying tradable kitty:**

**end point:** get-txn-and-inpututxolist-for-buy-kitty

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**input-coins:**  Reference of input coins owned by the buyer to be used for buying. We can input multiple input coins. EX: 4d732d8b0d0995151617c5c3beb600dc07a9e1be9fc8e95d9c792be42d65911000000000

**output_amount:** The amount to be paid for transaction which should be >= price of kitty.

**buyer_public_key:**  Public key of buyer i.e owner of coins used for buying: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**seller_public_key:**  Public key of seller i.e owner of kitty to be sold: Note it should start without 0X. Example: fab33c8c12f8df78fa515faa2fcc4bbf7829804a4d187984e13253660a9c1223

**Returns:** Transaction containing coins and kitty used in trading along with public keys of owner without redeemer.

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: bc147303f7d0a361ac22a50bf2ca2ec513d926a327ed678827c90d6512feadd6" -H "input-coins: 4d732d8b0d0995151617c5c3beb600dc07a9e1be9fc8e95d9c792be42d65911000000000" -H "output_amount: 200" -H "buyer_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" -H "seller_public_key: fab33c8c12f8df78fa515faa2fcc4bbf7829804a4d187984e13253660a9c1223"http://localhost:3000/get-txn-and-inpututxolist-for-buy-kitty

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:/patch-buy-kitty

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 **Returns:** Traded kitty with success or fail message.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x9bffe2abf274e0008f3f34af60cd083e909f884f2064e10f25ca46166306ae81","index":0},"redeemer":[134, 152, 55, 235, 162, 163, 255, 144, 247, 94, 237, 234, 127, 220, 149, 66, 226, 223, 43, 116, 16, 156, 165, 251, 221, 234, 13, 136, 132, 189, 187, 27, 206, 197, 48, 23, 188, 43, 41, 94, 103, 242, 174, 100, 249, 158, 206, 55, 88, 199, 103, 246, 227, 126, 138, 252, 205, 7, 132, 3, 112, 239, 52, 129]},{"output_ref":{"tx_hash":"0x4d732d8b0d0995151617c5c3beb600dc07a9e1be9fc8e95d9c792be42d659110","index":0},"redeemer":[166, 2, 32, 88, 200, 30, 54, 252, 155, 169, 122, 237, 29, 44, 33, 22, 102, 77, 71, 128, 35, 214, 84, 147, 193, 59, 45, 110, 69, 52, 25, 75, 5, 248, 227, 232, 110, 165, 177, 178, 218, 240, 235, 61, 25, 248, 242, 132, 106, 115, 62, 88, 57, 238, 39, 150, 202, 64, 237, 111, 147, 210, 215, 131]}],"peeks":[],"outputs":[{"payload":{"data":[0,0,2,0,0,0,0,0,0,0,188,20,115,3,247,208,163,97,172,34,165,11,242,202,46,197,19,217,38,163,39,237,103,136,39,201,13,101,18,254,173,214,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,200,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}},{"payload":{"data":[200,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[99,111,105,0]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xfab33c8c12f8df78fa515faa2fcc4bbf7829804a4d187984e13253660a9c1223"}}}],"checker":{"TradableKitty":"Buy"}}}' \
  http://localhost:3000/patch-buy-kitty
```

After the transaction is success.We can verify the kitty is transferred to buyer and coins are transferred to the seller using the below rest APIS :

 ```sh
$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-owned-kitty-list


curl -X GET -H "Content-Type: application/json" -H "owner_public_key: fab33c8c12f8df78fa515faa2fcc4bbf7829804a4d187984e13253660a9c1223" http://localhost:3000/get-owned-kitty-list

```

My test results of buy kitty: https://github.com/mlabs-haskell/TuxedoDapp/issues/27#issuecomment-2029302071

Please also see the below link for how to achieve the buy transaction which involves of signing from both buyer and seller in the same transaction :  https://github.com/Off-Narrative-Labs/Tuxedo/issues/169
