mod card_manager;
mod game_logic;
mod player_input_manager;
mod player_manager;
mod tests;
mod window_manager;

use std::{thread, time::Duration};

use card_manager::Shoe;
use player_manager::Players;
use window_manager::WindowManager;

fn main() {
    let mut window = WindowManager::new_window();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("./src/assets/fonts/Raleway-Black.ttf", 128).unwrap();

    window.canvas.clear();
    window.load_background();
    let mut shoe = Shoe::create_shoe();
    let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);
    players.draw_second_card_for_every_player(&mut shoe);
    
    window.render_initial_cards(&mut players);
    window.render_balance_and_bet_text(&mut players.player_one, &font);
    window.render_instructions(&font);

    window.refresh_screen();

    'running: loop {
        window.render_updated_bank_ballance(& players.player_one);
        if player_input_manager::check_for_key_press(&mut window.event_pump, &mut players.player_one) {
            break 'running;
        }
    }
}
