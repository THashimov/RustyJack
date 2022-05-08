use std::time::Duration;

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl, ttf::Font,
};

use crate::player_manager::{Players, Player};

const BACKGROUND_PATH: &str = "./src/assets/table_img.png";

pub struct WindowManager {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub window_size: (u32, u32),
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

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        WindowManager {
            sdl_context,
            canvas,
            event_pump,
            texture_creator,
            window_size,
        }
    }

    pub fn load_background(&mut self) {
        let background_img = self.texture_creator.load_texture(BACKGROUND_PATH).unwrap();

        self.canvas.copy(&background_img, None, None).unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }

    pub fn render_card(&mut self, src: &str, coords: (u32, u32)) {
        let card_img = self.texture_creator.load_texture(src).unwrap();
        let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

        self.canvas.copy(&card_img, None, Some(coords)).unwrap();
        std::thread::sleep(Duration::from_millis(30));

    }

    pub fn render_initial_cards(&mut self, players: &mut Players) {
        let src = players.dealer.hand[0].img_src.clone();
        let coords = players.dealer.coords;

        self.render_card(&src, coords);
        std::thread::sleep(Duration::from_millis(30));

        players.set_player_coords();
        players.render_initial_hand(self);
    }

    pub fn refresh_screen(&mut self) {
        self.canvas.present();
    }

    pub fn render_balance_and_bet_text(&mut self, player: &mut Player, font: &Font) {
        let text_height = self.window_size.0 / 25;
        let y_coord = (self.window_size.1 / 4) as i32;
        let mut bank_balance = String::from("Ballance: ");
        bank_balance.push_str(&player.bank_balance.to_string());
        let mut bet = String::from("Bet: ");
        bet.push_str(&player.bet.to_string());


        let mut coords = Rect::new(10, y_coord, (bank_balance.len() * 10) as u32, text_height);

        let mut surface = font.render(&bank_balance).blended(Color::RGB(0, 0, 0)).unwrap();
        let mut texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();

        self.canvas.copy(&texture, None, Some(coords)).unwrap();
        std::thread::sleep(Duration::from_millis(30));

        coords = Rect::new(10, y_coord + text_height as i32, (bet.len() * 10) as u32, text_height);
        surface = font.render(&bet).blended(Color::RGB(0, 0, 0)).unwrap();
        texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();

        self.canvas.copy(&texture, None, Some(coords)).unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }

    pub fn render_instructions(&mut self, font: &Font) {
        let text_height = self.window_size.0 / 25;
        let y_coord = (self.window_size.1 - (self.window_size.1 / 2)) as i32;
        let mut text = String::from("Up Arrow - Increase Bet");
        let mut coords = Rect::new(10, y_coord, (text.len() * 10) as u32, text_height);

        let mut surface = font.render(&text).blended(Color::RGB(0, 0, 0)).unwrap();
        let mut texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();

        for i in 0..7 {
            self.canvas.copy(&texture, None, Some(coords)).unwrap();
            std::thread::sleep(Duration::from_millis(30));
            surface = font.render(&text).blended(Color::RGB(0, 0, 0)).unwrap();
            texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
            coords = Rect::new(10, y_coord + (text_height * i) as i32, (text.len() * 10) as u32, text_height);

            match i {
                0 => {text = String::from("Down Arrow - Decrease Bet")},
                1 => {text = String::from("H - Hit")},
                2 => {text = String::from("C - Check")},
                3 => {text = String::from("D - Double")},
                4 => {text = String::from("S - Split")},
                _ => {}
            }
        }

    }

    pub fn render_updated_bank_ballance(&mut self, player: &Player) {
        
    }
}
