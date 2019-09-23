use std::collections::HashMap;

use crate::color::{color_blend, Color};

// rectangle drawing kit
pub const CHAR_CORNER_NW: u16 = 218;
pub const CHAR_CORNER_SW: u16 = 192;
pub const CHAR_CORNER_SE: u16 = 217;
pub const CHAR_CORNER_NE: u16 = 191;
pub const CHAR_LINE_H: u16 = 196;
pub const CHAR_LINE_V: u16 = 179;
// sub-pixel resolution kit
pub const CHAR_SUBP_NW: u16 = 226;
pub const CHAR_SUBP_NE: u16 = 227;
pub const CHAR_SUBP_N: u16 = 228;
pub const CHAR_SUBP_SE: u16 = 229;
pub const CHAR_SUBP_DIAG: u16 = 230;
pub const CHAR_SUBP_E: u16 = 231;
pub const CHAR_SUBP_SW: u16 = 232;

#[derive(Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

/// This contains the data for a console (including the one displayed on the screen) and methods to draw on it.
pub struct Console {
    width: u32,
    height: u32,
    // power of 2 size (for textures)
    pot_width: u32,
    pot_height: u32,
    ascii: Vec<u32>,
    back: Vec<Color>,
    fore: Vec<Color>,
    colors: HashMap<String, Color>,
    color_stack: Vec<Color>,
}

impl Console {
    /// create a new offscreen console that you can blit on another console
    /// width and height are in cells (characters), not pixels.
    pub fn new(width: u32, height: u32) -> Self {
        let mut back = Vec::new();
        let mut fore = Vec::new();
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
            fore.push((255, 255, 255, 255));
            ascii.push(' ' as u32);
        }
        Self {
            width,
            height,
            ascii,
            back,
            fore,
            pot_width,
            pot_height,
            colors: HashMap::new(),
            color_stack: Vec::new(),
        }
    }
    /// resizes the console
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let mut pot_width = 1;
        let mut pot_height = 1;
        while pot_width < width {
            pot_width *= 2;
        }
        while pot_height < height {
            pot_height *= 2;
        }
        self.pot_height = pot_height;
        self.pot_width = pot_width;
        self.back.clear();
        self.fore.clear();
        self.ascii.clear();
        for _ in 0..(pot_width * pot_height) as usize {
            self.back.push((0, 0, 0, 255));
            self.fore.push((255, 255, 255, 255));
            self.ascii.push(' ' as u32);
        }
    }
    /// associate a name with a color for this console.
    /// The color name can then be used in [`Console::print_color`]
    /// Example
    /// ```
    /// use doryen_rs::{Console, TextAlign};
    /// let mut con=Console::new(80,25);
    /// con.register_color("pink", (255, 0, 255, 255));
    /// con.print_color(5, 5, "This text contains a #[pink]pink#[] word", TextAlign::Left, None);
    /// ```
    pub fn register_color(&mut self, name: &str, value: Color) {
        self.colors.insert(name.to_owned(), value);
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
    /// for fast reading of the characters values
    pub fn borrow_ascii(&self) -> &Vec<u32> {
        &self.ascii
    }
    /// for fast reading of the characters colors
    pub fn borrow_foreground(&self) -> &Vec<Color> {
        &self.fore
    }
    /// for fast reading of the background colors
    pub fn borrow_background(&self) -> &Vec<Color> {
        &self.back
    }
    /// for fast writing of the characters values
    pub fn borrow_mut_ascii(&mut self) -> &mut Vec<u32> {
        &mut self.ascii
    }
    /// for fast writing of the characters colors
    pub fn borrow_mut_foreground(&mut self) -> &mut Vec<Color> {
        &mut self.fore
    }
    /// for fast writing of the background colors
    pub fn borrow_mut_background(&mut self) -> &mut Vec<Color> {
        &mut self.back
    }
    /// get the background color of a cell (if x,y inside the console)
    pub fn get_back(&self, x: i32, y: i32) -> Option<Color> {
        if self.check_coords(x, y) {
            return Some(self.unsafe_get_back(x, y));
        }
        None
    }
    /// get the foreground color of a cell (if x,y inside the console)
    pub fn get_fore(&self, x: i32, y: i32) -> Option<Color> {
        if self.check_coords(x, y) {
            return Some(self.unsafe_get_fore(x, y));
        }
        None
    }
    /// get the ascii code of a cell (if x,y inside the console)
    pub fn get_ascii(&self, x: i32, y: i32) -> Option<u16> {
        if self.check_coords(x, y) {
            return Some(self.unsafe_get_ascii(x, y));
        }
        None
    }
    /// get the background color of a cell (no boundary check)
    pub fn unsafe_get_back(&self, x: i32, y: i32) -> Color {
        let off = self.offset(x, y);
        self.back[off]
    }
    /// get the foreground color of a cell (no boundary check)
    pub fn unsafe_get_fore(&self, x: i32, y: i32) -> Color {
        let off = self.offset(x, y);
        self.fore[off]
    }
    /// get the ascii code of a cell (no boundary check)
    pub fn unsafe_get_ascii(&self, x: i32, y: i32) -> u16 {
        let off = self.offset(x, y);
        self.ascii[off] as u16
    }
    fn offset(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.pot_width as usize
    }
    fn check_coords(&self, x: i32, y: i32) -> bool {
        (x as u32) < self.width && (y as u32) < self.height
    }
    /// set the character at a specific position (doesn't change the color)
    pub fn ascii(&mut self, x: i32, y: i32, ascii: u16) {
        if self.check_coords(x, y) {
            self.unsafe_ascii(x, y, ascii);
        }
    }
    /// set the character color at a specific position
    pub fn fore(&mut self, x: i32, y: i32, col: Color) {
        if self.check_coords(x, y) {
            self.unsafe_fore(x, y, col);
        }
    }
    /// set the background color at a specific position
    pub fn back(&mut self, x: i32, y: i32, col: Color) {
        if self.check_coords(x, y) {
            self.unsafe_back(x, y, col);
        }
    }
    /// set the character at a specific position (no boundary check)
    pub fn unsafe_ascii(&mut self, x: i32, y: i32, ascii: u16) {
        let off = self.offset(x, y);
        self.ascii[off] = u32::from(ascii);
    }
    /// set the character color at a specific position (no boundary check)
    pub fn unsafe_fore(&mut self, x: i32, y: i32, col: Color) {
        let off = self.offset(x, y);
        self.fore[off] = col;
    }
    /// set the background color at a specific position (no boundary check)
    pub fn unsafe_back(&mut self, x: i32, y: i32, col: Color) {
        let off = self.offset(x, y);
        self.back[off] = col;
    }
    /// fill the whole console with values
    pub fn clear(&mut self, fore: Option<Color>, back: Option<Color>, fillchar: Option<u16>) {
        let w = self.width;
        let h = self.height;
        self.area(0, 0, w, h, fore, back, fillchar);
    }
    /// write a multi-color string. Foreground color is defined by #[color_name] patterns inside the string.
    /// color_name must have been registered with [`Console::register_color`] before.
    /// Default foreground color is white, at the start of the string.
    /// When an unknown color name is used, the color goes back to its previous value.
    /// You can then use an empty name to end a color span.
    /// Example
    /// ```
    /// use doryen_rs::{Console, TextAlign};
    /// let mut con=Console::new(80,25);
    /// con.register_color("pink", (255, 0, 255, 255));
    /// con.register_color("blue", (0, 0, 255, 255));
    /// con.print_color(5, 5, "#[blue]This blue text contains a #[pink]pink#[] word", TextAlign::Left, None);
    /// ```
    pub fn print_color(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        align: TextAlign,
        back: Option<Color>,
    ) {
        let mut cury = y;
        self.color_stack.clear();
        for line in text.to_owned().split('\n') {
            self.print_line_color(x, cury, line, align, back);
            cury += 1;
        }
    }

    /// compute the length of a string containing color codes.
    /// Example :
    /// ```
    /// use doryen_rs::Console;
    /// let len = Console::text_color_len("#[red]red text with a #[blue]blue#[] word");
    /// assert_eq!(len, 25); // actual string : "red text with a blue word"
    /// let len = Console::text_color_len("#[red]a\nb");
    /// assert_eq!(len, 3); // actual string : "a\nb"
    /// let len = Console::text_color_len("normal string");
    /// assert_eq!(len, 13);
    /// ```
    pub fn text_color_len(text: &str) -> usize {
        let mut text_len=0;
        for color_span in text.to_owned().split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.split(']');
            let col_name = col_text.next().unwrap();
            if let Some(text_span) = col_text.next() {
                text_len += text_span.chars().count();
            } else {
                text_len += col_name.chars().count();
            }
        }
        text_len
    }

    fn get_color_spans(&mut self, text: &str, text_len: &mut i32) -> Vec<(Color, String)> {
        let mut spans: Vec<(Color, String)> = Vec::new();
        *text_len = 0;
        let mut fore = *self.color_stack.last().unwrap_or(&(255, 255, 255, 255));
        for color_span in text.to_owned().split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.splitn(2,']');
            let col_name = col_text.next().unwrap();
            if let Some(text_span) = col_text.next() {
                if let Some(color) = self.colors.get(col_name) {
                    fore = *color;
                    self.color_stack.push(fore);
                } else {
                    self.color_stack.pop();
                    fore = *self.color_stack.last().unwrap_or(&(255, 255, 255, 255));
                }
                spans.push((fore, text_span.to_owned()));
                *text_len += text_span.chars().count() as i32;
            } else {
                spans.push((fore, col_name.to_owned()));
                *text_len += col_name.chars().count() as i32;
            }
        }
        spans
    }

    fn print_line_color(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        align: TextAlign,
        back: Option<Color>,
    ) {
        let mut str_len = 0;
        let spans = self.get_color_spans(text, &mut str_len);
        let mut ix = match align {
            TextAlign::Left => x,
            TextAlign::Right => (x - str_len + 1),
            TextAlign::Center => (x - str_len / 2),
        };
        for (color, span) in spans {
            self.print_line(ix, y, &span, TextAlign::Left, Some(color), back);
            ix += span.chars().count() as i32;
        }
    }
    /// write a string. If the string reaches the border of the console, it's truncated.
    /// If the string contains carriage return `"\n"`, multiple lines are printed.
    pub fn print(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        align: TextAlign,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        let mut cury = y;
        for line in text.to_owned().split('\n') {
            self.print_line(x, cury, line, align, fore, back);
            cury += 1;
        }
    }
    fn print_line(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        align: TextAlign,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        let stext = text.to_owned();
        let mut str_len = stext.chars().count() as i32;
        let mut start = 0;
        let mut ix = match align {
            TextAlign::Left => x,
            TextAlign::Right => (x - str_len + 1),
            TextAlign::Center => (x - str_len / 2),
        };
        if ix < 0 {
            str_len += ix;
            start -= ix;
            ix = 0;
        }
        if ix + str_len > self.width as i32 {
            str_len = self.width as i32 - ix;
        }
        let mut chars = stext.chars().skip(start as usize);
        for _ in 0..str_len {
            let ch = chars.next();
            self.cell(ix, y, Some(ch.unwrap() as u16), fore, back);
            ix += 1;
        }
    }
    /// draw a rectangle, possibly filling it with a character.
    pub fn rectangle(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fore: Option<Color>,
        back: Option<Color>,
        fill: Option<u16>,
    ) {
        let right = x + (w as i32) - 1;
        let down = y + (h as i32) - 1;
        self.cell(x, y, Some(CHAR_CORNER_NW), fore, back);
        self.cell(right, down, Some(CHAR_CORNER_SE), fore, back);
        self.cell(right, y, Some(CHAR_CORNER_NE), fore, back);
        self.cell(x, down, Some(CHAR_CORNER_SW), fore, back);
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
    /// fill an area with values
    pub fn area(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fore: Option<Color>,
        back: Option<Color>,
        fillchar: Option<u16>,
    ) {
        let right = x + (w as i32);
        let down = y + (h as i32);
        if let Some(fillchar) = fillchar {
            for iy in y.max(0)..down.min(self.height as i32) {
                let off = iy * self.pot_width as i32;
                for ix in x.max(0)..right.min(self.width as i32) {
                    self.ascii[(off + ix) as usize] = u32::from(fillchar);
                }
            }
        }
        if let Some(fore) = fore {
            for iy in y.max(0)..down.min(self.height as i32) {
                let off = iy * self.pot_width as i32;
                for ix in x.max(0)..right.min(self.width as i32) {
                    self.fore[(off + ix) as usize] = fore;
                }
            }
        }
        if let Some(back) = back {
            for iy in y.max(0)..down.min(self.height as i32) {
                let off = iy * self.pot_width as i32;
                for ix in x.max(0)..right.min(self.width as i32) {
                    self.back[(off + ix) as usize] = back;
                }
            }
        }
    }
    /// can change all properties of a console cell at once
    pub fn cell(
        &mut self,
        x: i32,
        y: i32,
        ascii: Option<u16>,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            if let Some(ascii) = ascii {
                self.ascii[off] = u32::from(ascii);
            }
            if let Some(fore) = fore {
                self.fore[off] = fore;
            }
            if let Some(back) = back {
                self.back[off] = back;
            }
        }
    }
    /// blit (draw) a console onto another one
    /// You can use fore_alpha and back_alpha to blend this console with existing background on the destination.
    /// If you define a key color, the cells using this color as background will be ignored. This makes it possible to blit
    /// non rectangular zones.
    pub fn blit(
        &self,
        x: i32,
        y: i32,
        destination: &mut Console,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<Color>,
    ) {
        self.blit_ex(
            0,
            0,
            self.width as i32,
            self.height as i32,
            destination,
            x,
            y,
            fore_alpha,
            back_alpha,
            key_color,
        );
    }
    /// blit a region of this console onto another one.
    /// see [`Console::blit`]
    pub fn blit_ex(
        &self,
        xsrc: i32,
        ysrc: i32,
        wsrc: i32,
        hsrc: i32,
        destination: &mut Console,
        xdst: i32,
        ydst: i32,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<Color>,
    ) {
        for y in 0..hsrc - ysrc {
            let off = (y + ysrc) * self.pot_width as i32;
            let doff = (y + ydst) * destination.pot_width as i32;
            for x in 0..wsrc - xsrc {
                if self.check_coords(xsrc + x, ysrc + y)
                    && destination.check_coords(xdst + x, ydst + y)
                {
                    let src_idx = (off + x + xsrc) as usize;
                    let dest_idx = (doff + x + xdst) as usize;
                    let src_back = self.back[src_idx];
                    let dst_back = destination.back[dest_idx];
                    if back_alpha > 0.0 {
                        let back = self.back[src_idx];
                        if let Some(key) = key_color {
                            if key == back {
                                continue;
                            }
                        }
                        destination.back[dest_idx] = color_blend(dst_back, src_back, back_alpha);
                    }
                    if fore_alpha > 0.0 {
                        let src_fore = self.fore[src_idx];
                        let dst_fore = destination.fore[dest_idx];
                        let src_char = self.ascii[src_idx];
                        let dst_char = destination.ascii[dest_idx];
                        let dst_back = destination.back[dest_idx];
                        if fore_alpha < 1.0 {
                            if src_char == ' ' as u32 {
                                destination.fore[dest_idx] =
                                    color_blend(dst_fore, src_back, back_alpha);
                            } else if dst_char == ' ' as u32 {
                                destination.ascii[dest_idx] = src_char;
                                destination.fore[dest_idx] =
                                    color_blend(dst_back, src_fore, fore_alpha);
                            } else if dst_char == src_char {
                                destination.fore[dest_idx] =
                                    color_blend(dst_fore, src_fore, fore_alpha);
                            } else if fore_alpha < 0.5 {
                                destination.fore[dest_idx] =
                                    color_blend(dst_fore, dst_back, fore_alpha * 2.0);
                            } else {
                                destination.ascii[dest_idx] = src_char;
                                destination.fore[dest_idx] =
                                    color_blend(dst_back, src_fore, (fore_alpha - 0.5) * 2.0);
                            }
                        } else {
                            destination.fore[dest_idx] = src_fore;
                            destination.ascii[dest_idx] = src_char;
                        }
                    }
                }
            }
        }
    }
}
