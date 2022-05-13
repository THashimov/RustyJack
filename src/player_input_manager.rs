use sdl2::{event::Event, keyboard::Keycode, EventPump};
use crate::{game_logic, player_manager::Player, card_manager::Shoe};


pub fn check_for_key_press(event_pump: &mut EventPump, player: &mut Player, shoe: &mut Shoe) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => {game_logic::increase_bet(player)},
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => {game_logic::decrease_bet(player)},
            Event::KeyUp {
                keycode: Some(Keycode::H),
                ..
            } => {game_logic::hit(player, shoe)},
            _ => {}
        }
    }
    return false;
}
