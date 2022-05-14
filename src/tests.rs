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

        println!("{}", path);
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

        println!("{:?}", players);
    }

    #[test]
    fn render_initial_hands() {
        let mut window = WindowManager::new_window();
        window.load_background();

        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);

        players.draw_second_card_for_every_player(&mut shoe);

        window.render_cards(&mut players);
        players.set_initial_x_coords();
    }

    #[test]
    fn get_bank_balance() {
        let mut shoe = Shoe::create_shoe();
        let window = WindowManager::new_window();
        let players = Players::init_players_and_dealer(&mut shoe, &window.window_size);

        let bank = players.player_one.bank_balance;

        println!("{}", bank)
    }

    #[test]
    fn increase_bet() {
        let mut shoe = Shoe::create_shoe();
        let window = WindowManager::new_window();
        let players = Players::init_players_and_dealer(&mut shoe, &window.window_size);
        let mut player_obj = players.player_one;

        player_obj.bank_balance = player_obj.bank_balance - player_obj.bet;

        assert_eq!(player_obj.bank_balance, 180)
    }

    #[test]
    fn assign_card_values() {
        let mut shoe = Shoe::create_shoe();
        for i in 0..shoe.shoe.len() {
            if shoe.shoe[i].value > 10 && shoe.shoe[i].value < 14 {
                shoe.shoe[i].value = 10
            } else if shoe.shoe[i].value == 14 {
                shoe.shoe[i].value = 11
            }
        }

        assert_eq!(shoe.shoe[7].value, 9);
        assert_eq!(shoe.shoe[10].value, 10);
        assert_eq!(shoe.shoe[11].value, 10);
        assert_eq!(shoe.shoe[12].value, 11);
    }

    #[test]
    fn check_for_bust() {
        let mut shoe = Shoe::create_shoe();
        let players = Players::init_players_and_dealer(&mut shoe, &(0, 0));
        let mut player_one = players.player_one;

        player_one.hand.push(shoe.draw_card());
        player_one.hand.push(shoe.draw_card());

        let mut hand_val = 0;

        for i in 0..player_one.hand.len() {
            hand_val += player_one.hand[i].value
        }

        if hand_val > 21 {
            player_one.is_bust = true
        }

        println!("{:?}", player_one.hand);
        println!("{:?}", hand_val);
        println!("{:?}", player_one.is_bust);
    }

    #[test]
    fn deal_again() {
        let mut shoe = Shoe::create_shoe();
        let mut players = Players::init_players_and_dealer(&mut shoe, &(0, 0));
        players.player_one.hand.push(shoe.draw_card());
        players.player_one.hand.push(shoe.draw_card());
        println!("{:?}", players.player_one.hand);

        players.dealer.hand.drain(..);
        players.player_one.hand.drain(..);
        players.player_two.hand.drain(..);
        players.player_three.hand.drain(..);
        players.player_four.hand.drain(..);

        players.dealer.hand.push(shoe.draw_card());

        players.player_one.hand.push(shoe.draw_card());
        players.player_two.hand.push(shoe.draw_card());
        players.player_three.hand.push(shoe.draw_card());
        players.player_four.hand.push(shoe.draw_card());

        players.player_one.has_won = false;
        players.player_one.is_bust = false;
        players.player_one.can_change_bet = true;


        println!("{:?}", players.player_one.hand);
    }

    #[test]
    fn change_aces() {
        let mut shoe = Shoe::create_shoe();
        let players = Players::init_players_and_dealer(&mut shoe, &(0, 0));
        let mut player = players.player_one;

        let card = Card::create_card(11, Suit::Diamonds, "./src/assets/AC.png".to_string());

        player.hand.push(card);
        player.hand.push(shoe.draw_card());

        let card = Card::create_card(11, Suit::Diamonds, "./src/assets/AD.png".to_string());

        player.hand.push(card);

        let card = Card::create_card(11, Suit::Diamonds, "./src/assets/AD.png".to_string());

        player.hand.push(card);


        let mut hand_val = game_logic::get_hand_value(&player.hand);
        let has_ace = game_logic::check_for_ace(&player.hand);

        // if hand > 21 Iter over hand and look for aces
    
        if hand_val > 21 && has_ace {
            'change_ace: loop {
            for i in 0..player.hand.len() {
                if player.hand[i].value == 11 {
                    player.hand[i].value = 1;
                    hand_val = game_logic::get_hand_value(&player.hand);
                        if hand_val < 21 {
                            break 'change_ace
                        }
                }
            }
        }
        }

        println!("{:?}", player.hand);
        println!("hand {}", hand_val);

    }

}
