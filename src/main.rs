mod config;
mod listener;
mod db;

use listener::listener::start_listener;

#[tokio::main]
async fn main() {
    // // Load configurations
    // let config = config::load_config();
    //
    // // Initialize the database connection
    // let db_conn = db::connection::establish_connection(&config.db_url);

    // Start the listener
    // start_listener(config, db_conn);

    start_listener().await;
}

