extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

struct MyRoguelike {}

impl Engine for MyRoguelike {
    fn update(&mut self, _api: &mut DoryenApi) {}
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        con.ascii(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            '@' as u16,
        );
        con.print(
            5,
            20,
            "utf-8サポートを楽しむ",
            TextAlign::Left,
            Some((255, 0, 0, 255)),
            None,
        );
        con.print(
            CONSOLE_WIDTH as i32 - 5,
            20,
            "我会说中文!",
            TextAlign::Right,
            Some((255, 0, 0, 255)),
            None,
        );
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 24,
        screen_height: CONSOLE_HEIGHT * 24,
        window_title: "doryen-rs font test".to_owned(),
        font_path: "unicode_font2_16x16.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: false,
    });
    app.set_engine(Box::new(MyRoguelike {}));
    app.run();
}
