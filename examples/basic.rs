extern crate doryen_rs;

use doryen_rs::App;

fn main() {
    let mut app = App::new(80, 45, "my roguelike", 128, 128);
    {
        let rccon = app.console();
        let mut con = rccon.borrow_mut();
        con.rectangle(0,0,80,45,(128,128,128,255),(0,0,0,255),Some('.' as u16));
        con.area(10,10,5,5,(255,64,64,255),(128,32,32,255),Some('&' as u16));
        con.ascii(40,20,'@' as u16);
        con.fore(40,20,(255,255,255,255));
    }
    app.run("terminal8x8_aa_ro.png");
}
