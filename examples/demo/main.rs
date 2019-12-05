extern crate doryen_rs;

mod level;
mod player;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign, UpdateEvent};

use level::Level;
use player::Player;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const PLAYER_SPEED: f32 = 0.15;
const BLACK: Color = (0, 0, 0, 255);
const WHITE: Color = (255, 255, 255, 255);

struct DoryenDemo {
    player: Player,
    mouse_pos: (f32, f32),
    level: Level,
    loaded: bool,
}

impl Engine for DoryenDemo {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        api.con().register_color("white", WHITE);
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        if !self.loaded && self.level.try_load() {
            self.loaded = true;
            self.player.move_to(self.level.start_pos());
        }
        if self.loaded {
            let input = api.input();
            if (input.key("ArrowLeft") || input.key("KeyA"))
                && !self.level.is_wall(self.player.left())
            {
                self.player.move_left();
            } else if (input.key("ArrowRight") || input.key("KeyD"))
                && !self.level.is_wall(self.player.right())
            {
                self.player.move_right();
            }
            if (input.key("ArrowUp") || input.key("KeyW")) && !self.level.is_wall(self.player.up())
            {
                self.player.move_up();
            } else if (input.key("ArrowDown") || input.key("KeyS"))
                && !self.level.is_wall(self.player.down())
            {
                self.player.move_down();
            }
            self.mouse_pos = input.mouse_pos();
        }
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        if self.loaded {
            self.clear_con(api);
            self.level.render(api);
            let con = api.con();
            let pos = self.player.pos();
            con.ascii(pos.0, pos.1, '@' as u16);
            con.fore(pos.0, pos.1, (255, 255, 255, 255));
            con.print_color(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT - 2) as i32,
                "#[white]Move with #[red]arrows or WSAD\n#[white]Fire with #[red]mouse",
                TextAlign::Center,
                None,
            );
        }
    }
}

impl DoryenDemo {
    pub fn new() -> Self {
        Self {
            player: Player::new(PLAYER_SPEED),
            mouse_pos: (0.0, 0.0),
            level: Level::new("demo/level"),
            loaded: false,
        }
    }
}

impl DoryenDemo {
    fn clear_con(&self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(Some(BLACK), Some(BLACK), Some(' ' as u16));
    }
}

fn main() {
    // here are all the available options.
    // better practise is to use default values (see other examples)
    let mut app = App::new(AppOptions {
        window_title: "doryen demo".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(DoryenDemo::new()));
    app.run();
}
