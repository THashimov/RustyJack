use crate::{card_manager::{Shoe, Card}, player_manager::{Player, Players}};

pub fn increase_bet(player: &mut Player) {
    if player.bank_balance > 0 && player.can_change_bet {
        player.bet += 1;
        player.bank_balance -= 1;
    }
}

pub fn decrease_bet(player: &mut Player) {
    if player.bet > 0 && player.can_change_bet {
        player.bet -= 1;
        player.bank_balance += 1;
    }
}

pub fn hit(player: &mut Player, shoe: &mut Shoe) {
    if !player.is_bust && get_hand_value(&player.hand) != 21 && !player.has_checked {
        let mut card = shoe.draw_card();
        let index = player.hand.len();

        let mut coords = player.hand[index - 1].coords;
        coords.0 += 20;
        coords.1 -= 20;
        card.coords = coords;
        player.hand.push(card);
    }
}

pub fn check_hand(player: &mut Player) {
    change_aces(player);
    if get_hand_value(&player.hand) > 21 && !player.is_bust {
        player.is_bust = true;
        player.bank_balance -= player.bet
    } else if get_hand_value(&player.hand) == 21 && player.hand.len() > 2 {
        player.has_won = true
    } else if get_hand_value(&player.hand) == 21 && player.hand.len() <= 2 {
        player.has_blackjack = true
    }
}

pub fn change_aces(player: &mut Player) {
    let has_ace = check_for_ace(&player.hand);
    let mut hand_val = get_hand_value(&player.hand);

    if hand_val > 21 && has_ace {
        'change_ace: loop {
        for i in 0..player.hand.len() {
            if player.hand[i].value == 11 {
                player.hand[i].value = 1;
                hand_val = get_hand_value(&player.hand);
                    if hand_val < 21 {
                        break 'change_ace
                    }
            }
        }
    }
    }
}

pub fn check_for_ace(hand: &Vec<Card>) -> bool {
    for i in 0..hand.len() {
        if hand[i].value == 11 {
            return true
        }
    }
    return false
}

pub fn get_hand_value(hand: &Vec<Card>) -> u8 {
    let mut hand_val = 0;

    for i in 0..hand.len() {
        hand_val += hand[i].value;
    }

    hand_val
}

pub fn deal_again(players: &mut Players, shoe: &mut Shoe, window_size: &(u32, u32)){
    players.dealer.hand.drain(..);
    players.player_one.hand.drain(..);
    players.player_two.hand.drain(..);
    players.player_three.hand.drain(..);
    players.player_four.hand.drain(..);

    players.dealer.hand.push(shoe.draw_card());

    players.player_one.hand.push(shoe.draw_card());
    players.player_two.hand.push(shoe.draw_card());
    players.player_three.hand.push(shoe.draw_card());
    players.player_four.hand.push(shoe.draw_card());

    players.player_one.has_won = false;
    players.player_one.is_bust = false;
    players.player_one.can_change_bet = true;
    players.player_one.has_blackjack = false;

    players.deal_cards(shoe, &window_size);
}

pub fn stand(player: &mut Player, shoe: &mut Shoe) {
    while get_hand_value(&player.hand) < 17 {
        let mut card = shoe.draw_card();
        let index = player.hand.len();

        let mut coords = player.hand[index - 1].coords;
        coords.0 -= 20;
        coords.1 += 20;
        card.coords = coords;
        player.hand.push(card);
        change_aces(player);
    }
}
