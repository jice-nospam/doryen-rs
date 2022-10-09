extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, Image, TextAlign};

// this part makes it possible to compile to wasm32 target
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

struct MyRoguelike {
    skull: Image,
}

impl Engine for MyRoguelike {
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
        window_title: "doryen-rs subcell resolution demo".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
