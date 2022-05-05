use sdl2::{pixels::Color, render::{Canvas, TextureCreator}, video::{Window, WindowContext}, Sdl, image::LoadTexture, EventPump, rect::Rect};

use crate::player_manager::Player;

const BACKGROUND_PATH: &str = "./src/assets/table_img.png";

pub struct WindowManager {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>
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


        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
    
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        WindowManager {
            sdl_context,
            canvas,
            event_pump,
            texture_creator
        }
    }

    pub fn load_background(&mut self) {
        let background_img = self.texture_creator.load_texture(BACKGROUND_PATH).unwrap();

        self.canvas.copy(&background_img, None, None).unwrap();   
    }

    pub fn render_card(&mut self, src: &String) {
        let card_img = self.texture_creator.load_texture(src).unwrap();
        let location = Rect::new(0, 0, 80, 110);

        self.canvas.copy(&card_img, None, Some(location)).unwrap();

    }

    pub fn refresh_screen(&mut self) {
        self.canvas.present();
    }
}
