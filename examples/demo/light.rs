use crate::BLACK;
use doryen_fov::{FovAlgorithm, MapData};
use doryen_rs::{color_add, color_blend, Color, Image};

pub struct Light {
    pos: (f32, f32),
    radius: f32,
    color: Color,
}

impl Light {
    pub fn new((x, y): (i32, i32), radius: f32, color: Color) -> Self {
        Self {
            pos: (x as f32, y as f32),
            radius,
            color,
        }
    }
    pub fn pos_mut(&mut self) -> &mut (f32, f32) {
        &mut self.pos
    }
    pub fn render(
        &self,
        level_map: &mut MapData,
        fov: &mut dyn FovAlgorithm,
        lightmap: &mut Image,
    ) {
        let minx = ((self.pos.0 - self.radius).floor() as i32).max(0) as u32;
        let maxx =
            ((self.pos.0 + self.radius).ceil() as i32).min(lightmap.width() as i32 - 1) as u32;
        let miny = ((self.pos.1 - self.radius).floor() as i32).max(0) as u32;
        let maxy =
            ((self.pos.1 + self.radius).ceil() as i32).min(lightmap.height() as i32 - 1) as u32;
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut map = MapData::new(width as usize, height as usize);
        for y in miny..=maxy {
            for x in minx..=maxx {
                map.set_transparent(
                    (x - minx) as usize,
                    (y - miny) as usize,
                    level_map.is_transparent(x as usize, y as usize),
                );
            }
        }
        fov.compute_fov(
            &mut map,
            self.radius as usize,
            self.radius as usize,
            self.radius as usize,
            true,
        );
        for y in miny..=maxy {
            for x in minx..=maxx {
                if map.is_in_fov((x - minx) as usize, (y - miny) as usize) {
                    let dx = x as f32 - self.pos.0;
                    let dy = y as f32 - self.pos.1;
                    let distance = dx * dx + dy * dy;
                    let coef = distance / (self.radius * self.radius);
                    if coef < 1.0 {
                        let light = color_blend(self.color, BLACK, coef);
                        let cur_light = lightmap.pixel(x, y).unwrap();
                        lightmap.put_pixel(x, y, color_add(light, cur_light));
                    }
                }
            }
        }
    }
}
