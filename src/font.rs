use std;

use image;
use uni_app;

struct AsyncImage(String, uni_app::fs::File);

pub struct FontLoader {
    async_images: Vec<Option<AsyncImage>>,
    pub img: Option<image::RgbaImage>,
    pub char_width: u32,
    pub char_height: u32,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            async_images: Vec::new(),
            img: None,
            char_width: 0,
            char_height: 0,
        }
    }
    pub fn load_font(&mut self, path: &str) {
        let start = path.rfind("_").unwrap_or(0);
        let end = path.rfind(".").unwrap_or(0);
        if start > 0 && end > 0 {
            let subpath = path[start + 1..end].to_owned();
            let charsize: Vec<&str> = subpath.split("x").collect();
            self.char_width = charsize[0].parse::<u32>().unwrap();
            self.char_height = charsize[1].parse::<u32>().unwrap();
        } else {
            self.char_width = 0;
            self.char_height = 0;
        }

        uni_app::App::print(format!("loading font {}\n", path));
        match open_file(path) {
            Ok(mut f) => {
                if f.is_ready() {
                    match f.read_binary() {
                        Ok(buf) => {
                            self.load_font_bytes(&buf);
                        }
                        Err(e) => panic!("Could not read file {} : {}\n", path, e),
                    }
                } else {
                    uni_app::App::print(format!("loading async file {}\n", path));
                    self.async_images.push(Some(AsyncImage(path.to_owned(), f)));
                }
            }
            Err(e) => panic!("Could not open file {} : {}\n", path, e),
        }
    }

    pub fn load_font_async(&mut self) -> bool {
        if self.async_images.len() == 0 {
            return true;
        }
        let mut to_load = Vec::new();
        let mut idx = 0;
        for ref oasfile in self.async_images.iter() {
            if let &&Some(ref asfile) = oasfile {
                if asfile.1.is_ready() {
                    to_load.push(idx);
                }
                idx += 1;
            }
        }
        for idx in to_load.iter() {
            let mut asfile = self.async_images[*idx].take().unwrap();
            match asfile.1.read_binary() {
                Ok(buf) => {
                    self.load_font_bytes(&buf);
                    return true;
                }
                Err(e) => {
                    uni_app::App::print(format!("could not load async file {} : {}", asfile.0, e))
                }
            }
        }
        self.async_images.retain(|f| f.is_some());
        return false;
    }

    fn load_font_bytes(&mut self, buf: &Vec<u8>) {
        let mut img = image::load_from_memory(buf).unwrap().to_rgba();
        self.process_image(&mut img);
        self.img = Some(img);
    }

    fn process_image(&mut self, img: &mut image::RgbaImage) {
        let pixel = img.get_pixel(0, 0).data;
        let alpha = pixel[3];
        if alpha == 255 {
            let transparent_color = (pixel[0], pixel[1], pixel[2]);
            uni_app::App::print(format!("transparent color: {:?}\n", transparent_color));
            let (width, height) = img.dimensions();
            for y in 0..height {
                for x in 0..width {
                    let p = img.get_pixel_mut(x, y);
                    let pixel = p.data;
                    if (pixel[0], pixel[1], pixel[2]) == transparent_color {
                        p.data[3] = 0;
                    } else {
                        let alpha = pixel[0];
                        p.data[0] = 255;
                        p.data[1] = 255;
                        p.data[2] = 255;
                        p.data[3] = alpha;
                    }
                }
            }
        }
    }
}

fn open_file(filename: &str) -> Result<uni_app::fs::File, std::io::Error> {
    let ffilename =
        if cfg!(not(target_arch = "wasm32")) && &filename[0..1] != "/" && &filename[1..2] != ":" {
            "static/".to_owned() + filename
        } else {
            filename.to_owned()
        };
    uni_app::fs::FileSystem::open(&ffilename)
}
