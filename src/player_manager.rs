use crate::{
    card_manager::{Card, Shoe, SpecialCards},
    game_logic::get_hand_value,
};

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
    pub has_won: [bool; 4],
    pub has_ace: [bool; 4],
    pub has_blackjack: [bool; 4],
    pub has_finished_dealing: bool,
    pub has_split: bool,
    pub all_hands_played: bool,
    pub has_doubled: [bool; 4],
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
            has_won: [false, false, false, false],
            has_ace: [false, false, false, false],
            has_blackjack: [false, false, false, false],
            has_finished_dealing: false,
            has_split: false,
            all_hands_played: false,
            has_doubled: [false, false, false, false],
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

pub fn return_hint(player: &mut Player, dealer: &mut Player) -> Option<String> {
    let mut hint = String::new();
    let which_hand = player.which_hand_being_played;

    let player_hand = &player.hands[which_hand].hand;

    let player_hand_val = Option::from(get_hand_value(&player_hand));
    let dealer_hand_val = Option::from(dealer.hands[0].hand[0].value);

    if player.hands[which_hand].hand[0] != player.hands[which_hand].hand[1] {
        if !player.has_ace[player.which_hand_being_played] {
            match player_hand_val {
                Some(5..=8) => hint = String::from("Hit"),
                Some(9) => match dealer_hand_val {
                    Some(3..=6) => hint = String::from("Double"),
                    Some(2 | 7..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(10) => match dealer_hand_val {
                    Some(2..=9) => hint = String::from("Double"),
                    Some(10..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(11) => match dealer_hand_val {
                    Some(2..=10) => hint = String::from("Double"),
                    Some(11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(12) => match dealer_hand_val {
                    Some(4..=6) => hint = String::from("Stand"),
                    Some(2..=3 | 7..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(13..=16) => match dealer_hand_val {
                    Some(2..=6) => hint = String::from("Stand"),
                    Some(7..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(17..=21) => hint = String::from("Stand"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            }
        }

        if player.has_ace[player.which_hand_being_played] {
            match player_hand_val {
                Some(13..=14) => match dealer_hand_val {
                    Some(5..=6) => hint = String::from("Double"),
                    Some(_) => hint = String::from("Hit"),
                    None => hint = String::from(" "),
                },
                Some(15..=16) => match dealer_hand_val {
                    Some(5..=6) => hint = String::from("Double"),
                    Some(2..=4 | 7..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(17) => match dealer_hand_val {
                    Some(3..=6) => hint = String::from("Double"),
                    Some(1..=2 | 7..=11) => hint = String::from("Hit"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(18) => match dealer_hand_val {
                    Some(3..=6) => hint = String::from("Double"),
                    Some(9..=11) => hint = String::from("Hit"),
                    Some(2 | 7..=8) => hint = String::from("Stand"),
                    Some(_) => hint = String::from(" "),
                    None => hint = String::from(" "),
                },
                Some(19..=21) => hint = String::from("Stand"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            }
        }
    } else {
        match player_hand_val {
            Some(4 | 6) => match dealer_hand_val {
                Some(2..=7) => hint = String::from("Split"),
                Some(8..=11) => hint = String::from("Hit"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(8) => match dealer_hand_val {
                Some(5..=6) => hint = String::from("Split"),
                Some(2..=4 | 7..=11) => hint = String::from("Hit"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(10) => match dealer_hand_val {
                Some(2..=9) => hint = String::from("Double"),
                Some(10..=11) => hint = String::from("Hit"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(12) => match dealer_hand_val {
                Some(2..=6) => hint = String::from("Split"),
                Some(7..=11) => hint = String::from("Hit"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(14) => match dealer_hand_val {
                Some(2..=7) => hint = String::from("Split"),
                Some(8..=11) => hint = String::from("Hit"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(16) => hint = String::from("Split"),
            Some(18) => match dealer_hand_val {
                Some(2..=6 | 8..=9) => hint = String::from("Split"),
                Some(7 | 10..=11) => hint = String::from("Stand"),
                Some(_) => hint = String::from(" "),
                None => hint = String::from(" "),
            },
            Some(20) => hint = String::from("Stand"),
            Some(22) => hint = String::from("Split"),
            Some(_) => hint = String::from(" "),
            None => hint = String::from(" "),
        }
    }

    Some(hint)
}
