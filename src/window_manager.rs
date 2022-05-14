use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
    EventPump, Sdl,
};

use crate::player_manager::{Player, Players};

const BACKGROUND_PATH: &str = "./src/assets/table_img.png";

pub struct BalanceAndBet {
    pub text_height: u32,
    pub y_coord: i32,
    pub text_col: Color,
    pub bank_balance_text: String,
    pub bet_amount_text: String,
    pub bank_balance_number: String,
    pub bet_amount_number: String,
}

impl BalanceAndBet {
    fn new_balance_details(window_size: &(u32, u32)) -> BalanceAndBet {
        let text_height = window_size.0 / 25;
        let y_coord = (window_size.1 / 4) as i32;
        let text_col = Color::RGB(150, 150, 100);
        BalanceAndBet {
            text_height,
            y_coord,
            text_col,
            bank_balance_text: String::from("Balance: "),
            bet_amount_text: String::from("Bet: "),
            bank_balance_number: String::new(),
            bet_amount_number: String::new(),
        }
    }
}

pub struct WindowManager {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub window_size: (u32, u32),
    pub balance_and_bet: BalanceAndBet,
}

impl WindowManager {
    pub fn new_window() -> WindowManager {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsys
            .window("RustyJack", 800, 600)
            .fullscreen_desktop()
            .build()
            .unwrap();

        let window_size = window.size();
        let balance_and_bet = BalanceAndBet::new_balance_details(&window_size);

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        canvas.clear();

        WindowManager {
            sdl_context,
            canvas,
            event_pump,
            texture_creator,
            window_size,
            balance_and_bet,
        }
    }

    pub fn load_background(&mut self) {
        let background_img = self.texture_creator.load_texture(BACKGROUND_PATH).unwrap();

        self.canvas.copy(&background_img, None, None).unwrap();
    }

    pub fn render_cards(&mut self, players: &Players) {
        self.render_dealer_cards(&players.dealer);
        self.render_player_cards(&players.player_one);
        self.render_player_cards(&players.player_two);
        self.render_player_cards(&players.player_three);
        self.render_player_cards(&players.player_four);
    }

    fn render_player_cards(&mut self, player: &Player) {
        for i in 0..player.hand.len() {
            let coords = player.hand[i].coords;
            let card_img = self
                .texture_creator
                .load_texture(player.hand[i].img_src.clone())
                .unwrap();
            let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

            self.canvas.copy(&card_img, None, Some(coords)).unwrap();
        }
    }

    fn render_dealer_cards(&mut self, dealer: &Player) {
        for i in 0..dealer.hand.len() {
            let coords = dealer.hand[i].coords;
            let card_img = self
                .texture_creator
                .load_texture(dealer.hand[i].img_src.clone())
                .unwrap();
            let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

            self.canvas.copy(&card_img, None, Some(coords)).unwrap();
        }
    }

    pub fn render_balance_and_bet_text(&mut self, font: &Font) {
        let bank_balance_coords = Rect::new(
            10,
            self.balance_and_bet.y_coord,
            (self.balance_and_bet.bank_balance_text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let surface = font
            .render(&self.balance_and_bet.bank_balance_text)
            .blended(self.balance_and_bet.text_col)
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas
            .copy(&texture, None, Some(bank_balance_coords))
            .unwrap();

        let bet_coords = Rect::new(
            10,
            self.balance_and_bet.y_coord + self.balance_and_bet.text_height as i32,
            (self.balance_and_bet.bet_amount_text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let surface = font
            .render(&self.balance_and_bet.bet_amount_text)
            .blended(self.balance_and_bet.text_col)
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(bet_coords)).unwrap();
    }

    pub fn render_updated_bank_ballance(&mut self, player: &Player, font: &Font) {
        let bank_balance = player.bank_balance.to_string();

        let coords = Rect::new(
            10 + (self.balance_and_bet.bank_balance_text.len() * 10) as i32,
            self.balance_and_bet.y_coord,
            (self.balance_and_bet.bank_balance_text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let surface = font
            .render(&bank_balance)
            .blended(self.balance_and_bet.text_col)
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(coords)).unwrap();
    }

    pub fn render_updated_bet(&mut self, player: &Player, font: &Font) {
        let bet_amount = player.bet.to_string();

        let coords = Rect::new(
            10 + (self.balance_and_bet.bet_amount_text.len() * 10) as i32,
            self.balance_and_bet.y_coord + self.balance_and_bet.text_height as i32,
            (self.balance_and_bet.bet_amount_text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let surface = font
            .render(&bet_amount)
            .blended(self.balance_and_bet.text_col)
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(coords)).unwrap();
    }

    pub fn render_instructions(&mut self, font: &Font) {
        let y_coord = (self.window_size.1 - (self.window_size.1 / 2)) as i32;
        let mut text = String::from("Up Arrow - Increase Bet");
        let mut coords = Rect::new(
            10,
            y_coord,
            (text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let mut surface = font.render(&text).blended(Color::RGB(0, 0, 0)).unwrap();
        let mut texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        for i in 0..8 {
            self.canvas.copy(&texture, None, Some(coords)).unwrap();
            surface = font
                .render(&text)
                .blended(self.balance_and_bet.text_col)
                .unwrap();
            texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            coords = Rect::new(
                10,
                y_coord + (self.balance_and_bet.text_height * i) as i32,
                (text.len() * 10) as u32,
                self.balance_and_bet.text_height,
            );

            match i {
                0 => text = String::from("Down Arrow - Decrease Bet"),
                1 => text = String::from("H - Hit"),
                2 => text = String::from("C - Check"),
                3 => text = String::from("D - Double"),
                4 => text = String::from("S - Split"),
                5 => text = String::from("R - Deal Again"),
                _ => {}
            }
        }
    }

    pub fn render_bust_or_win_text(&mut self, player: &Player, font: &Font) {
        let mut win_or_lose_message = String::from(" ");

        if player.is_bust {
            win_or_lose_message = String::from("You went bust!")
        } else if !player.is_bust && player.has_won {
            win_or_lose_message = String::from("You win!")
        }

        let text_coords = Rect::new(
            ((self.window_size.0 / 2) - ((win_or_lose_message.len() * 10) as u32 / 2)) as i32,
            (self.window_size.1 / 2) as i32,
            (win_or_lose_message.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        let surface = font
            .render(&win_or_lose_message)
            .blended(self.balance_and_bet.text_col)
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(text_coords)).unwrap();
    }

    pub fn refresh_screen(&mut self, players: &Players, font: &Font) {
        self.canvas.clear();
        self.load_background();
        self.render_cards(players);
        self.render_balance_and_bet_text(font);
        self.render_updated_bank_ballance(&players.player_one, &font);
        self.render_updated_bet(&players.player_one, &font);
        self.render_instructions(font);
        self.render_bust_or_win_text(&players.player_one, &font);
        self.canvas.present();
    }
}
