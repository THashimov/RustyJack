use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl, image::LoadTexture, EventPump};

const BACKGROUND_PATH: &str = "./src/assets/table_img.png";

pub struct WindowManager {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump
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
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        WindowManager {
            sdl_context,
            canvas,
            event_pump
        }
    }

    pub fn load_background(&mut self) {
        // let image_context = sdl2::image::init(InitFlag::PNG).unwrap();
        let texture_creator = self.canvas.texture_creator();
        let background_img = texture_creator.load_texture(BACKGROUND_PATH).unwrap();

        self.canvas.copy(&background_img, None, None).unwrap();   
    }

    pub fn render_card(&mut self) {

    }

    pub fn refresh_screen(&mut self) {
        self.canvas.present();
    }
}
