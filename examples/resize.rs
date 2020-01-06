extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent};

struct MyRoguelike {
    width: u32,
    height: u32,
    mouse_pos: (f32, f32),
}

impl Engine for MyRoguelike {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();
        self.mouse_pos = input.mouse_pos();
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            self.width,
            self.height,
            Some((128, 128, 128, 255)),
            Some((0, 0, 0, 255)),
            Some(' ' as u16),
        );
        con.area(
            10,
            10,
            5,
            5,
            Some((255, 64, 64, 255)),
            Some((128, 32, 32, 255)),
            Some('&' as u16),
        );
        con.print(
            (self.width / 2) as i32,
            (self.height / 2) as i32,
            &format!("{} x {}", self.width, self.height),
            TextAlign::Center,
            None,
            None,
        );
        con.back(
            self.mouse_pos.0 as i32,
            self.mouse_pos.1 as i32,
            (255, 255, 255, 255),
        );
    }
    fn resize(&mut self, api: &mut dyn DoryenApi) {
        self.width = api.get_screen_size().0 / 8;
        self.height = api.get_screen_size().1 / 8;
        api.con().resize(self.width, self.height);
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            width: 80,
            height: 50,
            mouse_pos: (0.0, 0.0),
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "resizable console".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
