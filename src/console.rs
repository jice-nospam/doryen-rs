

pub type Color = (u8,u8,u8,u8);

pub struct Console {
    width: u32,
    height: u32,
    // power of 2 size (for textures)
    pot_width: u32,
    pot_height: u32,
    ascii: Vec<u32>,
    back: Vec<Color>,
    front: Vec<Color>,
}

impl Console {
    pub fn new(width: u32, height: u32) -> Self {
        let mut back=Vec::new();
        let mut front=Vec::new();
        let mut ascii=Vec::new();
        let mut pot_width = 1;
        let mut pot_height = 1;
        while pot_width < width {
            pot_width *= 2;
        }
        while pot_height < height {
            pot_height *= 2;
        }
        for _ in 0..(pot_width*pot_height) as usize {
            back.push((0,0,0,255));
            front.push((255,255,255,255));
            ascii.push(0);
        }
        Self {
            width,
            height,
            ascii,
            back,
            front,
            pot_width,
            pot_height,
        }
    }
    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
    pub fn get_pot_width(&self) -> u32 { self.pot_width }
    pub fn get_pot_height(&self) -> u32 { self.pot_height }
    pub fn borrow_ascii(&self) -> &Vec<u32> {
        &self.ascii
    }
    pub fn borrow_foreground(&self) -> &Vec<Color> {
        &self.front
    }
    pub fn borrow_background(&self) -> &Vec<Color> {
        &self.back
    }
    fn offset(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.pot_width as usize
    }
    fn check_coords(&self, x: i32, y: i32) -> bool {
        (x as u32) < self.width && (y as u32) < self.height
    }
    pub fn ascii(&mut self, x: i32, y: i32, ascii: u16) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            self.ascii[off] = ascii as u32;
        }
    }
    pub fn fore(&mut self, x: i32, y: i32, col: Color) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            self.front[off] = col;
        }
    }
    pub fn back(&mut self, x: i32, y: i32, col: Color) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            self.back[off] = col;
        }
    }
}