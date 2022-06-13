use crate::{
    card_manager::Shoe,
    game_logic,
    player_manager::{self, Players},
};
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
            } => game_logic::increase_bet(&mut players.players[0]),
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => game_logic::decrease_bet(&mut players.players[0]),
            Event::KeyUp {
                keycode: Some(Keycode::H),
                ..
            } => {
            if !players.players[0].is_bust && !players.players[0].has_won && !players.dealer.is_bust && !players.dealer.has_won {
                players.players[0].can_change_bet = false;
                game_logic::hit(&mut players.players[0], shoe);
            }
        }
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => {
                if !players.players[0].is_bust && !players.players[0].has_won && !players.dealer.is_bust && !players.dealer.has_won {
                    game_logic::stand(&mut players.dealer, shoe);
                    players.players[0].has_checked = true;
                };
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                if !players.players[0].is_bust && !players.players[0].has_won && !players.dealer.is_bust && !players.dealer.has_won {
                    game_logic::hit(&mut players.players[0], shoe);
                    players.players[0].has_checked = true;
                    players.players[0].has_doubled = true;

                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => {
                if players.players[0].has_won
                    || players.players[0].is_bust
                    || players.dealer.has_won
                    || players.players[0].has_checked
                    || players.players[0].has_blackjack
                {
                    return QuitOrDeal::DealAgain;
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                if player_manager::check_if_hand_can_be_split(
                    &players.players[0].hands[players.players[0].which_hand_being_played].hand,
                ) {
                    game_logic::split(&mut players.players[0], shoe);
                }
            }
            _ => {}
        }
    }
    return QuitOrDeal::KeepPlaying;
}
