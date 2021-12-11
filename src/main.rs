mod server;
use server::Server;
extern crate chrono;
use std::{env, io::Error};
use std::sync::{Mutex,Arc};
use tokio::net::{TcpListener};
pub mod State{
    pub mod state;
    pub mod state_types;
}
pub mod DataStore{
    pub mod db_models;
    pub mod delete_queries;
    pub mod creation_queries;
    pub mod insert_queries;
    pub mod select_queries;
    pub mod sql_execution_handler;
    pub mod update_queries;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let _ = env_logger::try_init();
    let addr =  "127.0.0.1:8080".to_string();
    
    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    let mut state_holder = Arc::new(Mutex::new(State::state::ServerState::new()));
    
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(Server::accept_connection(stream,state_holder.clone()));
    }
    Ok(())
}
