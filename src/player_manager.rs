use crate::card_manager::{Card, Shoe};

#[derive(Debug)]
pub struct Players {
    pub players: Vec<Player>,
    pub dealer: Player,
}

#[derive(Debug)]
pub struct Player {
    pub bet: u32,
    pub bank_balance: u32,
    pub hand: Vec<Card>,
    pub window_size: (u32, u32),
    pub can_change_bet: bool,
    pub has_checked: bool,
    pub is_bust: bool,
    pub has_won: bool,
    pub has_blackjack: bool,
    pub has_finished_dealing: bool,
    pub can_split: bool
}

impl Players {
    pub fn init_players_and_dealer(shoe: &mut Shoe, window_size: &(u32, u32)) -> Players {
        let mut players = Vec::new();

        for _i in 0..4 {
            players.push(Player::init_player(shoe.draw_card(), &window_size));
        }

        Players {
            players,
            dealer: Player::init_player(shoe.draw_card(), &window_size),
        }
    }

    pub fn draw_second_card_for_every_player(&mut self, shoe: &mut Shoe) {
        for i in 0..self.players.len() {
            self.players[i].hand.push(shoe.draw_card())
        }
    }

    pub fn set_initial_x_coords(&mut self) {
        let space_between_players = self.players[0].window_size.0 / 5;
        let start_point = self.players[0].window_size.0 - (space_between_players * 4);
        self.dealer.hand[0].coords.0 = (self.players[0].window_size.0 / 2) - 40;
        self.players[0].hand[0].coords.0 = start_point;

        for i in 1..self.players.len() {
            self.players[i].hand[0].coords.0 = self.players[i - 1].hand[0].coords.0 + space_between_players;
        }

        for i in 0..self.players.len() {
            self.players[i].hand[1].coords.0 = self.players[i].hand[0].coords.0 + 20;
        }
    }

    pub fn set_initial_y_coords(&mut self, window_size: &(u32, u32)) {
        let dealer_y_coord = window_size.1 / 4;
        let player_y_coord = dealer_y_coord + dealer_y_coord * 2;

        self.dealer.hand[0].coords.1 = dealer_y_coord;

        for i in 0..self.players.len() {
            self.players[i].hand[0].coords.1 = player_y_coord;
            self.players[i].hand[1].coords.1 = player_y_coord - 20;
        }
    }

    pub fn deal_cards(&mut self, shoe: &mut Shoe, window_size: &(u32, u32)) {
        self.draw_second_card_for_every_player(shoe);
        self.set_initial_x_coords();
        self.set_initial_y_coords(&window_size);
    }
}

impl Player {
    fn init_player(card: Card, window_size: &(u32, u32)) -> Player {
        let hand = vec![card];
        Player {
            bet: 20,
            bank_balance: 200,
            hand,
            window_size: *window_size,
            can_change_bet: true,
            has_checked: false,
            is_bust: false,
            has_won: false,
            has_blackjack: false,
            has_finished_dealing: false,
            can_split: false
        }
    }
}
