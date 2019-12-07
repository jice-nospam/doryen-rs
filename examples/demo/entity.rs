use doryen_rs::{color_mul, color_scale, Color, DoryenApi};

use crate::level::Level;
use crate::light::{Light, LIGHT_COEF};

pub struct Entity {
    /// ascii character for this entity
    ch: u16,
    /// position on the map (cell coordinate)
    pub pos: (i32, i32),
    pub name: String,
    color: Color,
    light: bool,
}

impl Entity {
    pub fn new_goblin(pos: (i32, i32)) -> Self {
        Self {
            ch: 'g' as u16,
            pos,
            name: "a petrified goblin".to_owned(),
            color: (80, 150, 70, 255),
            light: false,
        }
    }
    pub fn new_light(pos: (i32, i32)) -> Self {
        Self {
            ch: 15,
            pos,
            name: "a flickering torch".to_owned(),
            color: (150, 174, 27, 255),
            light: true,
        }
    }
    pub fn render(&self, api: &mut dyn DoryenApi, level: &Level) {
        let (color, penumbra) = if self.light {
            (self.color, false)
        } else {
            let light = level.light_at(self.pos);
            let penumbra = Light::is_penumbra(light, 100);
            let mut color = color_mul(self.color, light);
            if penumbra {
                color = color_scale(color, LIGHT_COEF);
            }
            (color, penumbra)
        };
        api.con().ascii(
            self.pos.0,
            self.pos.1,
            if penumbra { '?' as u16 } else { self.ch },
        );
        api.con().fore(self.pos.0, self.pos.1, color);
    }
}
