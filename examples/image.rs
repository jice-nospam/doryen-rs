extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, Image};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    skull: Image,
    angle: f32,
    scale_time: f32,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut DoryenApi) {}
    fn update(&mut self, _api: &mut DoryenApi) {
        self.angle += 0.01;
        self.scale_time += 0.01;
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        let scale = self.scale_time.cos();
        con.clear(None, Some((0, 0, 0, 255)), None);
        self.skull.blit_ex(
            con,
            (CONSOLE_WIDTH / 2) as f32,
            (CONSOLE_HEIGHT / 2) as f32,
            scale,
            scale,
            self.angle,
            None,
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            skull: Image::new("skull.png"),
            angle: 0.0,
            scale_time: 0.0,
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
