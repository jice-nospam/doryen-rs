extern crate doryen_rs;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, UpdateEvent};

/*
* This example show how to use root console transparency to display the current frame on top of the previous one.
* Each frame, we only draw a circle of blue dots. We fill the rest of the console with a transparent black color.
* When rendering the console, we still see the previous frame but it slowly fades to black as layers of transparent black are added every new frame.
* Note that currently, there's no way to clear the framebuffer. If you don't want to see the previous frame, all the console cells must be opaque (alpha = 255).
* You can still use transparent colors on offscreen console that you blit on the opaque root console. Simply fill the root console with opaque black (0,0,0,255).
*/

struct MyRoguelike {
    cx: f32,
    cy: f32,
    radius: f32,
    angle: f32,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let con = api.con();
        // update the circle radius and center position
        self.angle += 0.6;
        self.radius = 10.0 + 3.0 * (self.angle / 10.0).sin();
        let cs = (self.angle / 20.0).cos();
        let sn = (self.angle / 15.0).sin();
        self.cx = (con.get_width() / 2) as f32 + cs * 15.0;
        self.cy = (con.get_height() / 2) as f32 + sn * 15.0;
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        // fill the console with transparent black. The more opaque it is, the faster the previous frames will fade to black.
        // replace alpha with a lower value, like 10 or 5 and the effect will last longer.
        con.clear(None, Some((0, 0, 0, 20)), None);
        // here we render current frame (only a circle of blue dots)
        for r in 0..10 {
            let angle = self.angle + r as f32 * std::f32::consts::PI * 2.0 / 10.0;
            let cs = angle.cos();
            let sn = angle.sin();
            let x = self.cx + self.radius * cs;
            let y = self.cy + self.radius * sn;
            con.back(x as i32, y as i32, (0, 0, 255, 255));
        }
    }
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "alpha test".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike {
        cx: 0.0,
        cy: 0.0,
        radius: 10.0,
        angle: 0.0,
    }));
    app.run();
}
