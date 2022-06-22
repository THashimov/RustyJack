use crate::{
    card_manager::Shoe,
    game_logic,
    player_manager::{Hand, Player, Players},
};

pub fn split_hands(hands: &Hand, shoe: &mut Shoe) -> Vec<Hand> {
    let mut new_hands = vec![];
    new_hands.push(hands.clone());

    if let Some(card) = new_hands[0].hand.pop() {
        new_hands.push(Hand { hand: vec![card] })
    }
    for i in 0..2 {
        new_hands[i].hand.push(shoe.draw_card());
    }
    new_hands
}

pub fn change_coords_of_split_cards(player: &mut Player) {
    let hands = &mut player.hands;
    let point = player.split_coords_point;

    let coords = vec![
        (point.0 - 100, point.1),
        (point.0 + 150, point.1),
        (point.0 - 100, point.1 - 200),
        (point.0 + 150, point.1 - 200),
    ];

    for i in 0..hands.len() {
        hands[i].hand[0].coords = coords[i];
        hands[i].hand[1].coords = coords[i];
        hands[i].hand[1].coords.0 += 20;
        hands[i].hand[1].coords.1 -= 20;
    }
}

pub fn change_hand_being_played(player: &mut Player) {
    let overflow = player.which_hand_being_played.overflowing_sub(1);
    if player.which_hand_being_played == 0 {
        player.all_hands_played = true;
    } else if overflow.1 {
        player.which_hand_being_played = 0;
    } else {
        player.which_hand_being_played = overflow.0;
    }
}

pub fn check_split_hands_for_win(players: &mut Players) {
    let player = &mut players.players[0];
    let dealer = &mut players.dealer;
    let dealer_hand_val = game_logic::get_hand_value(&dealer.hands[0].hand);
    let mut total_bet = 0;

    for i in 0..player.hands.len() {
        let player_hand_val = game_logic::get_hand_value(&player.hands[i].hand);
        if player_hand_val > dealer_hand_val && !player.is_bust[i] || dealer.is_bust[0] {
            player.bet[i] += player.bet[i]
        } else if dealer_hand_val > player_hand_val && !player.is_bust[i] {
            player.bet[i] -= player.bet[i]
        }
    }

    for i in 0..player.bet.len() {
        total_bet += player.bet[i]
    }

    player.bank_balance += total_bet;
}
