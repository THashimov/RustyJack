#[cfg(test)]
mod tests {
    use crate::card_manager::{self, Card, Shoe, Suit};
    use crate::game_logic;
    use crate::player_manager::Players;
    use crate::window_manager::WindowManager;
    extern crate rand;

    #[test]
    fn get_img_src_for_card() {
        let value = 14;
        let suit = Suit::Diamonds;
        let mut path = String::from("./src/assets/");

        match value {
            2 => path.push_str("2"),
            3 => path.push_str("3"),
            4 => path.push_str("4"),
            5 => path.push_str("5"),
            6 => path.push_str("6"),
            7 => path.push_str("7"),
            8 => path.push_str("8"),
            9 => path.push_str("9"),
            10 => path.push_str("10"),
            11 => path.push_str("J"),
            12 => path.push_str("Q"),
            13 => path.push_str("K"),
            14 => path.push_str("A"),
            _ => {}
        }

        match suit {
            Suit::Diamonds => path.push_str("D.png"),
            Suit::Hearts => path.push_str("H.png"),
            Suit::Clubs => path.push_str("C.png"),
            Suit::Spades => path.push_str("A.png"),
        }

        assert_eq!(path, "./src/assets/AD.png");
    }

    #[test]
    fn create_deck() {
        let mut deck: Vec<Card> = Vec::new();

        let mut value = 2;
        let mut suit = Suit::Clubs;
        let mut path = String::new();

        for _i in 0..4 {
            match _i {
                1 => suit = Suit::Spades,
                2 => suit = Suit::Diamonds,
                3 => suit = Suit::Hearts,
                _ => {}
            };

            for _j in 0..13 {
                let img_src = card_manager::get_img_src_for_card(Some(value), Some(suit));

                if let Some(str) = img_src {
                    path = str
                }

                let card = Card::create_card(value, suit, path.clone());

                value += 1;
                if value > 14 {
                    value = 2
                }
                deck.push(card)
            }
        }

        assert_eq!(
            deck[0],
            Card {
                value: 2,
                suit: Suit::Clubs,
                img_src: "./src/assets/2C.png".to_string(),
                coords: (0, 0)
            }
        )
    }

    #[test]
    fn create_shoe() {
        let mut shoe: Vec<Card> = Vec::new();

        let mut value = 2;
        let mut suit = Suit::Clubs;
        let mut path = String::new();

        for _i in 0..6 {
            for _j in 0..4 {
                match _j {
                    1 => suit = Suit::Spades,
                    2 => suit = Suit::Diamonds,
                    3 => suit = Suit::Hearts,
                    _ => {}
                };

                for _k in 0..13 {
                    let img_src = card_manager::get_img_src_for_card(Some(value), Some(suit));

                    if let Some(str) = img_src {
                        path = str
                    }

                    let card = Card::create_card(value, suit, path.clone());

                    value += 1;
                    if value > 14 {
                        value = 2
                    }
                    shoe.push(card)
                }
            }
        }
        card_manager::shuffle_cards(&mut shoe);

        assert_eq!(shoe.len(), 312)
    }

    #[test]
    fn create_players() {
        let window = WindowManager::new_window();
        let mut shoe = Shoe::create_shoe();
        let players = Players::init_players_and_dealer(&mut shoe, &window.window_size);

        assert_eq!(players.players.len(), 4);
    }

    #[test]
    fn get_bank_balance() {
        let mut shoe = Shoe::create_shoe();
        let players = Players::init_players_and_dealer(&mut shoe, &(1000, 1000));

        assert_eq!(players.players[0].bank_balance, 200)
    }

    #[test]
    fn increase_bet() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(1000, 1000));

        players.players[0].bank_balance = players.players[0].bank_balance - players.players[0].bet;

        assert_eq!(players.players[0].bank_balance, 180)
    }

    #[test]
    fn check_for_bust() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(0, 0));

        players.players[0].hand.push(shoe.draw_card());
        players.players[0].hand.push(shoe.draw_card());

        players.players[0].hand[0].value = 10;
        players.players[0].hand[1].value = 15;


        let hand_val = game_logic::get_hand_value(&players.players[0].hand);

        if hand_val > 21 {
            players.players[0].is_bust = true
        }

        assert_eq!(players.players[0].is_bust, true);
    }

    #[test]
    fn deal_again() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(0, 0));

        players.dealer.hand.drain(..);

        for i in 0..players.players.len() {
            players.players[i].hand.drain(..);
        }

        players.players[0].has_won = false;
        players.players[0].is_bust = false;
        players.players[0].can_change_bet = true;

        println!("Hand should be empty {:?}", players.players[0].hand)
    }

    #[test]
    fn change_aces() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(0, 0));
        let player = &mut players.players[0];

        player.hand.drain(..);

        for _i in 0..3 {
            let card = Card::create_card(11, Suit::Diamonds, "./src/assets/AD.png".to_string());
            player.hand.push(card);
        }

        let has_ace = game_logic::check_for_ace(&player.hand);
        let mut hand_val = game_logic::get_hand_value(&player.hand);

        // if hand > 21 Iter over hand and look for aces

        if hand_val > 21 && has_ace {
            'change_ace: loop {
                for i in 0..player.hand.len() {
                    if player.hand[i].value == 11 {
                        player.hand[i].value = 1;
                        hand_val = game_logic::get_hand_value(&player.hand);
                        if hand_val < 21 {
                            break 'change_ace;
                        }
                    }
                }
            }
        }

        assert_eq!(hand_val, 13);
    }

    #[test]
    fn stand() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(1000, 1000));
        players.deal_cards(&mut shoe, &(1000, 1000));
        let mut dealer = players.dealer;
        // dealer must draw to 16 and stand on 17

        while game_logic::get_hand_value(&dealer.hand) < 17 {
            let mut card = shoe.draw_card();
            let index = dealer.hand.len();

            let mut coords = dealer.hand[index - 1].coords;
            coords.0 -= 20;
            coords.1 += 20;
            card.coords = coords;
            dealer.hand.push(card);

            dealer.hand[0].value = 10;
            dealer.hand[1].value = 10;
            game_logic::change_aces(&mut dealer);
        }
        assert_eq!(game_logic::get_hand_value(&dealer.hand), 20);
    }

    #[test]
    fn check_for_winner() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(1000, 1000));
        players.deal_cards(&mut shoe, &(1000, 1000));
        let mut dealer = players.dealer;
        let mut player = &mut players.players[0];
        dealer.hand.push(shoe.draw_card());

        dealer.hand[0].value = 10;
        dealer.hand[1].value = 9;

        player.hand[0].value = 10;
        player.hand[1].value = 8;

        if game_logic::get_hand_value(&player.hand) > game_logic::get_hand_value(&dealer.hand) {
            player.has_won = true
        } else {
            dealer.has_won = true
        }

        assert_eq!(dealer.has_won, true)
    }
}