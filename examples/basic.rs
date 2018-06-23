extern crate doryen_rs;

use doryen_rs::App;

fn main() {
    let mut app = App::new(800, 600, "my roguelike", 128, 128);
    app.run("terminal8x8_aa_ro.png");
}
