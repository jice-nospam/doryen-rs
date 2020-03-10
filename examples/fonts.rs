extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent};

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

const FONTS: [&str; 19] = [
    "terminal_8x8.png",
    "terminal_colored_8x8.png",
    "terminal_8x12.png",
    "terminal_10x16.png",
    "terminal_12x12.png",
    "SmoothWalls_9x9.png",
    "Aesomatica_16x16.png",
    "Bisasam_20x20.png",
    "Buddy--graphical_10x10.png",
    "Cheepicus_8x8.png",
    "Cheepicus_15x15.png",
    "Cheepicus_16x16.png",
    "Herrbdog_12x12.png",
    "Kein_5x5.png",
    "Mkv_curses_6x6.png",
    "Runeset_24x24.png",
    "Teeto_K_18x18.png",
    "Terbert_7x7.png",
    "Yayo_tunur_13x13.png",
];

struct MyRoguelike {
    cur_font: usize,
}

impl Engine for MyRoguelike {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();
        if input.key_released("PageDown") || input.mouse_button_released(0) {
            self.cur_font = (self.cur_font + 1) % FONTS.len();
            api.set_font_index(self.cur_font);
        } else if input.key_released("PageUp") {
            self.cur_font = (self.cur_font + FONTS.len() - 1) % FONTS.len();
            api.set_font_index(self.cur_font);
        }
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Some((128, 128, 128, 255)),
            None,
            Some('.' as u16),
        );
        con.area(
            10,
            10,
            5,
            5,
            Some((255, 64, 64, 255)),
            Some((128, 32, 32, 255)),
            Some('&' as u16),
        );
        con.ascii(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            '@' as u16,
        );
        con.fore(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            (255, 255, 255, 255),
        );
        con.rectangle(
            (CONSOLE_WIDTH / 2 - 20) as i32,
            (CONSOLE_HEIGHT / 2 - 2) as i32,
            40,
            7,
            Some((255, 255, 255, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16),
        );
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2) as i32,
            FONTS[self.cur_font],
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            None,
        );
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2) as i32 + 2,
            "PageUp/PageDown to change font",
            TextAlign::Center,
            Some((255, 192, 128, 255)),
            None,
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self { cur_font: 0 }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 24,
        screen_height: CONSOLE_HEIGHT * 24,
        window_title: "doryen-rs font test".to_owned(),
        font_paths: FONTS.iter().map(|s| s.to_string()).collect(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
