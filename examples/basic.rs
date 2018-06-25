extern crate doryen_rs;

use doryen_rs::{App, AppOptions, Console, Engine, InputApi};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    player_pos: (i32, i32),
}

impl Engine for MyRoguelike {
    fn update(&mut self, input: &mut InputApi) {
        if input.key("ArrowLeft") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        } else if input.key("ArrowRight") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
        }
        if input.key("ArrowUp") {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        } else if input.key("ArrowDown") {
            self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
        }
    }
    fn render(&self, con: &mut Console) {
        con.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            (128, 128, 128, 255),
            (0, 0, 0, 255),
            Some('.' as u16),
        );
        con.area(
            10,
            10,
            5,
            5,
            (255, 64, 64, 255),
            (128, 32, 32, 255),
            Some('&' as u16),
        );
        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        window_title: "my roguelike".to_owned(),
        font_path: "terminal8x8_aa_ro.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: false,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
