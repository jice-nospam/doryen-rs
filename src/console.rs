pub type Color = (u8, u8, u8, u8);
pub const CHAR_CORNER_NW: u16 = 218;
pub const CHAR_CORNER_SW: u16 = 192;
pub const CHAR_CORNER_SE: u16 = 217;
pub const CHAR_CORNER_NE: u16 = 191;
pub const CHAR_LINE_H: u16 = 196;
pub const CHAR_LINE_V: u16 = 179;

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
        let mut back = Vec::new();
        let mut front = Vec::new();
        let mut ascii = Vec::new();
        let mut pot_width = 1;
        let mut pot_height = 1;
        while pot_width < width {
            pot_width *= 2;
        }
        while pot_height < height {
            pot_height *= 2;
        }
        for _ in 0..(pot_width * pot_height) as usize {
            back.push((0, 0, 0, 255));
            front.push((255, 255, 255, 255));
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
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_pot_width(&self) -> u32 {
        self.pot_width
    }
    pub fn get_pot_height(&self) -> u32 {
        self.pot_height
    }
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
    pub fn rectangle(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fore: Color,
        back: Color,
        fill: Option<u16>,
    ) {
        let right = x + (w as i32) - 1;
        let down = y + (h as i32) - 1;
        self.cell(x, y, CHAR_CORNER_NW, fore, back);
        self.cell(right, down, CHAR_CORNER_SE, fore, back);
        self.cell(right, y, CHAR_CORNER_NE, fore, back);
        self.cell(x, down, CHAR_CORNER_SW, fore, back);
        if (y as u32) < self.height {
            self.area(x + 1, y, w - 2, 1, fore, back, Some(CHAR_LINE_H));
        }
        if (down as u32) < self.height {
            self.area(x + 1, down, w - 2, 1, fore, back, Some(CHAR_LINE_H));
        }
        if (x as u32) < self.width {
            self.area(x, y + 1, 1, h - 2, fore, back, Some(CHAR_LINE_V));
        }
        if (right as u32) < self.width {
            self.area(right, y + 1, 1, h - 2, fore, back, Some(CHAR_LINE_V));
        }
        if fill.is_some() {
            self.area(x + 1, y + 1, w - 2, h - 2, fore, back, fill);
        }
    }
    pub fn area(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fore: Color,
        back: Color,
        fill: Option<u16>,
    ) {
        let right = x + (w as i32);
        let down = y + (h as i32);
        if let Some(fillchar) = fill {
            for ix in x.max(0)..right.min(self.width as i32) {
                for iy in y.max(0)..down.min(self.height as i32) {
                    self.cell(ix, iy, fillchar, fore, back);
                }
            }
        } else {
            for ix in x.max(0)..right.min(self.width as i32) {
                for iy in y.max(0)..down.min(self.height as i32) {
                    self.fore(ix, iy, fore);
                    self.back(ix, iy, back);
                }
            }
        }
    }
    pub fn clear(&mut self, fore: Color, back: Color, fillchar: Option<u16>) {
        let w = self.width;
        let h = self.height;
        self.area(0, 0, w, h, fore, back, fillchar);
    }
    pub fn cell(&mut self, x: i32, y: i32, ascii: u16, fore: Color, back: Color) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            self.ascii[off] = ascii as u32;
            self.front[off] = fore;
            self.back[off] = back;
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
