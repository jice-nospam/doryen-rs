extern crate doryen_rs;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign, UpdateEvent};

const WHITE: Color = (255, 255, 255, 255);

/*
* This example show how you can intercept the user trying to close the game window.
* All you have to do is to add the `intercept_close_request: true` option when creating the application
* and calling the `InputApi.close_requested()` to detect the event.
* This only works on native target right now.
*/

struct MyRoguelike {
    close_requested: bool,
}

impl Engine for MyRoguelike {
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
        intercept_close_request: true,
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
