use crate::card_manager::{Card, Shoe};
use crate::window_manager::WindowManager;

#[derive(Debug)]
pub enum WhoseTurn {
    Dealer,
    PlayerOne,
    PlayerTwo,
    PlayerThree,
    PlayerFour,
}

#[derive(Debug)]
pub struct Players {
    pub dealer: Player,
    pub player_one: Player,
    pub player_two: Player,
    pub player_three: Player,
    pub player_four: Player,
    pub turn: WhoseTurn,
}
#[derive(Debug)]
pub struct Player {
    pub bet: u32,
    pub bank_balance: u32,
    pub hand: Vec<Card>,
    pub coords: (u32, u32),
    window_size: (u32, u32),
}

impl Players {
    pub fn init_players_and_dealer(shoe: &mut Shoe, window_size: &(u32, u32)) -> Players {
        Players {
            dealer: 
            Player::init_player(shoe.draw_card(), &window_size, WhoseTurn::Dealer),
            player_one: 
            Player::init_player(shoe.draw_card(), &window_size, WhoseTurn::PlayerOne),
            player_two: 
            Player::init_player(shoe.draw_card(), &window_size, WhoseTurn::PlayerTwo),
            player_three: 
            Player::init_player(shoe.draw_card(), &window_size,WhoseTurn::PlayerThree),
            player_four: 
            Player::init_player(shoe.draw_card(), &window_size, WhoseTurn::PlayerFour),
            turn: WhoseTurn::Dealer,
        }
    }

    pub fn draw_second_card_for_every_player(&mut self, shoe: &mut Shoe) {
        self.player_one.hand.push(shoe.draw_card());
        self.player_two.hand.push(shoe.draw_card());
        self.player_three.hand.push(shoe.draw_card());
        self.player_four.hand.push(shoe.draw_card());
    }

    pub fn set_player_coords(&mut self) {
        let space_between_players = self.player_one.window_size.0 / 5;
        let start_point = self.player_one.window_size.0 - (space_between_players * 4);
        self.player_one.coords.0 = start_point;
        self.player_two.coords.0 = self.player_one.coords.0 + space_between_players;
        self.player_three.coords.0 = self.player_two.coords.0 + space_between_players;
        self.player_four.coords.0 = self.player_three.coords.0 + space_between_players;
    }

    pub fn render_initial_hand(&mut self, window: &mut WindowManager) {
        for i in 0..2 {
            let src = self.player_one.hand[i].img_src.clone();
            let coords = self.player_one.coords;
            self.player_one.coords.0 += 20;
            self.player_one.coords.1 -= 20;
            window.render_card(&src, coords);
            window.refresh_screen();
            println!("{:?}", self.player_one.coords);
        }

        for i in 0..2 {
            let src = self.player_two.hand[i].img_src.clone();
            let coords = self.player_two.coords;
            self.player_two.coords.0 += 20;
            self.player_two.coords.1 -= 20;
            window.render_card(&src, coords);
            window.refresh_screen();
            println!("{:?}", self.player_two.coords);

        }

        for i in 0..2 {
            let src = self.player_three.hand[i].img_src.clone();
            let coords = self.player_three.coords;
            self.player_three.coords.0 += 20;
            self.player_three.coords.1 -= 20;
            window.render_card(&src, coords);
            window.refresh_screen();
            println!("{:?}", self.player_three.coords);

        }

        for i in 0..2 {
            let src = self.player_four.hand[i].img_src.clone();
            let coords = self.player_four.coords;
            self.player_four.coords.0 += 20;
            self.player_four.coords.1 -= 20;
            window.render_card(&src, coords);
            window.refresh_screen();
            println!("{:?}", self.player_four.coords);

        }
    }
}

impl Player {
    fn init_player(card: Card, window_size: &(u32, u32), turn: WhoseTurn) -> Player {
        let hand = vec![card];
        let x_coord = (window_size.0 / 2) - 40;
        let mut y_coord = window_size.1 / 4;

        match turn {
            WhoseTurn::Dealer => {}
            WhoseTurn::PlayerOne => {
                y_coord += y_coord * 2
            }
            WhoseTurn::PlayerTwo => {
                y_coord += y_coord * 2
            }
            WhoseTurn::PlayerThree => {
                y_coord += y_coord * 2
            }
            WhoseTurn::PlayerFour => {
                y_coord += y_coord * 2
            }
        };

        Player {
            bet: 20,
            bank_balance: 200,
            hand,
            coords: (x_coord, y_coord),
            window_size: *window_size,
        }
    }
}
