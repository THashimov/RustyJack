mod card_manager;
mod game_logic;
mod player_input_manager;
mod player_manager;
mod split_logic;
mod tests;
mod window_manager;

use card_manager::Shoe;
use player_input_manager::QuitOrDeal;
use player_manager::Players;
use window_manager::WindowManager;

fn main() {
    let mut window = WindowManager::new_window();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font("./src/assets/fonts/Raleway-Black.ttf", 128)
        .unwrap();

    let mut shoe = Shoe::create_shoe();
    let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);
    
    players.deal_cards(&mut shoe, &window.window_size);


    let mut hand_val = [0, 0, 0, 0];
    'running: loop {
        match player_input_manager::check_for_key_press(
            &mut window.event_pump,
            &mut players,
            &mut shoe,
        ) {
            QuitOrDeal::Quit => break 'running,
            QuitOrDeal::DealAgain => {
                game_logic::deal_again(&mut players, &mut shoe, &window.window_size)
            }
            QuitOrDeal::KeepPlaying => {}
        }

        if !players.players[0].all_hands_played {
            game_logic::check_for_blackjack_and_bust(&mut players.players[0]);
        }

        if players.players[0].all_hands_played {
            if players.players[0].has_checked && !players.players[0].has_split {
                game_logic::stand(&mut players.dealer, &mut shoe);
                game_logic::check_for_winner(&mut players);
                players.players[0].has_checked = false;
            } else if players.players[0].has_checked && players.players[0].has_split {
                split_logic::check_split_hands_for_win(&mut players);
                players.players[0].has_checked = false;
            };
        };

        if shoe.shoe.len() < 50 {
            shoe = Shoe::create_shoe()
        }

        // if players.players[0].is_bust[players.players[0].which_hand_being_played]
        // || game_logic::get_hand_value(&players.players[0].hands[players.players[0].which_hand_being_played].hand) == 21
    // {
        // split_logic::change_hand_being_played(&mut players.players[0]);
    // }

        window.refresh_screen(&mut players, &font);

        let h = players.players[0].which_hand_being_played;



        if players.players[0].hands.len() == 4 {
        for i in 0..hand_val.len() {
            let v = game_logic::get_hand_value(&players.players[0].hands[i].hand);
            hand_val[i] = v;
        }
    }
        println!("hand {:?}", hand_val);
        println!("bet {:?}", players.players[0].bet);
        println!("{:?}", players.players[0].is_bust)
    }
}
