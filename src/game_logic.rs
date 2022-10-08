use crate::{
    card_manager::{Card, Shoe},
    player_manager::{Hand, Player, Players},
    split_logic::{self, change_hand_being_played},
};

pub fn increase_bet(player: &mut Player) {
    if player.bank_balance > 0 && player.can_change_bet {
        player.bet[0] += 1;
        player.bank_balance -= 1;
    }
}

pub fn decrease_bet(player: &mut Player) {
    if player.bet[0] > 0 && player.can_change_bet {
        player.bet[0] -= 1;
        player.bank_balance += 1;
    }
}

pub fn hit(player: &mut Player, shoe: &mut Shoe) {
    let which_hand = player.which_hand_being_played;

    if !player.is_bust[0]
        && get_hand_value(&player.hands[which_hand].hand) != 21
        && !player.has_checked
    {
        let mut card = shoe.draw_card();
        let index_by_last_card_in_hand = player.hands[which_hand].hand.len();

        let mut coords = player.hands[which_hand].hand[index_by_last_card_in_hand - 1].coords;
        coords.0 += 20;
        coords.1 -= 20;
        card.coords = coords;
        player.hands[which_hand].hand.push(card);
    }
}

pub fn stand(dealer: &mut Player, shoe: &mut Shoe) {
    while get_hand_value(&dealer.hands[0].hand) < 17 {
        let mut card = shoe.draw_card();
        let index = dealer.hands[0].hand.len();

        let mut coords = dealer.hands[0].hand[index - 1].coords;
        coords.0 -= 20;
        coords.1 += 20;
        card.coords = coords;
        dealer.hands[0].hand.push(card);
        change_aces(dealer);
    }

    dealer.has_finished_dealing = true;

    if get_hand_value(&dealer.hands[0].hand) > 21 {
        dealer.is_bust[0] = true;
    }
}

pub fn split(player: &mut Player, shoe: &mut Shoe) {
    let new_hands = split_logic::split_hands(&player.hands[player.which_hand_being_played], shoe);
    player.hands.push(new_hands[0].clone());

    split_logic::change_coords_of_split_cards(player);
    player.which_hand_being_played += 1;
    player.bet[player.which_hand_being_played] = player.bet[0];
    player.bank_balance -= player.bet[player.which_hand_being_played];
    player.has_split = true;
}

pub fn check_for_blackjack_and_bust(player: &mut Player) {
    let which_hand = player.which_hand_being_played;
    change_aces(player);
    let hand_val = get_hand_value(&player.hands[which_hand].hand);
    if hand_val > 21 {
        player_is_bust(player);
    } else if hand_val == 21 {
        if player.hands[which_hand].hand.len() <= 2 && !player.has_split {
            player_has_bj(player);
        } else if player.has_split {
            change_hand_being_played(player);
        } else {
            player.all_hands_played = true;
            player.has_checked = true;
        }
    }
    if player.is_bust[0] || player.has_blackjack[0] {
        player.all_hands_played = true;
        player.has_checked = true;
    }
}

fn player_has_bj(player: &mut Player) {
    player.has_blackjack[player.which_hand_being_played] = true;
    if !player.has_split {
        player.has_checked = true;
        player.has_won[0] = true;
        player.all_hands_played = true;
    }
}

fn player_is_bust(player: &mut Player) {
    player.is_bust[player.which_hand_being_played] = true;
    split_logic::change_hand_being_played(player);
}

pub fn change_aces(player: &mut Player) {
    let which_hand = player.which_hand_being_played;

    let has_ace = check_for_ace(&player.hands[which_hand].hand);
    let mut hand_val = get_hand_value(&player.hands[which_hand].hand);

    if hand_val > 21 && has_ace {
        'change_ace: loop {
            for i in 0..player.hands[which_hand].hand.len() {
                if player.hands[which_hand].hand[i].value == 11 {
                    player.hands[which_hand].hand[i].value = 1;
                    hand_val = get_hand_value(&player.hands[which_hand].hand);
                    if hand_val < 21 {
                        break 'change_ace;
                    }
                }
            }
        }
    }
}

pub fn check_for_ace(hand: &Vec<Card>) -> bool {
    for i in 0..hand.len() {
        if hand[i].value == 11 {
            return true;
        }
    }
    return false;
}

pub fn get_hand_value(hand: &Vec<Card>) -> u8 {
    let mut hand_val = 0;

    for i in 0..hand.len() {
        hand_val += hand[i].value;
    }

    hand_val
}

pub fn deal_again(players: &mut Players, shoe: &mut Shoe, window_size: &(u32, u32)) {
    players.dealer.hands[0].hand.drain(..);
    players.dealer.hands[0].hand.push(shoe.draw_card());
    players.dealer.has_won[0] = false;
    players.dealer.is_bust[0] = false;
    players.dealer.has_finished_dealing = false;

    for i in 0..players.players.len() {
        for _j in 0..players.players[0].hands.len() {
            players.players[i].bet[i] = 0;
            players.players[i].hands.clear();
            players.players[i].hands.push(Hand {
                hand: vec![shoe.draw_card()],
            });
            players.players[i].has_won[0] = false;
            players.players[i].has_checked = false;
            players.players[i].is_bust[i] = false;
            players.players[i].can_change_bet = true;
            players.players[i].has_blackjack[i] = false;
            players.players[i].all_hands_played = false;
            players.players[i].has_doubled[i] = false;
        }
    }
    players.players[0].bet[0] = 20;
    players.deal_cards(shoe, &window_size);
}

// This only runs if player hasn't split. Split winner check is handled in the split module
pub fn check_for_winner(players: &mut Players) {
    let player_hand_val = get_hand_value(&players.players[0].hands[0].hand);
    let dealer_hand_val = get_hand_value(&players.dealer.hands[0].hand);
    let player = &mut players.players[0];
    let dealer = &mut players.dealer;

    if !dealer.is_bust[0] && !player.is_bust[0] && !player.has_blackjack[0] {
        if player_hand_val > dealer_hand_val {
            player.has_won[0] = true;
        } else if dealer_hand_val > player_hand_val {
            dealer.has_won[0] = true;
        } else if player_hand_val == dealer_hand_val {
            player.has_won[0] = true;
            dealer.has_won[0] = true;
        }
    } else if player.is_bust[0] && !dealer.is_bust[0] {
        dealer.has_won[0] = true;
    } else if !player.is_bust[0] && dealer.is_bust[0] {
        player.has_won[0] = true;
    }

    update_player_winnings(players);
}

// This only runs if player hasn't split. Split winner check is handled in the split module
pub fn update_player_winnings(players: &mut Players) {
    let player = &mut players.players[0];
    let dealer = &mut players.dealer;

    if !player.has_blackjack[0] {
        if player.has_won[0] && !dealer.has_won[0] {
            player.bank_balance += player.bet[0]
        } else if !player.has_won[0] && dealer.has_won[0] {
            player.bank_balance = player.bank_balance.overflowing_sub(player.bet[0]).0;
        // if balance reaches 0, end the game
        // Todo
        } else if player.has_won[0] && dealer.has_won[0] {
        }
    } else if player.has_blackjack[0] && get_hand_value(&dealer.hands[0].hand) != 21 {
        player.bank_balance += player.bet[0] * 2;
    } else {
    }
}
