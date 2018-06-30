use console::{Color, Console};
use file::FileLoader;
use image;

struct MipMap {
    img: Option<image::RgbaImage>,
    dirty: bool,
    width: u32,
    height: u32,
}

impl MipMap {
    fn allocate(&mut self) {
        self.img = Some(image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(
            self.width,
            self.height,
        ));
    }
}

pub struct Image {
    file_loader: FileLoader,
    img: Option<image::RgbaImage>,
    mipmaps: Vec<MipMap>,
}

impl Image {
    pub fn new(file_path: &str) -> Self {
        let mut file_loader = FileLoader::new();
        file_loader.load_file(file_path);
        Self {
            file_loader,
            mipmaps: Vec::new(),
            img: None,
        }
    }
    pub fn is_loaded(&mut self) -> bool {
        if self.img.is_some() {
            return true;
        }
        if self.file_loader.is_file_ready(0) {
            let buf = self.file_loader.get_file_content(0);
            self.intialize_image(&buf);
            return true;
        }
        return false;
    }
    fn intialize_image(&mut self, buf: &Vec<u8>) {
        let img = image::load_from_memory(&buf).unwrap().to_rgba();
        let mut w = img.width();
        let mut h = img.height();
        w >>= 2;
        h >>= 2;
        while w > 0 && h > 0 {
            self.mipmaps.push(MipMap {
                img: None,
                dirty: false,
                width: w,
                height: h,
            });
            w >>= 2;
            h >>= 2;
        }
        self.img = Some(img);
    }
    pub fn get_size(&self) -> Option<(u32, u32)> {
        if let Some(ref img) = self.img {
            return Some((img.width(), img.height()));
        }
        return None;
    }
    pub fn blit(&mut self, con: &mut Console, x: i32, y: i32, transparent: Option<Color>) {
        if !self.is_loaded() {
            return;
        }
        if let Some(ref img) = self.img {
            let width = img.width() as i32;
            let height = img.height() as i32;
            let minx = x.max(0);
            let miny = y.max(0);
            let maxx = (x + width).min(con.get_width() as i32);
            let maxy = (y + height).min(con.get_height() as i32);
            let mut offx = if x < 0 { -x } else { 0 };
            let mut offy = if y < 0 { -y } else { 0 };
            let con_width = con.get_pot_width();
            let back = con.borrow_mut_background();
            for cx in minx..maxx {
                for cy in miny..maxy {
                    let pixel = img
                        .get_pixel((cx - minx + offx) as u32, (cy - miny + offy) as u32)
                        .data;
                    let color = (pixel[0], pixel[1], pixel[2], pixel[3]);
                    if let Some(ref t) = transparent {
                        if color == *t {
                            continue;
                        }
                    }
                    let offset = (cx as u32 + cy as u32 * con_width) as usize;
                    back[offset] = color;
                }
            }
        }
    }
    pub fn blit_ex(
        &mut self,
        con: &mut Console,
        x: f32,
        y: f32,
        scalex: f32,
        scaley: f32,
        angle: f32,
        transparent: Option<Color>,
    ) {
        if !self.is_loaded() || scalex == 0.0 || scaley == 0.0 {
            return;
        }
        let size = self.get_size().unwrap();
        let rx = x - size.0 as f32 * 0.5;
        let ry = y - size.1 as f32 * 0.5;
        if scalex == 1.0 && scaley == 1.0 && angle == 0.0 && rx.floor() == rx && ry.floor() == ry {
            let ix = rx as i32;
            let iy = ry as i32;
            self.blit(con, ix, iy, transparent);
            return;
        }
        let iw = (size.0 / 2) as f32 * scalex;
        let ih = (size.1 / 2) as f32 * scaley;
        // get the coordinates of the image corners in the console
        let newx_x = angle.cos();
        let newx_y = -angle.sin();
        let newy_x = newx_y;
        let newy_y = -newx_x;
        // image corners coordinates
        // 0 = P - w/2 x' +h/2 y'
        let x0 = x - iw * newx_x + ih * newy_x;
        let y0 = y - iw * newx_y + ih * newy_y;
        // 1 = P + w/2 x' + h/2 y'
        let x1 = x + iw * newx_x + ih * newy_x;
        let y1 = y + iw * newx_y + ih * newy_y;
        // 2 = P + w/2 x' - h/2 y'
        let x2 = x + iw * newx_x - ih * newy_x;
        let y2 = y + iw * newx_y - ih * newy_y;
        // 3 = P - w/2 x' - h/2 y'
        let x3 = x - iw * newx_x - ih * newy_x;
        let y3 = y - iw * newx_y - ih * newy_y;
        // get the affected rectangular area in the console
        let rx = x0.min(x1).min(x2).min(x3) as i32;
        let ry = y0.min(y1).min(y2).min(y3) as i32;
        let rw = x0.max(x1).max(x2).max(x3) as i32 - rx;
        let rh = y0.max(y1).max(y2).max(y3) as i32 - ry;
        // clip it
        let minx = rx.max(0);
        let miny = ry.max(0);
        let maxx = (rx + rw).min(con.get_width() as i32);
        let maxy = (ry + rh).min(con.get_height() as i32);
        let invscalex = 1.0 / scalex;
        let invscaley = 1.0 / scaley;
        let con_width = con.get_pot_width();
        let back = con.borrow_mut_background();
        if let Some(ref img) = self.img {
            for cx in minx..maxx {
                for cy in miny..maxy {
                    // map the console pixel to the image world
                    let ix =
                        (iw + (cx as f32 - x) * newx_x + (cy as f32 - y) * (-newy_x)) * invscalex;
                    let iy =
                        (ih + (cx as f32 - x) * (newx_y) - (cy as f32 - y) * newy_y) * invscaley;
                    if ix as u32 >= size.0 || iy as u32 >= size.1 {
                        continue;
                    }
                    let pixel = img.get_pixel(ix as u32, iy as u32).data;
                    let color = (pixel[0], pixel[1], pixel[2], pixel[3]);
                    if let Some(ref t) = transparent {
                        if color == *t {
                            continue;
                        }
                    }
                    let offset = (cx as u32 + cy as u32 * con_width) as usize;
                    if scalex < 1.0 || scaley < 1.0 {
                        // todo mipmap
                    }
                    back[offset] = color;
                }
            }
        }
    }
    fn generate_mip(&mut self, level: usize) {
        if let Some(ref source) = self.img {
            if self.mipmaps[level].img.is_none() {
                self.mipmaps[level].allocate();
            }
            if let Some(ref mut dest) = self.mipmaps[level].img {
                compute_mipmap(level, source, dest);
            }
        }
    }
}

fn compute_mipmap(level: usize, source: &image::RgbaImage, dest: &mut image::RgbaImage) {
    for y in 0..source.height() {
        for x in 0..source.width() {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut count = 0;
            for sy in y << (level + 1)..(y + 1) << (level + 1) {
                for sx in x << (level + 1)..(x + 1) << (level + 1) {
                    let p = &source.get_pixel(sx, sy).data;
                    count += 1;
                    r += p[0];
                    g += p[1];
                    b += p[2];
                }
            }
            r /= count;
            g /= count;
            b /= count;
            dest.put_pixel(x, y, image::Pixel::from_channels(r, g, b, 255));
        }
    }
}
