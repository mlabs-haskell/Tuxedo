# Tuxedo web service Template Wallet 

A REST API powered  wallet for the Tuxedo Node Template

## Overview

This wallet works with the Tuxedo Node Template and Tuxedo Template Runtime which is also contained in this repository.

Like many UTXO wallets, this one synchronizes a local-to-the-wallet database of UTXOs that exist on the current best chain.
The wallet does not sync the entire blockchain state.
Rather, it syncs a subset of the state that it considers "relevant".
Currently, the wallet syncs any UTXOs that contain tokens owned by a key in the wallet's keystore.
However, the wallet is designed so that this notion of "relevance" is generalizeable.
This design allows developers building chains with Tuxedo to extend the wallet for their own needs.
However, because this is a text- based wallet, it is likely not well-suited for end users of popular dapps.

## REST Documentation
