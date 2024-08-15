// use crate::config::Config;
// use crate::db::connection::DbConn;

use alloy::{primitives::address, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::{BlockNumberOrTag, Filter}, sol};
use futures_util::stream::StreamExt;

sol!(
    event PairCreated(address indexed token0, address indexed token1, address pair, uint);
);
pub async fn start_listener() {
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "";

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let uniswap_token_address = address!("8909Dc15e40173Ff4699343b6eB8132c65e18eC6");
    let filter = Filter::new()
        .address(uniswap_token_address)
        // By specifying an `event` or `event_signature` we listen for a specific event of the
        // contract. In this case the `Transfer(address,address,uint256)` event.
        .event("PairCreated(address,address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await.unwrap();
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let PairCreated { token1, token0, pair, ..} = log.log_decode().unwrap().inner.data;
        // check basic token data -> post to endpoint
        // load token to database -> post to endpoint
        // load pair to database -> post to endpoint
        // trade
        // load trade to database
        println!("Token0: {}, Token1: {}, Pair: {}", token0, token1, pair);
    }
}