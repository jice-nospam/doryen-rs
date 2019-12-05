use doryen_rs::{Color, DoryenApi, Image};

const START_COLOR: Color = (255, 0, 0, 255);
const WALL_COLOR: Color = (255, 255, 255, 255);

pub struct Level {
    img: Option<Image>,
    ground: Image,
    size: (i32, i32),
    start: (i32, i32),
    walls: Vec<bool>,
}

impl Level {
    pub fn new(img_path: &str) -> Self {
        Self {
            img: Some(Image::new(&(img_path.to_owned() + ".png"))),
            ground: Image::new(&(img_path.to_owned() + "_color.png")),
            size: (0, 0),
            start: (0, 0),
            walls: Vec::new(),
        }
    }
    pub fn try_load(&mut self) -> bool {
        if let Some(ref mut img) = self.img {
            if img.try_load() {
                let size = img.try_get_size().unwrap();
                for y in 0..size.1 {
                    for x in 0..size.0 {
                        let p = img.pixel(x, y).unwrap();
                        self.walls.push(p == WALL_COLOR);
                        if p == START_COLOR {
                            self.start = (x as i32, y as i32);
                        }
                    }
                }
                self.size = (size.0 as i32, size.1 as i32);
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
    fn offset(&self, (x, y): (i32, i32)) -> usize {
        (x + y * self.size.0 as i32) as usize
    }
    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        self.walls[self.offset(pos)]
    }
    pub fn render(&mut self, api: &mut dyn DoryenApi) {
        let mut con = api.con();
        self.ground.blit_2x(&mut con, 0, 0, 0, 0, None, None, None);
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if self.walls[self.offset((x, y))] {
                    con.back(x as i32, y as i32, (100, 100, 100, 255));
                }
            }
        }
    }
}
