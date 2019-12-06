extern crate doryen_fov;
extern crate doryen_rs;

mod level;
mod light;
mod noise;
mod player;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign, UpdateEvent};

use level::Level;
use player::Player;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const PLAYER_SPEED: f32 = 0.2;
const PLAYER_FOV_RADIUS: usize = 40;
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
            self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
        }
        if self.loaded {
            let mut coef = 1.0 / std::f32::consts::SQRT_2;
            let mut mov = self.player.move_from_input(api);
            if self.level.is_wall(self.player.next_pos((mov.0, 0))) {
                mov.0 = 0;
                coef = 1.0;
            }
            if self.level.is_wall(self.player.next_pos((0, mov.1))) {
                mov.1 = 0;
                coef = 1.0;
            }
            if self.player.move_by(mov, coef) {
                self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
            }
            self.mouse_pos = api.input().mouse_pos();
            self.level.update();
        }
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        if self.loaded {
            self.clear_con(api);
            let player_pos = self.player.pos();
            self.level.render(api, player_pos);
            let player_light = self.level.light_at(player_pos);
            self.player.render(api, player_light);
            let fps = api.fps();
            api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT - 2) as i32,
                &format!("#[white]Move with #[red]arrows or WSAD\n#[white]Fire with #[red]mouse   {:4} fps",fps),
                TextAlign::Center,
                None,
            );
        } else {
            api.con().print_color(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT / 2) as i32,
                &format!("#[white]Loading#[red]..."),
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
    let mut app = App::new(AppOptions {
        window_title: "doryen demo".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(DoryenDemo::new()));
    app.run();
}
