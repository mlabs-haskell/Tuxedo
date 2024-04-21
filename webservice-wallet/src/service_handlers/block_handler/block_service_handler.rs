use serde::Serialize;

use jsonrpsee::http_client::HttpClientBuilder;

use crate::rpc;
use anyhow::Error;
use crate::get_blockchain_node_endpoint;

/// The default RPC endpoint for the wallet to connect to
//const DEFAULT_ENDPOINT: &str = "http://localhost:9944";
use axum::{http::HeaderMap, Json};

use anyhow::bail;
use runtime::Block;

#[derive(Debug, Serialize)]
pub struct BlockResponse {
    pub message: String,
}

pub async fn get_block(headers: HeaderMap) -> Json<BlockResponse> {
    let block_number_header = headers.get("Block-Number").unwrap_or_else(|| {
        panic!("Block-Number header is missing");
    });
    let block_number = block_number_header.to_str().unwrap_or_else(|_| {
        panic!("Failed to parse Block-Number header");
    });

    // Convert the block number to the appropriate type if needed
    let block_number: u128 = block_number.parse().unwrap_or_else(|_| {
        panic!("Failed to parse block number as u128");
    });

    match get_blocks(block_number).await {
        Ok(Some(node_block)) => Json(BlockResponse {
            message: format!("block found {:?}", node_block),
        }),

        Ok(None) => Json(BlockResponse {
            message: format!("Node's block not found"),
        }),
        Err(err) => Json(BlockResponse {
            message: format!("Error getting the block: {:?}", err),
        }),
    }
}

async fn get_blocks(number: u128) -> anyhow::Result<Option<Block>> {
    //let client = HttpClientBuilder::default().build(DEFAULT_ENDPOINT)?;
    let client = HttpClientBuilder::default()
        .build(get_blockchain_node_endpoint().expect("Failed to get the node end point"))
        .expect("http client buider error");

    let node_block_hash = rpc::node_get_block_hash(number.try_into().unwrap(), &client)
        .await;
    //.expect("node should be able to return some hash");
    let hash = match node_block_hash {
        Ok(Some(h)) => h,
        Ok(None) => {
            return Ok(None);
        },
        Err(_) => {
            let err = Error::msg("Error fetching block hash");
            return Err(err);
        },
    };

    let maybe_block = rpc::node_get_block(hash, &client).await?;
    println!("BlockData {:?} ", maybe_block.clone().unwrap());
    match maybe_block {
        Some(block) => Ok(Some(block)),
        None => bail!("Block not found for hash: {:?}", hash),
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::HeaderMap};
    //use axum::{body::Body, http::Request};
    use crate::get_block;

    #[tokio::test]
    async fn test_get_genesis_block_success() {
        let block_number = "0";
        
        let mut headers = HeaderMap::new();
        headers.insert("Block-Number", block_number.parse().unwrap());
    
        //let response = get_block(headers).await.into_response();
        let response = get_block(headers).await;
        assert!(response.message.contains("parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000"));  
    }

    #[tokio::test]
    async fn test_get_block_success() {
        let block_number = "1";
        
        let mut headers = HeaderMap::new();
        headers.insert("Block-Number", block_number.parse().unwrap());
    
        //let response = get_block(headers).await.into_response();
        let response = get_block(headers).await;
        assert!(response.message.contains("block found ") &&
            response.message.contains("number: 1"));
    }

    #[tokio::test]
    async fn test_get_block_block_number_not_present_fail() {
        let block_number = "999999";
        
        let mut headers = HeaderMap::new();
        headers.insert("Block-Number", block_number.parse().unwrap());
    
        //let response = get_block(headers).await.into_response();
        let response = get_block(headers).await;
        assert!(response.message.contains("Node's block not found"));
    }
}


