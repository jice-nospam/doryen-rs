extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, Image, UpdateEvent};

struct MyRoguelike {
    skull: Image,
    angle: f32,
    scale_time: f32,
}

impl Engine for MyRoguelike {
    fn update(&mut self, _api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        self.angle += 0.01;
        self.scale_time += 0.01;
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        let scale = self.scale_time.cos();
        con.clear(None, Some((0, 0, 0, 255)), None);
        self.skull.blit_ex(
            con,
            (con.get_width() / 2) as f32,
            (con.get_height() / 2) as f32,
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
        window_title: "doryen-rs image demo".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
