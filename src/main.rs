mod card_manager;
mod game_logic;
mod player_input_manager;
mod player_manager;
mod tests;
mod window_manager;

use card_manager::Shoe;
use player_manager::Players;
use window_manager::WindowManager;

fn main() {
    let mut window = WindowManager::new_window();
    window.load_background();
    let mut shoe = Shoe::create_shoe();
    let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);
    players.draw_second_card_for_every_player(&mut shoe);
   
    window.render_initial_cards(&mut players);
    window.refresh_screen();

    'running: loop {
        if player_input_manager::check_for_key_press(&mut window.event_pump) {
            break 'running;
        }
    }
}
