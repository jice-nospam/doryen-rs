use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};
use doryen_rs::{color_blend, Color, DoryenApi, Image};

const START_COLOR: Color = (255, 0, 0, 255);
const WALL_COLOR: Color = (255, 255, 255, 255);
const VISITED_BLEND_COLOR: Color = (10, 10, 40, 255);
const VISITED_BLEND_COEF: f32 = 0.8;

pub struct Level {
    img: Option<Image>,
    ground: Image,
    size: (i32, i32),
    start: (i32, i32),
    walls: Vec<bool>,
    visited_2x: Vec<bool>,
    fov: FovRestrictive,
    map: MapData,
}

impl Level {
    pub fn new(img_path: &str) -> Self {
        Self {
            img: Some(Image::new(&(img_path.to_owned() + ".png"))),
            ground: Image::new(&(img_path.to_owned() + "_color.png")),
            size: (0, 0),
            start: (0, 0),
            walls: Vec::new(),
            visited_2x: Vec::new(),
            fov: FovRestrictive::new(),
            map: MapData::new(1, 1),
        }
    }
    pub fn try_load(&mut self) -> bool {
        if let Some(ref mut img) = self.img {
            if img.try_load() {
                self.compute_walls_2x_and_start_pos();
                self.compute_walls();
                self.img = None;
            } else {
                return false;
            }
        }
        true
    }
    pub fn start_pos(&self) -> (i32, i32) {
        self.start
    }
    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        self.walls[self.offset(pos)]
    }
    pub fn render(&mut self, api: &mut dyn DoryenApi) {
        if self.ground.try_load() {
            let mut con = api.con();
            let mut img = Image::new_empty(self.size.0 as u32 * 2, self.size.1 as u32 * 2);

            for y in 0..self.size.1 as usize * 2 {
                for x in 0..self.size.0 as usize * 2 {
                    let off = self.offset_2x((x as i32, y as i32));
                    if self.map.is_in_fov(x, y) {
                        img.put_pixel(
                            x as u32,
                            y as u32,
                            self.ground.pixel(x as u32, y as u32).unwrap(),
                        );
                        self.visited_2x[off] = true;
                    } else if self.visited_2x[off] {
                        let col = self.ground.pixel(x as u32, y as u32).unwrap();
                        let dark_col = color_blend(col, VISITED_BLEND_COLOR, VISITED_BLEND_COEF);
                        img.put_pixel(x as u32, y as u32, dark_col);
                    } else {
                        img.put_pixel(x as u32, y as u32, (0, 0, 0, 255));
                    }
                }
            }
            img.blit_2x(&mut con, 0, 0, 0, 0, None, None, None);
        }
    }
    pub fn compute_fov(&mut self, (x, y): (i32, i32), radius: usize) {
        self.map.clear_fov();
        self.fov
            .compute_fov(&mut self.map, x as usize * 2, y as usize * 2, radius, true);
    }
    fn compute_walls_2x_and_start_pos(&mut self) {
        if let Some(ref mut img) = self.img {
            let size = img.try_get_size().unwrap();
            self.map = MapData::new(size.0 as usize, size.1 as usize);
            for y in 0..size.1 {
                for x in 0..size.0 {
                    let p = img.pixel(x, y).unwrap();
                    self.map
                        .set_transparent(x as usize, y as usize, p != WALL_COLOR);
                    self.visited_2x.push(false);
                    if p == START_COLOR {
                        self.start = (x as i32 / 2, y as i32 / 2);
                    }
                }
            }
            self.size = (size.0 as i32 / 2, size.1 as i32 / 2);
        }
    }
    fn compute_walls(&mut self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let mut count = 0;
                let x2 = x as usize * 2;
                let y2 = y as usize * 2;
                if self.map.is_transparent(x2, y2) {
                    count += 1;
                }
                if self.map.is_transparent(x2 + 1, y2) {
                    count += 1;
                }
                if self.map.is_transparent(x2, y2 + 1) {
                    count += 1;
                }
                if self.map.is_transparent(x2 + 1, y2 + 1) {
                    count += 1;
                }
                self.walls.push(count < 2);
            }
        }
    }
    fn offset(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.size.0 as i32) as usize
    }
    fn offset_2x(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.size.0 as i32 * 2) as usize
    }
}
