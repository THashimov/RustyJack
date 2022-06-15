use crate::player_manager::{Hand, self};

pub fn split_hands(hands: &mut Vec<Hand>) {
    if player_manager::check_if_hand_can_be_split(&hands[0].hand) {
        if let Some(card) = hands[0].hand.pop() {
            hands.push(Hand { hand: vec![card] })
        }
    }
}
