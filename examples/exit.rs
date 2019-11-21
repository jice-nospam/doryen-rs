extern crate doryen_rs;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign, UpdateEvent};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const WHITE: Color = (255, 255, 255, 255);

struct MyRoguelike {
    close_requested: bool,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();
        if self.close_requested {
            if input.key("KeyY") {
                return Some(UpdateEvent::Exit);
            } else if input.key("KeyN") {
                self.close_requested = false;
            }
        } else if input.key("Escape") || input.close_requested() {
            self.close_requested = true;
        }
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(None, None, Some(' ' as u16));
        if self.close_requested {
            con.print(
                5,
                5,
                "Exit game ? (press Y or N)",
                TextAlign::Left,
                Some(WHITE),
                None,
            );
        } else {
            con.print(
                5,
                5,
                "Press ESC to exit",
                TextAlign::Left,
                Some(WHITE),
                None,
            );
        }
    }
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            close_requested: false,
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "my roguelike".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
        intercept_close_request: true,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
