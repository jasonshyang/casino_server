use crate::player::Player;
use colored::*;
use std::collections::HashMap;
use rand::Rng;

pub struct Casino {
    players: HashMap<u32, Player>,
    bets_placed: usize,
}

impl Casino {
    const ODDS: f64 = 1.9;

    pub fn new() -> Casino {
        Casino {
            players: HashMap::new(),
            bets_placed: 0,
        }
    }

    pub fn add_player(&mut self, id: u32, name: String) {
        println!("{}: {}", "New player".green(), name);
        self.players.insert(id, Player::new(id, name));
    }

    pub fn remove_player(&mut self, id: u32) {
        self.players.remove(&id);
    }

    pub fn get_player(&self, id: u32) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    pub fn get_balance(&self, id: u32) -> Option<f64> {
        self.players.get(&id).map(|player| player.balance)
    }

    pub fn new_bet(&mut self) {
        self.bets_placed += 1;
    }

    pub fn all_bets_placed(&self) -> bool {
        self.bets_placed == self.players.len()
    }

    pub fn all_roll(&mut self) {
        for player in self.players.values_mut() {
            player.roll(Self::roll_dice());
        }
        self.bets_placed = 0;
    }

    pub fn resolve_bets(&mut self) -> String{
        let dealer_roll = Self::roll_dice();
        println!("{}: {}", "Dealer roll".green(), dealer_roll);

        let mut results = format!("Dealer roll: {}\n", dealer_roll);

        for player in self.players.values_mut() {
            if player.current_dice_roll > dealer_roll {
                let winnings = player.current_bet * Self::ODDS;
                player.balance += winnings;
                results.push_str(&format!("{}: Won {} with roll {}\n", player.name, winnings, player.current_dice_roll));
                println!("{}: {}, {}", "Winner".green(), player.name, winnings);
            } else {
                results.push_str(&format!("{}: Lost {} with roll {}\n", player.name, player.current_bet, player.current_dice_roll));
                println!("{}: {}, {}", "Loser".red(), player.name, player.current_bet);
            }
            player.current_bet = 0.0;
        }

        results
    }

    fn roll_dice() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=10)
    }
}
