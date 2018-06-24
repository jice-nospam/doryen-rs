extern crate doryen_rs;

use doryen_rs::App;

fn main() {
    let mut app = App::new(80, 45, "my roguelike", 128, 128);
    {
        let rccon = app.console();
        let mut con = rccon.borrow_mut();
        con.ascii(40,20,'@' as u16);
        con.fore(40,20,(255,0,0,255));
        con.back(40,20,(0,0,255,255));
    }
    app.run("terminal8x8_aa_ro.png");
}
