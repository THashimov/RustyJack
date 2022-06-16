use crate::card_manager::{Card, Shoe, SpecialCards};

#[derive(Debug)]
pub struct Players {
    pub players: Vec<Player>,
    pub dealer: Player,
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub hand: Vec<Card>,
}

impl Hand {
    pub fn new_hand(card: Vec<Card>) -> Hand {
        Hand { hand: card }
    }
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
            self.players[i].hands[0].hand.push(shoe.draw_card())
        }
    }

    pub fn set_initial_x_coords(&mut self) {
        let space_between_players = self.players[0].window_size.0 / 5;
        let start_point = self.players[0].window_size.0 - (space_between_players * 4);
        self.dealer.hands[0].hand[0].coords.0 = (self.players[0].window_size.0 / 2) - 40;
        self.players[0].hands[0].hand[0].coords.0 = start_point;
        self.players[0].split_coords_point.0 = start_point;

        for i in 1..self.players.len() {
            self.players[i].hands[0].hand[0].coords.0 =
                self.players[i - 1].hands[0].hand[0].coords.0 + space_between_players;
        }

        for i in 0..self.players.len() {
            self.players[i].hands[0].hand[1].coords.0 =
                self.players[i].hands[0].hand[0].coords.0 + 20;
        }
    }

    pub fn set_initial_y_coords(&mut self, window_size: &(u32, u32)) {
        let dealer_y_coord = window_size.1 / 4;
        let player_y_coord = dealer_y_coord + dealer_y_coord * 2;
        self.players[0].split_coords_point.1 = player_y_coord;

        self.dealer.hands[0].hand[0].coords.1 = dealer_y_coord;

        for i in 0..self.players.len() {
            self.players[i].hands[0].hand[0].coords.1 = player_y_coord;
            self.players[i].hands[0].hand[1].coords.1 = player_y_coord - 20;
        }
    }

    pub fn deal_cards(&mut self, shoe: &mut Shoe, window_size: &(u32, u32)) {
        self.draw_second_card_for_every_player(shoe);
        self.set_initial_x_coords();
        self.set_initial_y_coords(&window_size);
    }
}

#[derive(Debug)]
pub struct Player {
    pub bet: [u32; 4],
    pub bank_balance: u32,
    pub hands: Vec<Hand>,
    pub window_size: (u32, u32),
    pub split_coords_point: (u32, u32),
    pub which_hand_being_played: usize,
    pub can_change_bet: bool,
    pub has_checked: bool,
    pub is_bust: [bool; 4],
    pub has_won: bool,
    pub has_blackjack: [bool; 4],
    pub has_finished_dealing: bool,
    pub has_split: bool,
    pub all_hands_played: bool,
    pub has_doubled: bool,
}

impl Player {
    fn init_player(card: Card, window_size: &(u32, u32)) -> Player {
        let hand = vec![card];
        let hands = vec![Hand::new_hand(hand)];
        Player {
            bet: [20, 0, 0, 0],
            bank_balance: 200,
            hands,
            window_size: *window_size,
            which_hand_being_played: 0,
            split_coords_point: (0, 0),
            can_change_bet: true,
            has_checked: false,
            is_bust: [false, false, false, false],
            has_won: false,
            has_blackjack: [false, false, false, false],
            has_finished_dealing: false,
            has_split: false,
            all_hands_played: false,
            has_doubled: false,
        }
    }
}

pub fn check_if_hand_can_be_split(hand: &Vec<Card>) -> bool {
    let mut card_one = SpecialCards::None;
    let mut card_two = SpecialCards::None;

    for i in 0..2 {
        let card: Vec<char> = hand[i].img_src.chars().collect();
        for j in 0..card.len() {
            match card[j] {
                '1' => {
                    if i == 0 {
                        card_one = SpecialCards::Ten
                    } else {
                        card_two = SpecialCards::Ten
                    }
                }
                'J' => {
                    if i == 0 {
                        card_one = SpecialCards::Jack
                    } else {
                        card_two = SpecialCards::Jack
                    }
                }
                'Q' => {
                    if i == 0 {
                        card_one = SpecialCards::Queen
                    } else {
                        card_two = SpecialCards::Queen
                    }
                }
                'K' => {
                    if i == 0 {
                        card_one = SpecialCards::King
                    } else {
                        card_two = SpecialCards::King
                    }
                }
                'A' => {
                    if i == 0 {
                        card_one = SpecialCards::Ace
                    } else {
                        card_two = SpecialCards::Ace
                    }
                }
                _ => {}
            }
        }
    }

    if card_one == card_two {
        true
    } else {
        false
    }
}
