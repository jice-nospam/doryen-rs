extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent};

struct MyRoguelike {
    width: u32,
    height: u32,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    fn update(&mut self, _api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
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
            None,
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
