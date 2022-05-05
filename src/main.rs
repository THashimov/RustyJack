mod card_manager;
mod player_input_manager;
mod window_manager;
mod game_logic;
mod player_manager;
mod tests;

use card_manager::Shoe;
use window_manager::WindowManager;
use player_manager::Players;

fn main() {
    let mut window = WindowManager::new_window();
    window.load_background();
    let mut shoe = Shoe::create_shoe();
    let players = Players::init_players_and_dealer(&mut shoe);

    window.render_card(&players.player_one.hand[0].img_src);
    window.refresh_screen();

    'running: loop {
        if player_input_manager::check_for_key_press(&mut window.event_pump) {
            break 'running;
        }
    }
}
