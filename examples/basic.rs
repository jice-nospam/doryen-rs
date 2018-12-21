extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    player_pos: (i32, i32),
}

impl Engine for MyRoguelike {
    fn init(&mut self, api: &mut DoryenApi) {
        api.con().register_color("white", (255, 255, 255, 255));
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));
    }
    fn update(&mut self, api: &mut DoryenApi) {
        let input = api.input();
        if input.key("ArrowLeft") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        } else if input.key("ArrowRight") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
        }
        if input.key("ArrowUp") {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        } else if input.key("ArrowDown") {
            self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
        }
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Some((128, 128, 128, 255)),
            None,
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
            "#[white]Move with #[red]arrows",
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
    }
    fn resize(&mut self, _api: &mut DoryenApi) {}
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
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
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
