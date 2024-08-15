use alloy::providers::{Provider, ProviderBuilder};
use crate::config::Config;
use crate::db::connection::DbConn;
pub async fn start_listener(config, db_conn)