extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent, KeyEvent};

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
Apart from the basic real-time walking, this example shows how screenshots can be captured in-game.
Because it uses UpdateEvent, any combination of keys can be specified to activate it.
*/

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    player_pos: (i32, i32),
    mouse_pos: (f32, f32),
    screenshot_idx: usize,
}

impl Engine for MyRoguelike {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        api.con().register_color("white", (255, 255, 255, 255));
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();
        if input.key(KeyEvent::ArrowLeft) {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        } else if input.key(KeyEvent::ArrowRight) {
            self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
        }
        if input.key(KeyEvent::ArrowUp) {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        } else if input.key(KeyEvent::ArrowDown) {
            self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
        }
        self.mouse_pos = input.mouse_pos();

        // capture the screen
        if input.key(KeyEvent::Ctrl) && input.key_pressed(KeyEvent::Key('s')) {
            self.screenshot_idx += 1;
            println!("Screenshot taken");
            return Some(UpdateEvent::Capture(format!(
                "screenshot_{:03}.png",
                self.screenshot_idx
            )));
        }

        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Some((128, 128, 128, 255)),
            Some((0, 0, 0, 255)),
            Some('.' as u16),
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
        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        con.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 1) as i32,
            "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
            TextAlign::Center,
            None,
        );
        con.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 3) as i32,
            &format!(
                "#[white]Mouse coordinates: #[red]{}, {}",
                self.mouse_pos.0, self.mouse_pos.1
            ),
            TextAlign::Center,
            None,
        );
        con.print_color(
            5,
            5,
            "#[blue]This blue text contains a #[red]red#[] word",
            TextAlign::Left,
            None,
        );
        con.back(
            self.mouse_pos.0 as i32,
            self.mouse_pos.1 as i32,
            (255, 255, 255, 255),
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
            mouse_pos: (0.0, 0.0),
            screenshot_idx: 0,
        }
    }
}

fn main() {
    // here are all the available options.
    // better practise is to use default values (see other examples)
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
        intercept_close_request: false,
        max_fps: 0,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
