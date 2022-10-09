extern crate doryen_rs;
extern crate uni_app;

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

/*
This example shows how you can lower the number of frames per second to limit CPU consumption
using the max_fps field in the AppOptions parameter.
Note that this has no effect on the tickrate at which update() is called which still is 60 times per second.
You can lower the tickrate by calling your world updating code only once every n calls.
*/

struct MyRoguelike;

impl Engine for MyRoguelike {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        api.con().register_color("red", (255, 92, 92, 255));
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let fps = api.fps();
        let con = api.con();
        con.print_color(
            (con.get_width() / 2) as i32,
            (con.get_height() / 2) as i32,
            &format!("Frames since last second : #[red]{}", fps),
            TextAlign::Center,
            None,
        );
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "lowfps test".to_owned(),
        vsync: false,
        max_fps: 10,
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike));
    app.run();
}
