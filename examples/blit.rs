extern crate doryen_rs;

use doryen_rs::{App, AppOptions, Console, DoryenApi, Engine, TextAlign};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    c1_pos: (i32, i32),
    c1_spd: (i32, i32),
    c2_pos: (i32, i32),
    c2_spd: (i32, i32),
    c1: Console,
    c2: Console,
    alpha: f32,
    step: usize,
}

fn move_con(pos: &mut (i32, i32), spd: &mut (i32, i32), size: (i32, i32)) {
    pos.0 += spd.0;
    if pos.0 == size.0 - 20 {
        spd.0 = -spd.0;
    } else if pos.0 == 0 {
        spd.0 = -spd.0;
    }
    pos.1 += spd.1;
    if pos.1 == size.1 - 20 {
        spd.1 = -spd.1;
    } else if pos.1 == 0 {
        spd.1 = -spd.1;
    }
}

impl Engine for MyRoguelike {
    fn update(&mut self, _api: &mut DoryenApi) {
        if self.step == 0 {
            move_con(
                &mut self.c1_pos,
                &mut self.c1_spd,
                (CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32),
            );
            move_con(
                &mut self.c2_pos,
                &mut self.c2_spd,
                (CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32),
            );
        }
        self.alpha = (self.alpha + 0.01) % 1.0;
        self.step = (self.step + 1) % 10;
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        con.clear(Some((0, 0, 0, 255)), None, Some(' ' as u16));
        for x in 0..CONSOLE_WIDTH as i32 {
            for y in 0..CONSOLE_HEIGHT as i32 {
                con.back(
                    x,
                    y,
                    if (x + y) & 1 == 1 {
                        (96, 64, 32, 255)
                    } else {
                        (32, 64, 96, 255)
                    },
                );
            }
        }
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2) as i32,
            "You create offscreen consoles",
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            None,
        );
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 + 1) as i32,
            "and blit them on other consoles",
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            None,
        );

        self.c1.blit(
            self.c1_pos.0,
            self.c1_pos.1,
            con,
            self.alpha,
            self.alpha,
            None,
        );
        self.c2.blit(
            self.c2_pos.0,
            self.c2_pos.1,
            con,
            1.0 - self.alpha,
            1.0 - self.alpha,
            Some((0, 0, 0, 255)),
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        let mut c1 = Console::new(20, 20);
        let mut c2 = Console::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                c1.back(x, y, (((x + y * 10) % 255) as u8, 0, 0, 255));
                c2.back(
                    x,
                    y,
                    if (x - 10) * (x - 10) + (y - 10) * (y - 10) < 100 {
                        (255, 192, 32, 255)
                    } else {
                        (0, 0, 0, 255)
                    },
                );
            }
        }
        c1.print(10, 10, "Hello", TextAlign::Center, None, None);
        c2.print(10, 10, "Circle", TextAlign::Center, None, None);
        Self {
            c1_pos: (5, 5),
            c2_pos: (15, 20),
            c1_spd: (1, 1),
            c2_spd: (-1, 1),
            c1,
            c2,
            alpha: 1.0,
            step: 0,
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "blitting demo".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: false,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
