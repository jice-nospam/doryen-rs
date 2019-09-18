extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, Image, TextAlign};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    skull: Image,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    fn update(&mut self, _api: &mut dyn DoryenApi) {}
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(None, Some((0, 0, 0, 255)), None);
        self.skull.blit_2x(con, 23, 0, 0, 0, None, None, None);
        con.print(
            40,
            4,
            "Those pixels\nare twice smaller\nthan a console cell.\nMagic!",
            TextAlign::Center,
            Some((0, 0, 0, 255)),
            None,
        );
    }
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
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
        window_title: "doryen-rs subcell resolution demo".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
