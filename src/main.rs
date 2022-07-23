mod card_manager;
mod game_logic;
mod player_input_manager;
mod player_manager;
mod split_logic;
mod ai;
mod tests;
mod window_manager;

use card_manager::Shoe;
use player_input_manager::KeyStroke;
use player_manager::Players;
use window_manager::WindowManager;

fn main() {
    let mut window = WindowManager::new_window();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(
        "/home/mighty/projects/RustyJack/src/assets/fonts/Raleway-Black.ttf",
        128,
    );

    let font = match font {
        Ok(font) => font,
        Err(err) => panic!("{}", err),
    };

    let mut shoe = Shoe::create_shoe();
    let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);

    players.deal_cards(&mut shoe, &window.window_size);

    'running: loop {
        match player_input_manager::check_for_key_press(
            &mut window.event_pump,
            &mut players,
            &mut shoe,
        ) {
            KeyStroke::Quit => break 'running,
            KeyStroke::DealAgain => {
                game_logic::deal_again(&mut players, &mut shoe, &window.window_size)
            }
            KeyStroke::KeepPlaying => {}
            KeyStroke::Counter => {
                if window.show_counter == true {
                    window.show_counter = false
                } else {
                    window.show_counter = true
                }
            }
            KeyStroke::Hint => {
                if window.show_hint == true {
                    window.show_hint = false
                } else {
                    window.show_hint = true
                }
            }
        }

        if !players.players[0].all_hands_played {
            game_logic::check_for_blackjack_and_bust(&mut players.players[0]);
        }

        if players.players[0].all_hands_played {
            if players.players[0].has_checked && !players.players[0].has_split {
                game_logic::stand(&mut players.dealer, &mut shoe);
                game_logic::check_for_winner(&mut players);
                players.players[0].has_checked = false;
                ai::ai_turn(&mut players, &mut shoe);
            } else if players.players[0].has_checked && players.players[0].has_split {
                split_logic::check_split_hands_for_win_and_update_winnings(&mut players);
                ai::ai_turn(&mut players, &mut shoe);
                players.players[0].has_checked = false;
            };
        };

        if shoe.shoe.len() < 50 {
            shoe = Shoe::create_shoe()
        }

        window.refresh_screen(&mut players, &shoe, &font);
    }
}
