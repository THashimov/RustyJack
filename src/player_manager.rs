use crate::card_manager::{Shoe, Card};

#[derive(Debug)]
pub struct Players {
    pub dealer: Player,
    pub player_one: Player,
    pub player_two: Player,
    pub player_three: Player,
    pub player_four: Player
}
#[derive(Debug)]
pub struct Player {
    pub bet: u32,
    pub bank_balance: u32,
    pub hand: Vec<Card>
}

impl Players {
    pub fn init_players_and_dealer(shoe: &mut Shoe) -> Players {
        Players { 
            dealer: Player::init_player(shoe.draw_card()),
            player_one: Player::init_player(shoe.draw_card()),
            player_two: Player::init_player(shoe.draw_card()),
            player_three: Player::init_player(shoe.draw_card()), 
            player_four: Player::init_player(shoe.draw_card()),
        }
    }
}

impl Player {
    fn init_player(card: Card) -> Player {
        let hand = vec![card];
        Player {
            bet: 20,
            bank_balance: 200,
            hand
        }
    }
}