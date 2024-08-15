// // config.rs
//
// use dotenv::dotenv;
// use std::env;
//
// #[derive(Debug, Clone)]
// pub struct Config {
//     pub base_https_rpc_url: String,
//     pub factory_contract_address: String,
//     pub router_address: String,
//     pub db_url: String,
//     pub private_keys: Vec<String>,
// }
//
// impl Config {
//     pub fn load() -> Config {
//         dotenv().ok();
//
//         Config {
//             base_https_rpc_url: env::var("BASE_HTTPS_RPC_URL")
//                 .expect("BASE_HTTPS_RPC_URL must be set in .env"),
//             factory_contract_address: env::var("FACTORY_CONTRACT_ADDRESS")
//                 .expect("FACTORY_CONTRACT_ADDRESS must be set in .env"),
//             router_address: env::var("ROUTER_ADDRESS")
//                 .expect("ROUTER_ADDRESS must be set in .env"),
//             db_url: env::var("DATABASE_URL")
//                 .expect("DATABASE_URL must be set in .env"),
//             private_keys: env::var("PRIVATE_KEYS")
//                 .expect("PRIVATE_KEYS must be set in .env")
//                 .split(',')
//                 .map(String::from)
//                 .collect(),
//         }
//     }
// }