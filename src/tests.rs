#[cfg(test)]
mod tests {
    use crate::card_manager::{self, Card, Shoe, Suit};
    use crate::player_input_manager;
    use crate::player_manager::{Players, WhoseTurn};
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
                img_src: "./src/assets/2C.png".to_string()
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

        let mut src = String::new();
        let mut coords = (0, 0);

        src = players.dealer.hand[0].img_src.clone();
        coords = players.dealer.coords;
        window.render_card(&src, coords);
        players.set_player_coords();
        window.refresh_screen();
       
    }
}
