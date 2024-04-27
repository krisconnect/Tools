// Define the game state and player structures.
#[derive(Debug, Clone)]
pub enum GameState {
    Day,
    Night,
    GameOver,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub life: u8,
    pub is_wolf: bool,
}


pub struct WerewolfGame {
    pub players: Vec<Player>,
    pub game_state: GameState,
    pub current_turn: usize,
    pub entry_fee: u64,
    pub prize_pool: u64,
}

impl WerewolfGame {
    // Initialize a new game with players and entry fee.
    pub fn new(players: Vec<Player>, entry_fee: u64) -> Self {
        let prize_pool = entry_fee * players.len() as u64;
        WerewolfGame {
            players,
            game_state: GameState::Night,
            current_turn: 0,
            entry_fee,
            prize_pool,
        }
    }


    // Simulate the wolf's attack during the night.
    pub fn wolf_attack(&mut self) {
        require(caller.program_id = wolf) // Only wolf can call the function and only at night time (TODO)
        if let GameState::Night = self.game_state {
            let mut targets: Vec<usize> = self.players.iter()
                .enumerate()
                .filter(|(_, p)| p.life > 0 /*&& !p.is_wolf wolf can bite themselves for ultimate mindgames*/)
                .map(|(i, _)| i)
                .collect();

            if !targets.is_empty() {
                let target_index = *targets.choose().unwrap(); // Take input from ww (TODO)
                self.players[target_index].life -= 1;
            }
        }
    }


}

// // Pseudo code
// fn main() {
//     let players = vec![
//         Player { id: "Player1".to_string(), life: 2, is_wolf: false },
//         Player { id: "Player2".to_string(), life: 2, is_wolf: false },
//         Player { id: "Player3".to_string(), life: 2, is_wolf: false },
//         Player { id: "Player4".to_string(), life: 2, is_wolf: false },
//         Player { id: "Player5".to_string(), life: 2, is_wolf: false },
//     ];
//     let mut game = WerewolfGame::new(players, 10);  // Assuming entry fee is 10 SOL
//     game.assign_wolf();
//     while game.game_state != GameState::GameOver {
//         game.wolf_attack();
//         game.change_state();
//         game.check_end_conditions();
//     }
// }
