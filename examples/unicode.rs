extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

struct MyRoguelike {}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut DoryenApi) {}
    fn update(&mut self, _api: &mut DoryenApi) {}
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        con.clear(Some((32, 16, 0, 255)), Some((255, 240, 224, 255)), None);
        con.ascii(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            '@' as u16,
        );
        con.print(5, 20, "驚くばかり！", TextAlign::Left, None, None);
        con.print(15, 20, "真棒！", TextAlign::Right, None, None);
        con.print(15, 10, "классно", TextAlign::Center, None, None);
        con.print(25, 5, "Φοβερός!", TextAlign::Center, None, None);
        con.print(8, 12, "ça, c'est énorme!", TextAlign::Left, None, None);
    }
    fn resize(&mut self, _api: &mut DoryenApi) {}
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 16,
        screen_height: CONSOLE_HEIGHT * 16,
        window_title: "doryen-rs unicode demo".to_owned(),
        font_path: "unicode_16x16.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(MyRoguelike {}));
    app.run();
}
