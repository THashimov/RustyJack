use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl,
};

use crate::player_manager::Players;

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
    }

    pub fn render_card(&mut self, src: &str, coords: (u32, u32)) {
        let card_img = self.texture_creator.load_texture(src).unwrap();
        let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

        self.canvas.copy(&card_img, None, Some(coords)).unwrap();
    }

    pub fn render_initial_cards(&mut self, players: &mut Players) {
        let src = players.dealer.hand[0].img_src.clone();
        let coords = players.dealer.coords;

        players.set_player_coords();
        players.render_initial_hand(self);
    
        self.render_card(&src, coords);
    }

    pub fn refresh_screen(&mut self) {
        self.canvas.present();
    }
}
