use crate::player_manager::Player;

pub fn increase_bet(player: &mut Player) {
    player.bet += 1;
    player.bank_balance -= player.bet
}

pub fn decrease_bet(player: &mut Player) {
    player.bet -= 1;
    player.bank_balance += player.bet
}