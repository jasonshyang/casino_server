use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{broadcast, Mutex};
use tokio::net::TcpStream;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use crate::casino::Casino;
use crate::command::handle_command;
use colored::*;

pub async fn handle_connection(
    mut socket: TcpStream,
    casino: Arc<Mutex<Casino>>,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
    client_id_counter: Arc<AtomicUsize>,
) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all("Enter your name: ".as_bytes()).await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let client_name = line.trim().to_string();
    let client_id = client_id_counter.fetch_add(1, Ordering::SeqCst);
    println!("{}: {}, {}", "New client".green(), client_name, client_id);
    line.clear();

    casino.lock().await.add_player(client_id as u32, client_name.clone());

    writer.write_all(format!("Welcome, {}! You can use the commands 'balance' and 'bet <amount>'.\n", client_name).as_bytes()).await.unwrap();
    writer.flush().await.unwrap();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result.unwrap() == 0 {
                    break;
                }
                let command = line.trim();
                let exit = handle_command(
                    &mut writer,
                    command,
                    &casino,
                    client_id,
                    &tx,
                ).await;
                if exit { break; }
                line.clear();
            }
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        writer.write_all(msg.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(_) => break,
                }
            }
        }
    }
    casino.lock().await.remove_player(client_id as u32);
    println!("{}: {}", "Client disconnected".red(), client_id);
}