use std::sync::{atomic::AtomicUsize, Arc};
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use colored::*;
use casino_server::casino::Casino;
use casino_server::handler::handle_connection;

#[tokio::main]
async fn main() {
    let casino = Arc::new(Mutex::new(Casino::new()));
    println!("{}", "Casino server started".green());
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    let (tx, _rx) = broadcast::channel::<String>(100);
    let client_id_counter = Arc::new(AtomicUsize::new(1));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("{}: {}", "New connection".green(), addr);
        let casino = casino.clone();
        let tx = tx.clone();
        let rx = tx.subscribe();
        let client_id_counter = client_id_counter.clone();

        tokio::spawn(async move {
            handle_connection(
                socket,
                casino,
                tx,
                rx,
                client_id_counter,
            ).await;
        });
    }

}
