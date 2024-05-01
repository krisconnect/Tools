// Transfer the entry fee from each player's account to the game account
for _ in 0..5 {
    let player_account = next_account_info(account_info_iter)?;
    **game_account.lamports.borrow_mut() += entry_fee;
    **player_account.lamports.borrow_mut() -= entry_fee;
}

// Randomly assign the role of the wolf to one of the players.
pub fn assign_wolf(&mut self) {
    use rand::Rng;
    let wolf_index = rand::thread_rng().gen_range(0..self.players.len());
    self.players[wolf_index].is_wolf = true;
}

 // Transition game state between day and night.
 pub fn change_state(&mut self) {
    self.game_state = match self.game_state {
        GameState::Day => GameState::Night,
        GameState::Night => GameState::Day,
        GameState::GameOver => return,
    };
}

// Process end of the game conditions and distribute funds.
pub fn check_end_conditions(&mut self) {
    let alive_players = self.players.iter().filter(|p| p.life > 0).count();
    let wolf_alive = self.players.iter().any(|p| p.is_wolf && p.life > 0);
    let villagers_alive = self.players.iter().filter(|p| !p.is_wolf && p.life > 0).count();

    if !wolf_alive || villagers_alive == 1 {
        self.game_state = GameState::GameOver;
        self.distribute_prize();
    }
}

// Distribute the prize pool based on the game outcome.
fn distribute_prize(&self) {
    if self.players.iter().any(|p| p.is_wolf && p.life > 0) {
        println!("Wolf wins! Prize pool of {} SOL sent to the wolf.", self.prize_pool);
    } else {
        let share = self.prize_pool / self.players.iter().filter(|p| !p.is_wolf && p.life > 0).count() as u64;
        println!("Villagers win! Each surviving villager receives {} SOL.", share);
    }
}