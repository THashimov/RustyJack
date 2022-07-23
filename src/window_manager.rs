use std::{fs, io::Write};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
    EventPump, Sdl, VideoSubsystem,
};

use crate::{
    card_manager::Shoe,
    game_logic,
    player_manager::{self, Player, Players},
};

const BACKGROUND_PATH: &str = "/home/mighty/projects/RustyJack/src/assets/table_img.png";
const X_INDENT: i32 = 10;

pub struct ValueCoords {
    pub x_coord: i32,
    pub y_coord: i32,
}

impl ValueCoords {
    fn new_val_coords(
        players: &Players,
        window_size: (u32, u32),
        balance_and_bet: &BalanceAndBet,
    ) -> ValueCoords {
        let player = &players.players[0];
        let mut player_hand_val_string = String::from("Hand value: ");
        let player_hand_val =
            game_logic::get_hand_value(&player.hands[player.which_hand_being_played].hand);
        player_hand_val_string.push_str(&player_hand_val.to_string());

        let x_coord = (window_size.0 - player_hand_val_string.len() as u32 * 20) as i32 + 7;
        let y_coord = balance_and_bet.y_coord;

        ValueCoords { x_coord, y_coord }
    }
}
pub struct BalanceAndBet {
    pub text_height: u32,
    pub x_coord: i32,
    pub y_coord: i32,
    pub text_col: Color,
    pub bank_balance_text: String,
    pub bank_balance_coords: Rect,
    pub bet_amount_text: String,
    pub bet_amount_text_coords: Rect,
    pub bank_balance_number: String,
    pub bet_amount_number: String,
}

impl BalanceAndBet {
    fn new_balance_details(window_size: &(u32, u32)) -> BalanceAndBet {
        let text_height = window_size.0 / 35;
        let y_coord = (window_size.1 / 4) as i32;
        let text_col = Color::RGB(0, 0, 0);
        let bank_balance_text = String::from("Balance: ");
        let bet_amount_text = String::from("Bet: ");

        let bank_balance_coords = Rect::new(
            X_INDENT,
            y_coord,
            (bank_balance_text.len() * 10) as u32,
            text_height,
        );

        let bet_amount_text_coords = Rect::new(
            X_INDENT,
            y_coord + text_height as i32,
            (bank_balance_text.len() * 10) as u32,
            text_height,
        );

        BalanceAndBet {
            text_height,
            y_coord,
            x_coord: 10,
            text_col,
            bank_balance_text,
            bank_balance_coords,
            bet_amount_text,
            bet_amount_text_coords,
            bank_balance_number: String::new(),
            bet_amount_number: String::new(),
        }
    }
}

pub struct InstructionText {
    coords: Rect,
    text: String,
}

impl InstructionText {
    fn init_inst_location(y_coord: i32, text_height: i32) -> InstructionText {
        let text = String::new();
        let coords = Rect::new(
            X_INDENT,
            y_coord,
            (text.len() * 10) as u32,
            text_height as u32,
        );

        InstructionText { coords, text }
    }

    fn change_width_of_rect(&mut self, text_height: u32) {
        let rect = Rect::new(
            self.coords.x,
            self.coords.y,
            self.text.len() as u32 * 10,
            text_height,
        );

        self.coords = rect;
    }
}

pub struct WindowManager {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub window_size: (u32, u32),
    pub balance_and_bet: BalanceAndBet,
    pub value_coords: ValueCoords,
    win_or_lose_text_coords: Rect,
    pub show_counter: bool,
    pub show_hint: bool,
}

impl WindowManager {
    pub fn new_window() -> WindowManager {
        let (video, event_pump) = init_sdl_and_event_pump();

        let window = video
            .window("RustyJack", 800, 600)
            .fullscreen_desktop()
            .build()
            .unwrap();

        let window_size = window.size();
        let balance_and_bet = BalanceAndBet::new_balance_details(&window_size);

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let win_or_lose_text_coords = Rect::new(0, 0, 0, 0);

        let value_coords = ValueCoords {
            x_coord: 0,
            y_coord: 0,
        };

        canvas.clear();

        WindowManager {
            canvas,
            event_pump,
            texture_creator,
            window_size,
            balance_and_bet,
            value_coords,
            win_or_lose_text_coords,
            show_counter: false,
            show_hint: false,
        }
    }

    pub fn load_background(&mut self) {
        let background_img = self.texture_creator.load_texture(BACKGROUND_PATH).unwrap();

        self.canvas.copy(&background_img, None, None).unwrap();
    }

    pub fn render_cards(&mut self, players: &Players) {
        self.render_dealer_cards(&players.dealer);

        for i in 0..4 {
            self.render_player_cards(&players.players[i]);
        }
    }

    fn render_player_cards(&mut self, player: &Player) {
        for i in 0..player.hands.len() {
            for j in 0..player.hands[i].hand.len() {
                let coords = player.hands[i].hand[j].coords;
                let card_img = self
                    .texture_creator
                    .load_texture(player.hands[i].hand[j].img_src.clone())
                    .unwrap();
                let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

                self.canvas.copy(&card_img, None, Some(coords)).unwrap();
            }
        }
    }

    fn render_dealer_cards(&mut self, dealer: &Player) {
        for i in 0..dealer.hands[0].hand.len() {
            let coords = dealer.hands[0].hand[i].coords;
            let card_img = self
                .texture_creator
                .load_texture(dealer.hands[0].hand[i].img_src.clone())
                .unwrap();
            let coords = Rect::new(coords.0 as i32, coords.1 as i32, 80, 110);

            self.canvas.copy(&card_img, None, Some(coords)).unwrap();
        }
    }

    pub fn render_text(&mut self, font: &Font, rect: Rect, text: &str) {
        let surface = font.render(&text).blended(self.balance_and_bet.text_col);

        let surface = match surface {
            Ok(surface) => surface,
            Err(error) => panic!("{:?}", error),
        };

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(rect)).unwrap();
    }

    pub fn render_balance_and_bet_text(&mut self, font: &Font) {
        let balance = self.balance_and_bet.bank_balance_text.clone();
        let bet = self.balance_and_bet.bet_amount_text.clone();
        self.render_text(font, self.balance_and_bet.bank_balance_coords, &balance);
        self.render_text(font, self.balance_and_bet.bet_amount_text_coords, &bet);
    }

    pub fn render_updated_bank_ballance(&mut self, player: &Player, font: &Font) {
        let mut rect = self.balance_and_bet.bank_balance_coords;
        rect.x += (self.balance_and_bet.bank_balance_text.len() * 10) as i32;
        self.render_text(font, rect, &player.bank_balance.to_string());
    }

    pub fn render_updated_bet(&mut self, player: &Player, font: &Font) {
        let bet_amount_as_str = player.bet[player.which_hand_being_played].to_string();
        let mut rect = self.balance_and_bet.bet_amount_text_coords;

        rect.x += (self.balance_and_bet.bet_amount_text.len() * 10) as i32 + 40;
        rect.y = self.balance_and_bet.y_coord + self.balance_and_bet.text_height as i32;

        self.render_text(font, rect, &bet_amount_as_str);
    }

    pub fn render_instructions(&mut self, font: &Font) {
        let y_coord =
            self.balance_and_bet.y_coord + ((self.balance_and_bet.text_height * 2) + 20) as i32;

        let mut inst_obj =
            InstructionText::init_inst_location(y_coord, self.balance_and_bet.text_height as i32);

        for i in 0..9 {
            match i {
                0 => inst_obj.text = String::from("Up Arrow - Increase Bet"),
                1 => inst_obj.text = String::from("Down Arrow - Decrease Bet"),
                2 => inst_obj.text = String::from("H - Hit"),
                3 => inst_obj.text = String::from("C - Check"),
                4 => inst_obj.text = String::from("D - Double"),
                5 => inst_obj.text = String::from("S - Split"),
                6 => inst_obj.text = String::from("R - Deal Again"),
                7 => inst_obj.text = String::from("Z - Show Counter"),
                8 => inst_obj.text = String::from("X - Strategy Hint"),
                _ => {}
            }
            inst_obj.change_width_of_rect(self.balance_and_bet.text_height);
            self.render_text(font, inst_obj.coords, &inst_obj.text);
            inst_obj.coords.y += self.balance_and_bet.text_height as i32;
        }
    }

    pub fn render_bust_or_win_text(&mut self, players: &mut Players, font: &Font) {
        let mut text = String::from(" ");
        let player = &mut players.players[0];

        if player.hands.len() < 2 {
            if player.is_bust[0] {
                text = String::from("You went bust!")
            } else if player.has_won[0] && !player.has_blackjack[0] && !players.dealer.has_won[0] {
                text = String::from("You win!")
            } else if player.has_blackjack[0] && player.has_won[0] {
                text = String::from("Blackjack!");
            } else if players.dealer.has_won[0] && !player.has_won[0] {
                text = String::from("Dealer wins!")
            } else if players.dealer.has_won[0] && player.has_won[0] {
                text = String::from("Push")
            }
        }

        self.win_or_lose_text_coords = Rect::new(
            ((self.window_size.0 / 2) - ((text.len() * 10) as u32 / 2)) as i32,
            (self.window_size.1 / 2) as i32,
            (text.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        self.render_text(font, self.win_or_lose_text_coords, &text);
    }

    pub fn render_player_and_dealer_hand_value(&mut self, players: &Players, font: &Font) {
        let val_cords_obj =
            ValueCoords::new_val_coords(players, self.window_size, &self.balance_and_bet);

        self.value_coords.x_coord = val_cords_obj.x_coord;
        self.value_coords.y_coord = val_cords_obj.y_coord;

        let player = &players.players[0];
        let which_hand = player.which_hand_being_played;

        let player_hand_val = game_logic::get_hand_value(&player.hands[which_hand].hand);
        let mut player_hand_val_string = String::from("Hand value: ");
        player_hand_val_string.push_str(&player_hand_val.to_string());

        let mut dealer_hand_val_string = String::from("Dealer hand value: ");
        let dealer_hand_val = game_logic::get_hand_value(&players.dealer.hands[0].hand);
        dealer_hand_val_string.push_str(&dealer_hand_val.to_string());

        let mut rect = Rect::new(
            val_cords_obj.x_coord,
            val_cords_obj.y_coord,
            (player_hand_val_string.len() * 15) as u32,
            self.balance_and_bet.text_height,
        );

        self.render_text(font, rect, &player_hand_val_string);
        rect.y += self.balance_and_bet.text_height as i32 + 10;
        self.render_text(font, rect, &dealer_hand_val_string);
    }

    fn render_count(&mut self, shoe: &Shoe, font: &Font) {
        let mut count_str = String::from("Count: ");
        count_str.push_str(&shoe.count.to_string());

        let rect = Rect::new(
            self.value_coords.x_coord,
            self.value_coords.y_coord + (self.balance_and_bet.text_height * 2) as i32 + 10,
            (count_str.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        self.render_text(font, rect, &count_str);
    }

    pub fn render_hint(&mut self, font: &Font, players: &mut Players) {
        let hint = player_manager::return_hint(&mut players.players[0], &mut players.dealer);
        let mut hint_str = String::new();

        if let Some(some_string) = hint {
            hint_str = some_string
        };

        let rect = Rect::new(
            self.value_coords.x_coord,
            self.value_coords.y_coord + (self.balance_and_bet.text_height * 3) as i32 + 10,
            (hint_str.len() * 10) as u32,
            self.balance_and_bet.text_height,
        );

        self.render_text(font, rect, &hint_str);
    }
    pub fn refresh_screen(&mut self, players: &mut Players, shoe: &Shoe, font: &Font) {
        self.canvas.clear();
        self.load_background();
        self.render_cards(players);
        self.render_balance_and_bet_text(font);
        self.render_updated_bank_ballance(&players.players[0], &font);
        self.render_updated_bet(&players.players[0], &font);
        self.render_instructions(font);
        self.render_bust_or_win_text(players, &font);
        self.render_player_and_dealer_hand_value(&players, font);
        if self.show_counter {
            self.render_count(shoe, font);
        }
        if self.show_hint {
            self.render_hint(font, players)
        }
        self.canvas.present();
    }
}

fn init_sdl_and_event_pump() -> (VideoSubsystem, EventPump) {
    let path = String::from("./src/log.txt");
    let attempts = 0;

    let sdl_context = create_sdl_context(attempts, &path);
    let video_subsystem = build_subsys(&sdl_context, attempts, &path);
    let event_pump = create_event_pump(&sdl_context, attempts, &path);

    return (video_subsystem, event_pump);
}

fn create_sdl_context(mut attempts: u8, path: &str) -> Sdl {
    if fs::metadata(path).is_ok() {
        fs::remove_file(path).unwrap();
    };

    let mut log = fs::File::create(path).unwrap();

    log = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    attempts += 1;
    if attempts <= 10 {
        let sdl_context = match sdl2::init() {
            Ok(sdl) => {
                writeln!(log, "Success! Sdl initialized").unwrap();
                sdl
            }
            Err(e) => {
                writeln!(log, "Failed to create sdl due to error: {:?}", e).unwrap();
                create_sdl_context(attempts, path)
            }
        };
        return sdl_context;
    } else {
        panic!("Failed to create sdl context. Check log.txt for details")
    }
}

fn build_subsys(sdl_context: &Sdl, mut attempts: u8, path: &str) -> VideoSubsystem {
    attempts += 1;
    let mut log = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if attempts <= 10 {
        let video_subsystem = match sdl_context.video() {
            Ok(video) => {
                writeln!(log, "Success! Video subsystem initialized").unwrap();
                video
            }
            Err(e) => {
                writeln!(log, "Failed to initialize video subsystem due to: {}", e).unwrap();
                build_subsys(sdl_context, attempts, path)
            }
        };
        return video_subsystem;
    } else {
        panic!("Failed to create video subsystem. Check log.txt for more information")
    }
}

fn create_event_pump(sdl_context: &Sdl, mut attempts: u8, path: &str) -> EventPump {
    let mut log = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    attempts += 1;

    if attempts <= 10 {
        let event_pump = match sdl_context.event_pump() {
            Ok(event) => {
                writeln!(log, "Success! Event pump created").unwrap();
                event
            }
            Err(e) => {
                writeln!(log, "Failed to create event pump due to: {}", e).unwrap();
                create_event_pump(sdl_context, attempts, path)
            }
        };
        return event_pump;
    } else {
        panic!("Failed to create event pump. Check log.txt for more information")
    }
}
