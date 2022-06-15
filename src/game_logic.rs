use crate::{
    card_manager::{Card, Shoe},
    player_manager::{self, Player, Players},
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
    if !player.is_bust
        && get_hand_value(&player.hands[which_hand].hand) != 21
        && !player.has_checked
    {
        let mut card = shoe.draw_card();
        let index = player.hands[which_hand].hand.len();

        let mut coords = player.hands[which_hand].hand[index - 1].coords;
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
        dealer.is_bust = true;
    }
}

pub fn split(player: &mut Player, shoe: &mut Shoe) {}

pub fn check_for_blackjack_and_bust(player: &mut Player) {
    let which_hand = player.which_hand_being_played;
    change_aces(player);

    if get_hand_value(&player.hands[which_hand].hand) > 21 && !player.is_bust {
        player.is_bust = true;
        player.bank_balance -= player.bet[0];
    } else if get_hand_value(&player.hands[which_hand].hand) == 21
        && player.hands[which_hand].hand.len() <= 2
    {
        player.has_blackjack = true;
        player.has_checked = true;
        player.has_won = true;
        player.all_hands_played = true;
    }
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
    players.dealer.has_won = false;
    players.dealer.is_bust = false;
    players.dealer.has_finished_dealing = false;

    for i in 0..players.players.len() {
        players.players[i].bet[i] = 0;
        players.players[i].hands[0].hand.drain(..);
        players.players[i].hands[0].hand.push(shoe.draw_card());
        players.players[i].has_won = false;
        players.players[i].has_checked = false;
        players.players[i].is_bust = false;
        players.players[i].can_change_bet = true;
        players.players[i].has_blackjack = false;
        players.players[i].all_hands_played = false;
        players.players[i].has_doubled = false;
    }
    players.players[0].bet[0] = 20;

    players.deal_cards(shoe, &window_size);
}

pub fn check_for_winner(players: &mut Players) {
    let player_hand_val = get_hand_value(&players.players[0].hands[0].hand);
    let dealer_hand_val = get_hand_value(&players.dealer.hands[0].hand);

    if player_hand_val > dealer_hand_val && !players.players[0].is_bust || players.dealer.is_bust {
        players.players[0].has_won = true;
    } else if dealer_hand_val > player_hand_val && !players.dealer.is_bust {
        players.dealer.has_won = true;
    } else if player_hand_val == dealer_hand_val && !players.players[0].is_bust
        || !players.dealer.is_bust
    {
        players.players[0].has_won = true;
        players.dealer.has_won = true;
    }
    update_player_winnings(players);
}

pub fn update_player_winnings(players: &mut Players) {
    if players.players[0].has_won && players.players[0].has_blackjack {
        players.players[0].bank_balance += players.players[0].bet[0] * 2;
    } else if players.players[0].has_doubled
        && !players.players[0].has_won
        && players.dealer.has_won
    {
        players.players[0].bank_balance -= players.players[0].bet[0] * 2;
    } else if players.players[0].has_doubled
        && players.players[0].has_won
        && !players.dealer.has_won
    {
        players.players[0].bank_balance += players.players[0].bet[0] * 2;
    } else if players.dealer.has_won {
        players.players[0].bank_balance -= players.players[0].bet[0];
    } else if players.players[0].has_won && players.dealer.has_won {
    } else if players.players[0].has_won {
        players.players[0].bank_balance += players.players[0].bet[0];
    }
}
