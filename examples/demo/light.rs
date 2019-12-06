use doryen_fov::{FovAlgorithm, MapData};
use doryen_rs::{color_add, color_blend, color_scale, Color, Image};

use crate::noise::simplex;
use crate::BLACK;

const TIME_SCALE: f32 = 0.1;
const LIGHT_INTENSITY: f32 = 1.5;
const LIGHT_FLICKER_MOVE: f32 = 3.0;
const LIGHT_FLICKER_INTENSITY: f32 = 0.4;
const LIGHT_FLICKER_RADIUS: f32 = 0.3;

pub struct Light {
    pos: (f32, f32),
    radius: f32,
    intensity: f32,
    color: Color,
    t: f32,
}

impl Light {
    pub fn new((x, y): (i32, i32), radius: f32, color: Color) -> Self {
        Self {
            pos: (x as f32, y as f32),
            radius,
            color,
            intensity: LIGHT_INTENSITY,
            t: 0.0,
        }
    }
    pub fn pos_mut(&mut self) -> &mut (f32, f32) {
        &mut self.pos
    }
    pub fn update(&mut self) {
        self.t += TIME_SCALE;
    }
    pub fn render(
        &self,
        level_map: &mut MapData,
        fov: &mut dyn FovAlgorithm,
        lightmap: &mut Image,
        flicker: bool,
    ) {
        let (px, py, intensity, radius) = if flicker {
            // alter light position, radius and intensity over time
            (
                self.pos.0 + (LIGHT_FLICKER_MOVE * (simplex(self.t) - 0.5)),
                self.pos.1 + (LIGHT_FLICKER_MOVE * (simplex(self.t + 2.0) - 0.5)),
                self.intensity + LIGHT_FLICKER_INTENSITY * (simplex(self.t + 4.0) - 0.5),
                self.radius * (1.0 + LIGHT_FLICKER_RADIUS * (simplex(self.t + 6.0) - 0.5)),
            )
        } else {
            (self.pos.0, self.pos.1, self.intensity, self.radius)
        };
        let minx = ((px - radius).floor() as i32).max(0) as u32;
        let maxx = ((px + radius).ceil() as i32).min(lightmap.width() as i32 - 1) as u32;
        let miny = ((py - radius).floor() as i32).max(0) as u32;
        let maxy = ((py + radius).ceil() as i32).min(lightmap.height() as i32 - 1) as u32;
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
            radius as usize,
            radius as usize,
            radius as usize,
            true,
        );
        let light_color = color_scale(self.color, intensity);
        let radius2 = radius * radius;
        for y in miny..=maxy {
            for x in minx..=maxx {
                if map.is_in_fov((x - minx) as usize, (y - miny) as usize) {
                    let dx = x as f32 - px;
                    let dy = y as f32 - py;
                    let distance = dx * dx + dy * dy;
                    let coef = distance / radius2;
                    if coef < 1.0 {
                        let light = color_blend(light_color, BLACK, coef);
                        let cur_light = lightmap.pixel(x, y).unwrap();
                        lightmap.put_pixel(x, y, color_add(light, cur_light));
                    }
                }
            }
        }
    }
}
