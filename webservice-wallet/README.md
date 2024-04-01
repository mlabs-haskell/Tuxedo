# Tuxedo webservice functionality

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

## REST Documentation

Webservice can be run by using 

```sh
$ cargo run
```

## Guided tour for REST APIS usage 

This guided tour shows orest apis usage and curl command used to hit the end points :

### Minting coins 

Rest apis for minting coins

**end point:**: mint-coins

**Amount to mint:** 6000

**Public_key of owner:** d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"amount": 6000,"owner_public_key":"d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}' http://localhost:3000/mint-coins

```

### Get all coins 

Rest apis for getting all the coins stored in the web service. Basically web service stores all the coin UTXO which are synced from the genesis block to the current height.

**end point:**: get-all-coins

```sh
$ curl -X GET -H "Content-Type: application/json" http://localhost:3000/get-all-coins

```

### Get all owned coins 

Rest API for getting all the coins owned by a particular user or public key  in the web service. Web service stores all the coin utxos which are synced from the genesis block to the current height. Webservice will filter the coin UTXO filtered by the supplied public jey.

**end point:**:get-owned-coins

**Public_key of owner:** Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

```sh
$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-owned-coins

```

### Create kitty:

Rest API for creating the kitty 

**end point:**:create-kitty

**Name of kitty to be created:**:amit

**Public_key of owner of kitty:** Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67


```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"name": "amit","owner_public_key":"d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}' http://localhost:3000/create-kitty

```

### Get all kitties:

Rest API forgetting all the kitties stored in the local db. It returns all the kitties irrespective of onwer.

**end point:**:get-all-kitty-list

```sh
$ curl -X GET -H "Content-Type: application/json"  http://localhost:3000/get-all-kitty-list

```

### Get owned kitties:

Rest API forgetting all the owned kitties by any particular owner i.e. public key stored in the local db.

**end point:**:get-owned-kitty-list

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example : d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67


```sh
$ curl -X GET -H "Content-Type: application/json" -H "owner_public_key: 563b6da067f38dc194cbe41ce0b840a985dcbef92b1e5b0a6e04f35544ddfd16" http://localhost:3000/get-owned-kitty-list

```
### Get kitty details by DNA :

Rest API for getting all the details of the kitty by DNA.

**end point:**:get-kitty-by-dna

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de" http://localhost:3000/get-kitty-by-dna

```
## From now on all the below APIS will have two API Calls in Sequence for one operation: 

**1. Get Transaction and Input UTXO List:**
 Retrieves the transaction and input list required for generating the Redeemer by the web DApp. This call is not routed to the blockchain but is handled entirely by the web service.

 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service :**
 Sends the signed transaction to the blockchain via web service for verification and validation using the verifier and constraint checker, respectively.


### List kitty for sale :
Rest API used for listing a Kitty for sale, converting it into a TradableKitty with an associated price.

**1. Get Transaction and Input UTXO List for list kitty for sale:**

**end point:**:get-txn-and-inpututxolist-for-listkitty-forsale

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example : d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-price:**  Price of the kitty

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 394bd079207af3e0b1a9b1eb1dc40d5d5694bd1fd904d56b96d6fad0039b1f7c" -H "kitty-price: 100" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-listkitty-forsale

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:listkitty-for-sale

**signed_transaction:**: Send the signed transaction. i.e all inputs should have redeemer to prove the ownership of spending or usage.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x0367d974927186bdeb3f1f1c111352711d9e1106a68bde6e4cfd0e64722e4f3a","index":0},"redeemer":[198, 69, 78, 148, 249, 1, 63, 2, 217, 105, 106, 87, 179, 252, 24, 66, 129, 190, 253, 17, 31, 87, 71, 231, 100, 31, 9, 81, 93, 141, 7, 81, 155, 0, 27, 38, 87, 16, 30, 55, 164, 220, 174, 37, 207, 163, 82, 216, 155, 195, 166, 253, 67, 95, 47, 240, 74, 20, 108, 160, 185, 71, 199, 129]}],"peeks":[],"outputs":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"ListKittiesForSale"}},"input_utxo_list":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/listkitty-for-sale

```

### Tradable kitty name update :
Rest API is used for updating the name of tradable kitty.

**1. Get Transaction and Input UTXO List for name update of tradable kitty:**

**end point:**:get-txn-and-inpututxolist-for-td-kitty-name-update

**DNA of tradable kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of tradable kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-new-name:**  New name of the kitty

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 394bd079207af3e0b1a9b1eb1dc40d5d5694bd1fd904d56b96d6fad0039b1f7c" -H "kitty-new-name: jbbl" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-td-kitty-name-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:update-td-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0xb696b071fdbdca1adcec9149d21a167a04d851693e97b70900ac7547e23c0d0e","index":0},"redeemer":[232, 135, 109, 225, 49, 100, 3, 154, 233, 14, 37, 46, 219, 87, 87, 126, 194, 46, 21, 194, 58, 138, 235, 176, 121, 59, 164, 20, 98, 31, 165, 109, 121, 81, 63, 97, 243, 214, 105, 123, 163, 143, 8, 179, 52, 18, 168, 140, 193, 238, 120, 215, 59, 174, 231, 168, 22, 92, 124, 114, 78, 51, 15, 129]}],"peeks":[],"outputs":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,231,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"UpdateKittiesName"}},"input_utxo_list":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,231,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/update-td-kitty-name

```
### Tradable kitty price update :
Rest API is used for updating the price of tradable kitty.

**1. Get Transaction and Input UTXO List for price update of tradable kitty:**

**end point:**:get-txn-and-inpututxolist-for-td-kitty-name-update

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-new-name:**  New name of the kitty

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 394bd079207af3e0b1a9b1eb1dc40d5d5694bd1fd904d56b96d6fad0039b1f7c" -H "kitty-new-name: jbbl" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-td-kitty-name-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:update-td-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0xb696b071fdbdca1adcec9149d21a167a04d851693e97b70900ac7547e23c0d0e","index":0},"redeemer":[232, 135, 109, 225, 49, 100, 3, 154, 233, 14, 37, 46, 219, 87, 87, 126, 194, 46, 21, 194, 58, 138, 235, 176, 121, 59, 164, 20, 98, 31, 165, 109, 121, 81, 63, 97, 243, 214, 105, 123, 163, 143, 8, 179, 52, 18, 168, 140, 193, 238, 120, 215, 59, 174, 231, 168, 22, 92, 124, 114, 78, 51, 15, 129]}],"peeks":[],"outputs":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,231,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"UpdateKittiesName"}},"input_utxo_list":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,97,109,105,116,231,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/update-td-kitty-name
```

### De-List kitty from sale :
Rest API is used for removing a tradable Kitty from the sale, converting it back to a Basic Kitty without an associated price.

**1. Get Transaction and Input UTXO List for delist-kitty-from-sale:**

**end point:**:get-txn-and-inpututxolist-for-delist-kitty-from-sale

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna:95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-delist-kitty-from-sale

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:update-td-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0xe680ce989ddaa35c7ed9f3ec1f48ff956457e00a9f4635bd97f2e682cf7e300a","index":0},"redeemer":[74, 200, 62, 251, 42, 74, 130, 155, 97, 200, 209, 13, 99, 178, 179, 5, 181, 124, 177, 221, 67, 131, 151, 81, 188, 224, 7, 56, 253, 244, 36, 76, 23, 177, 67, 218, 177, 229, 88, 178, 78, 42, 182, 143, 133, 172, 75, 96, 169, 132, 83, 203, 16, 210, 96, 190, 19, 118, 84, 78, 40, 56, 236, 128]}],"peeks":[],"outputs":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"TradableKitty":"DelistKittiesFromSale"}},"input_utxo_list":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,98,98,108,231,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"type_id":[116,100,107,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}]}' \
  http://localhost:3000/delist-kitty-from-sale

```

### kitty name update :
Rest API is used for updating the name of basic kitty.

**1. Get Transaction and Input UTXO List for name update of kitty:**

**end point:**:get-txn-and-inpututxolist-for-kitty-name-update

**DNA of kitty:**  Input the DNA of kitty. Note it should start without 0X. Example 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de

**Public_key of owner of kitty:**  Public key of owner: Note it should start without 0X. Example: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67

**kitty-new-name:**  New name of the kitty

```sh
$ curl -X GET -H "Content-Type: application/json" -H "kitty-dna: 95b951b609a4434b19eb4435dc4fe3eb6f0102ff3448922d933e6edf6b14f6de" -H "kitty-new-name: jram" -H "owner_public_key: d2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67" http://localhost:3000/get-txn-and-inpututxolist-for-kitty-name-update

```
 **2. Perform Actual Operation i.e send the signed transaction to the blockchain via web service:**

 **end point:**:update-kitty-name

**signed_transaction:**: Send the signed transaction. i.e all inputs should have a redeemer to prove the ownership of spending or usage.

 ```sh
$ curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "signed_transaction": {"inputs":[{"output_ref":{"tx_hash":"0x9492d8c80fb5a8cf2720c0072d00c91c821502894fa4482a9c99fc027bf22daf","index":0},"redeemer":[132, 84, 163, 3, 64, 12, 74, 150, 176, 70, 223, 124, 252, 222, 23, 187, 141, 55, 207, 97, 55, 172, 128, 201, 147, 148, 8, 228, 108, 113, 36, 24, 10, 118, 178, 195, 8, 124, 127, 238, 172, 23, 127, 249, 203, 109, 196, 101, 76, 64, 162, 102, 184, 93, 63, 187, 193, 247, 129, 94, 44, 84, 200, 141]}],"peeks":[],"outputs":[{"payload":{"data":[1,0,2,0,0,0,0,0,0,0,57,75,208,121,32,122,243,224,177,169,177,235,29,196,13,93,86,148,189,31,217,4,213,107,150,214,250,208,3,155,31,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,114,97,109],"type_id":[75,105,116,116]},"verifier":{"Sr25519Signature":{"owner_pubkey":"0xd2bf4b844dfefd6772a8843e669f943408966a977e3ae2af1dd78e0f55f4df67"}}}],"checker":{"FreeKitty":"UpdateKittiesName"}}}' \
  http://localhost:3000/update-kitty-name

```


