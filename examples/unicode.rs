extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

struct MyRoguelike {}

impl Engine for MyRoguelike {
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(Some((32, 16, 0, 255)), Some((255, 240, 224, 255)), None);
        con.area(
            5,
            5,
            30,
            15,
            Some((255, 255, 255, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16),
        );
        con.print(20, 8, "こんにちは!", TextAlign::Center, None, None);
        con.print(20, 10, "真棒!", TextAlign::Center, None, None);
        con.print(20, 12, "классно", TextAlign::Center, None, None);
        con.print(20, 14, "Φοβερός!", TextAlign::Center, None, None);
        con.print(20, 16, "Ça, c'est énorme!", TextAlign::Center, None, None);
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 16,
        screen_height: CONSOLE_HEIGHT * 16,
        window_title: "doryen-rs unicode demo".to_owned(),
        font_path: "unicode_16x16.png".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike {}));
    app.run();
}
