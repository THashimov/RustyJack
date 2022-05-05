mod card_manager;
mod player_input_manager;
mod window_manager;
mod game_logic;
mod tests;

use card_manager::Shoe;
use window_manager::WindowManager;

fn main() {
    let mut window = WindowManager::new_window();
    window.load_background();
    let mut shoe = Shoe::create_shoe();

    window.refresh_screen();
    'running: loop {
        if player_input_manager::check_for_key_press(&mut window.event_pump) {
            break 'running;
        }
    }
}
