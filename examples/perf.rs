extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

// this part makes it possible to compile to wasm32 target
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

struct PerfTest {
    seed: u64,
}

impl Engine for PerfTest {
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let fps = api.fps();
        let con = api.con();
        let con_width = con.get_width();
        let con_height = con.get_height();
        for y in 0..con_height as i32 {
            for x in 0..con_width as i32 {
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
            (con_width / 2 - 10) as i32,
            (con_height / 2 - 2) as i32,
            20,
            5,
            Some((255, 255, 255, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16),
        );
        con.print(
            (con_width / 2) as i32,
            (con_height / 2) as i32,
            &format!("{} fps", fps),
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            None,
        );
    }
    fn resize(&mut self, api: &mut dyn DoryenApi) {
        let new_width = api.get_screen_size().0 / 8;
        let new_height = api.get_screen_size().1 / 8;
        api.con().resize(new_width, new_height);
    }
}

impl PerfTest {
    pub fn new() -> Self {
        Self { seed: 0xdead_beef }
    }
    fn rnd(&mut self) -> u64 {
        self.seed = 214_013u64.wrapping_mul(self.seed).wrapping_add(2_531_011);
        self.seed
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "doryen-rs performance test".to_owned(),
        vsync: false,
        ..Default::default()
    });
    app.set_engine(Box::new(PerfTest::new()));
    app.run();
}
