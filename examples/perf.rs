extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct PerfTest {
    seed: u64,
}

impl Engine for PerfTest {
    fn init(&mut self, _api: &mut DoryenApi) {}
    fn update(&mut self, _api: &mut DoryenApi) {}
    fn render(&mut self, api: &mut DoryenApi) {
        let fps = api.fps();
        let con = api.con();
        for y in 0..CONSOLE_HEIGHT as i32 {
            for x in 0..CONSOLE_WIDTH as i32 {
                let val = self.rnd();
                con.back(
                    x,
                    y,
                    (
                        (val & 0xFF) as u8,
                        ((val >> 8) & 0x5F) as u8,
                        ((val >> 16) & 0x3F) as u8,
                        255,
                    ),
                );
                con.fore(
                    x,
                    y,
                    (
                        ((val >> 16) & 0xFF) as u8,
                        ((val >> 24) & 0xFF) as u8,
                        ((val >> 32) & 0xFF) as u8,
                        255,
                    ),
                );
                con.ascii(x, y, ((val >> 40) & 0xFF) as u16);
            }
        }
        con.rectangle(
            (CONSOLE_WIDTH / 2 - 10) as i32,
            (CONSOLE_HEIGHT / 2 - 2) as i32,
            20,
            5,
            Some((255, 255, 255, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16),
        );
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2) as i32,
            &format!("{} fps", fps),
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            None,
        );
    }
}

impl PerfTest {
    pub fn new() -> Self {
        Self { seed: 0xdeadbeef }
    }
    fn rnd(&mut self) -> u64 {
        self.seed = 214013u64.wrapping_mul(self.seed).wrapping_add(2531011);
        self.seed
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "doryen-rs performance test".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: false,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(PerfTest::new()));
    app.run();
}
