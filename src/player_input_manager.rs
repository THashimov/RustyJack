use crate::{card_manager::Shoe, game_logic, player_manager::Players};
use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub enum QuitOrDeal {
    DealAgain,
    Quit,
    KeepPlaying,
}

pub fn check_for_key_press(
    event_pump: &mut EventPump,
    players: &mut Players,
    shoe: &mut Shoe,
) -> QuitOrDeal {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return QuitOrDeal::Quit,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => game_logic::increase_bet(&mut players.player_one),
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => game_logic::decrease_bet(&mut players.player_one),
            Event::KeyUp {
                keycode: Some(Keycode::H),
                ..
            } => {
                players.player_one.can_change_bet = false;
                game_logic::hit(&mut players.player_one, shoe)
            }
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => {
                if players.player_one.has_won
                    || players.player_one.is_bust
                    || players.dealer.has_won
                {
                    return QuitOrDeal::DealAgain;
                }
            }
            _ => {}
        }
    }
    return QuitOrDeal::KeepPlaying;
}
