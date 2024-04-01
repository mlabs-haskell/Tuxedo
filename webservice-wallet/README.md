# Tuxedo webservice functionality

A REST API for communicating with Tuxedo node template.

## Overview

Cryptokitties is a decentralized application (DApp) built on the Tuxedo Blockchain that allows users to create, trade, breed, and manage virtual cats known as "Kitties". This README provides an overview of the available operations and REST APIs for interacting with the Cryptokitties platform.

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
