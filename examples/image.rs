extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, Image};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    skull: Image,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut DoryenApi) {}
    fn update(&mut self, _api: &mut DoryenApi) {}
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        self.skull.blit(
            con,
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2) as i32,
            1.0,
            1.0,
            0.0,
            None,
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            skull: Image::new("skull.png"),
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "doryen-rs image demo".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: false,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
