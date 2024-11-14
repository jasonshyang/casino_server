use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;
use tokio::sync::{broadcast, Mutex};
use std::sync::Arc;
use colored::*;

use crate::casino::Casino;

pub async fn handle_command(
    writer: &mut WriteHalf<'_>,
    command: &str,
    casino: &Arc<Mutex<Casino>>,
    client_id: usize,
    tx: &broadcast::Sender<String>,
) -> bool {
    let command = command.trim();
    println!("{} Player {}: <{}>", "Command".cyan(), client_id, command);

    if command == "balance" {
        let balance = casino.lock().await.get_balance(client_id as u32);
        match balance {
            Some(balance) => writer.write_all(format!("Your balance is: {}\n", balance).as_bytes()).await.unwrap(),
            None => writer.write_all(b"Invalid client\n").await.unwrap(),
        }
    } else if command.starts_with("bet ") {
        let bet: Vec<&str> = command.split_whitespace().collect();
        if bet.len() == 2 {
            if let Ok(amount) = bet[1].parse::<f64>() {
                let mut casino = casino.lock().await;
                casino.new_bet();
                let player = casino.get_player_mut(client_id as u32).unwrap();
                match player.dice_bet(amount) {
                    Ok(()) => {
                        let msg = format!("Player {} bet: {}\n", player.name, amount);
                        println!("Player {} bet: {}", player.name, amount);
                        tx.send(msg).unwrap();
                        writer.write_all(format!("You bet: {}\n", amount).as_bytes()).await.unwrap();

                        if casino.all_bets_placed() {
                            casino.all_roll();
                            let results = casino.resolve_bets();
                            tx.send(results).unwrap();
                        }
                    }
                    Err(e) => {
                        writer.write_all(format!("Error: {}\n", e).as_bytes()).await.unwrap();
                    }
                } 
            } else {
                    writer.write_all(b"Invalid amount\n").await.unwrap();
            }
        } else {
            writer.write_all(b"Invalid command\n").await.unwrap();
        }
    } else if command == "exit" {
        writer.write_all(b"Goodbye!\n").await.unwrap();
        return true;
    } else {
        writer.write_all(b"Invalid command\n").await.unwrap();
    }

    false
}
