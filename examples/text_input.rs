extern crate doryen_rs;

use unicode_segmentation::UnicodeSegmentation;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign, UpdateEvent};

const WHITE: Color = (255, 255, 255, 255);

struct MyRoguelike {
    txt: String,
}

impl Engine for MyRoguelike {
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();
        // input.text returns the characters typed by the player since last update
        let txt = input.text();
        if !txt.is_empty() {
            self.txt.push_str(&txt);
        }
        // handle backspace
        if input.key_released("Backspace") && !self.txt.is_empty() {
            // convoluted way to remove the last character of the string
            // in a way that also works with utf-8 graphemes
            // where one character != one byte
            let mut graphemes = self.txt.graphemes(true).rev();
            graphemes.next();
            self.txt = graphemes.rev().collect();
        }
        // handle tab
        if input.key_released("Tab") {
            self.txt.push_str("   ");
        }
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(None, None, Some(' ' as u16));
        con.print(
            5,
            5,
            &format!("Type some text : {}", self.txt),
            TextAlign::Left,
            Some(WHITE),
            None,
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self { txt: String::new() }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "text input".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
