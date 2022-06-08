use crate::card_manager::{Card, Shoe};

#[derive(Debug)]
pub struct Players {
    pub dealer: Player,
    pub player_one: Player,
    pub player_two: Player,
    pub player_three: Player,
    pub player_four: Player,
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
}

impl Players {
    pub fn init_players_and_dealer(shoe: &mut Shoe, window_size: &(u32, u32)) -> Players {
        Players {
            dealer: Player::init_player(shoe.draw_card(), &window_size),
            player_one: Player::init_player(shoe.draw_card(), &window_size),
            player_two: Player::init_player(shoe.draw_card(), &window_size),
            player_three: Player::init_player(shoe.draw_card(), &window_size),
            player_four: Player::init_player(shoe.draw_card(), &window_size),
        }
    }

    pub fn draw_second_card_for_every_player(&mut self, shoe: &mut Shoe) {
        self.player_one.hand.push(shoe.draw_card());
        self.player_two.hand.push(shoe.draw_card());
        self.player_three.hand.push(shoe.draw_card());
        self.player_four.hand.push(shoe.draw_card());
    }

    pub fn set_initial_x_coords(&mut self) {
        let space_between_players = self.player_one.window_size.0 / 5;
        let start_point = self.player_one.window_size.0 - (space_between_players * 4);
        self.dealer.hand[0].coords.0 = (self.player_one.window_size.0 / 2) - 40;
        self.player_one.hand[0].coords.0 = start_point;
        self.player_two.hand[0].coords.0 = self.player_one.hand[0].coords.0 + space_between_players;
        self.player_three.hand[0].coords.0 =
            self.player_two.hand[0].coords.0 + space_between_players;
        self.player_four.hand[0].coords.0 =
            self.player_three.hand[0].coords.0 + space_between_players;

        self.player_one.hand[1].coords.0 = self.player_one.hand[0].coords.0 + 20;
        self.player_two.hand[1].coords.0 = self.player_two.hand[0].coords.0 + 20;
        self.player_three.hand[1].coords.0 = self.player_three.hand[0].coords.0 + 20;
        self.player_four.hand[1].coords.0 = self.player_four.hand[0].coords.0 + 20;
    }

    pub fn set_initial_y_coords(&mut self, window_size: &(u32, u32)) {
        let dealer_y_coord = window_size.1 / 4;
        let player_y_coord = dealer_y_coord + dealer_y_coord * 2;

        self.dealer.hand[0].coords.1 = dealer_y_coord;
        self.player_one.hand[0].coords.1 = player_y_coord;
        self.player_two.hand[0].coords.1 = player_y_coord;
        self.player_three.hand[0].coords.1 = player_y_coord;
        self.player_four.hand[0].coords.1 = player_y_coord;

        self.player_one.hand[1].coords.1 = player_y_coord - 20;
        self.player_two.hand[1].coords.1 = player_y_coord - 20;
        self.player_three.hand[1].coords.1 = player_y_coord - 20;
        self.player_four.hand[1].coords.1 = player_y_coord - 20;
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
        }
    }
}
