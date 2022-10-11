use crate::{
    card_manager::Shoe,
    game_logic,
    player_manager::{self, Players},
    split_logic,
};
use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub enum KeyStroke {
    DealAgain,
    Quit,
    KeepPlaying,
    Counter,
    Hint,
}

pub fn check_for_key_press(
    event_pump: &mut EventPump,
    players: &mut Players,
    shoe: &mut Shoe,
) -> KeyStroke {
    let player = &mut players.players[0];
    let dealer = &mut players.dealer;
    let which_hand = player.which_hand_being_played;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return KeyStroke::Quit,
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
                if !player.is_bust[which_hand]
                    && !player.has_won[0]
                    && !dealer.is_bust[0]
                    && !dealer.has_won[0]
                {
                    player.can_change_bet = false;
                    game_logic::hit(player, shoe);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => return KeyStroke::Counter,
            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => return KeyStroke::Hint,
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => {
                if !player.is_bust[which_hand]
                    && !player.has_won[0]
                    && !dealer.is_bust[0]
                    && !dealer.has_won[0]
                {
                    if !player.has_split {
                        game_logic::stand(dealer, shoe);
                        player.has_checked = true;
                        player.all_hands_played = true;
                    } else {
                        split_logic::change_hand_being_played(player);
                        if player.all_hands_played {
                            game_logic::stand(dealer, shoe);
                            player.has_checked = true;
                        }
                    }
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                if !player.is_bust[which_hand]
                    && !dealer.is_bust[0]
                    && !dealer.has_won[0]
                    && !player.all_hands_played
                {
                    if player.has_split {
                        split_logic::double_split_bet(player);
                    }
                    game_logic::hit(player, shoe);
                    player.has_doubled[which_hand] = true;
                    if player.all_hands_played || !player.has_split {
                        game_logic::stand(dealer, shoe);
                        player.has_checked = true;
                        player.all_hands_played = true;
                    }
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => {
                if player.has_won[0]
                    || player.is_bust[0]
                    || player.has_checked
                    || player.has_blackjack[0]
                    || player.all_hands_played
                {
                    return KeyStroke::DealAgain;
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                if player_manager::check_if_hand_can_be_split(
                    &player.hands[player.which_hand_being_played].hand,
                ) {
                    game_logic::split(player, shoe);
                    player.has_split = true;
                }
            }
            _ => {}
        }
    }
    return KeyStroke::KeepPlaying;
}
