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
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("./src/assets/fonts/Raleway-Black.ttf", 128).unwrap();

    let mut shoe = Shoe::create_shoe();
    let mut players = Players::init_players_and_dealer(&mut shoe, &window.window_size);
    players.draw_second_card_for_every_player(&mut shoe);
    players.set_initial_x_coords();
    players.set_initial_y_coords(&window.window_size);

    'running: loop {
        window.render_updated_bank_ballance(&players.player_one, &font);
        if player_input_manager::check_for_key_press(&mut window.event_pump, &mut players.player_one, &mut shoe) {
            break 'running;
        }
        window.refresh_screen(&mut players, &font);
    }
}
