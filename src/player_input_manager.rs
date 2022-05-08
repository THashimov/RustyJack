use sdl2::{event::Event, keyboard::Keycode, EventPump};
use crate::{game_logic, player_manager::Player};


pub fn check_for_key_press(event_pump: &mut EventPump, player: &mut Player) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {game_logic::increase_bet(player)},
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {game_logic::decrease_bet(player)},
            _ => {}
        }
    }
    return false;
}
