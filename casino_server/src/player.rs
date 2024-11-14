pub struct Player {
    pub id: u32,
    pub name: String,
    pub balance: f64,
    pub current_dice_roll: u8,
    pub current_bet: f64,
}

impl Player {
    const DEFAULT_BALANCE: f64 = 100.0;
    
    pub fn new(id: u32, name: String) -> Player {
        Player {
            id,
            name,
            balance: Self::DEFAULT_BALANCE,
            current_dice_roll: 0,
            current_bet: 0.0,
        }
    }

    pub fn dice_bet(&mut self, bet: f64) -> Result<(), String> {
        if bet > self.balance {
            return Err("Insufficient balance".to_string());
        }
        self.balance -= bet;
        self.current_bet += bet;
        Ok(())
    }

    pub fn roll(&mut self, result: u8) {
        self.current_dice_roll = result;
    }
}