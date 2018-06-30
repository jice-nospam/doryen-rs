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
    pub fn blit(
        &mut self,
        con: &mut Console,
        x: i32,
        y: i32,
        scalex: f32,
        scaley: f32,
        angle: f32,
        transparent: Option<Color>,
    ) {
        if !self.is_loaded() || scalex == 0.0 || scaley == 0.0 {
            return;
        }
        if let Some(ref img) = self.img {
            let width = img.width() as i32;
            let height = img.height() as i32;
            let rx = x as f32 - width as f32 * 0.5;
            let ry = y as f32 - height as f32 * 0.5;
            if scalex == 1.0
                && scaley == 1.0
                && angle == 0.0
                && rx.floor() == rx
                && ry.floor() == ry
            {
                let ix = rx as i32;
                let iy = ry as i32;
                let minx = ix.max(0);
                let miny = iy.max(0);
                let maxx = (ix + width).min(con.get_width() as i32);
                let maxy = (iy + height).min(con.get_height() as i32);
                let mut offx = if ix < 0 { -ix } else { 0 };
                let mut offy = if iy < 0 { -iy } else { 0 };
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
            } else {
                // TODO
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
