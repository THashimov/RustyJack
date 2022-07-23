use crate::{player_manager::{Players, self, Player}, game_logic, card_manager::Shoe};

pub enum BestMove {
    Hit,
    Stand,
    Split,
    Double
}


pub fn ai_turn(players: &mut Players, shoe: &mut Shoe) {
    for i in 1..players.players.len() {
        match check_for_best_move(&mut players.players[i], &mut players.dealer) {
            BestMove::Hit => {game_logic::hit(&mut players.players[i], shoe)},
            BestMove::Stand => {},
            BestMove::Split => {game_logic::split(&mut players.players[i], shoe)},
            BestMove::Double => {game_logic::hit(&mut players.players[i], shoe)},
        }
    }
}

fn check_for_best_move(player: &mut Player, dealer: &mut Player) -> BestMove {
    let hint = player_manager::return_hint(player, dealer);

    let mut hint_str = String::new();

    if let Some(some_str) = hint {
        hint_str = some_str;
    }

    hint_str = hint_str.to_ascii_lowercase();

    match hint_str.as_str() {
        "hit" => return BestMove::Hit,
        "stand" => return BestMove::Stand,
        "split" => return BestMove::Split,
        "double" => return BestMove::Double,
        _ => panic!("Failed to get best move")
    }
}