use crate::{player_manager::Player, card_manager::Shoe};

pub fn increase_bet(player: &mut Player) {
    if player.bank_balance > 0 {
    player.bet += 1;
    player.bank_balance -= 1;
    }
}

pub fn decrease_bet(player: &mut Player) {
    if player.bet > 0 {
    player.bet -= 1;
    player.bank_balance += 1;
    }
}

pub fn hit(player: &mut Player, shoe: &mut Shoe) {
    let mut card = shoe.draw_card();
    let index = player.hand.len();

    let mut coords = player.hand[index - 1].coords;
    coords.0 += 20;
    coords.1 -= 20;
    card.coords = coords;
    player.hand.push(card);

    check_for_bust(&player) 
}

fn check_for_bust(player: &Player) {

}

