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
    let player = &mut players.players[0];
    let dealer = &mut players.dealer;

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
            } => game_logic::increase_bet(player),
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => game_logic::decrease_bet(player),
            Event::KeyUp {
                keycode: Some(Keycode::H),
                ..
            } => {
                if !player.is_bust
                    && !player.has_won
                    && !dealer.is_bust
                    && !dealer.has_won
                {
                    player.can_change_bet = false;
                    game_logic::hit(player, shoe);
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => {
                if !player.is_bust
                    && !player.has_won
                    && !dealer.is_bust
                    && !dealer.has_won
                {
                    game_logic::stand(dealer, shoe);
                    player.has_checked = true;
                };
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                if !player.is_bust
                    && !player.has_won
                    && !dealer.is_bust
                    && !dealer.has_won
                {
                    game_logic::hit(player, shoe);
                    player.has_checked = true;
                    player.has_doubled = true;
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => {
                if player.has_won
                    || player.is_bust
                    || dealer.has_won
                    || player.has_checked
                    || player.has_blackjack
                {
                    return QuitOrDeal::DealAgain;
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                if player_manager::check_if_hand_can_be_split(&player.hands[player.which_hand_being_played].hand) 
                && player.hands.len() < 4 {
                    game_logic::split(player, shoe);
                }
            }
            _ => {}
        }
    }
    return QuitOrDeal::KeepPlaying;
}
