use crate::file::FileLoader;

pub struct FontLoader {
    loader: FileLoader,
    pub img: Option<image::RgbaImage>,
    pub char_width: u32,
    pub char_height: u32,
    id: usize,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            loader: FileLoader::new(),
            img: None,
            char_width: 0,
            char_height: 0,
            id: 0,
        }
    }
    pub fn load_font(&mut self, path: &str) {
        let start = path.rfind('_').unwrap_or(0);
        let end = path.rfind('.').unwrap_or(0);
        if start > 0 && end > 0 {
            let subpath = path[start + 1..end].to_owned();
            let charsize: Vec<&str> = subpath.split('x').collect();
            self.char_width = charsize[0].parse::<u32>().unwrap();
            self.char_height = charsize[1].parse::<u32>().unwrap();
        } else {
            self.char_width = 0;
            self.char_height = 0;
        }
        match self.loader.load_file(path) {
            Ok(id) => {
                self.id = id;
                self.load_font_async();
            }
            Err(msg)=> {
                eprint!("Error while loading file {} : {}",path,msg);
            }
        }
    }

    pub fn load_font_async(&mut self) -> bool {
        if self.img.is_some() {
            return true;
        }
        if self.loader.check_file_ready(self.id) {
            let buf = self.loader.get_file_content(self.id);
            self.load_font_bytes(&buf);
            return true;
        }
        false
    }

    fn load_font_bytes(&mut self, buf: &[u8]) {
        let mut img = image::load_from_memory(buf).unwrap().to_rgba8();
        self.process_image(&mut img);
        self.img = Some(img);
    }

    fn process_image(&mut self, img: &mut image::RgbaImage) {
        let pixel = img.get_pixel(0, 0);
        let alpha = pixel[3];
        if alpha == 255 {
            let transparent_color = (pixel[0], pixel[1], pixel[2]);
            let greyscale = transparent_color == (0, 0, 0);
            uni_app::App::print(format!(
                "{}transparent color: {:?}\n",
                if greyscale { "greyscale " } else { "" },
                transparent_color
            ));
            let (width, height) = img.dimensions();
            for y in 0..height {
                for x in 0..width {
                    let pixel = img.get_pixel_mut(x, y);
                    if (pixel[0], pixel[1], pixel[2]) == transparent_color {
                        pixel[3] = 0;
                        pixel[0] = 0;
                        pixel[1] = 0;
                        pixel[2] = 0;
                    } else if greyscale && pixel[0] == pixel[1] && pixel[1] == pixel[2] {
                        let alpha = pixel[0];
                        pixel[0] = 255;
                        pixel[1] = 255;
                        pixel[2] = 255;
                        pixel[3] = alpha;
                    }
                }
            }
        }
    }
}
