use std;

use uni_app;

struct AsyncImage(String, uni_app::fs::File);

pub struct FontLoader {
    async_images: Vec<Option<AsyncImage>>,
    pub image_data: Option<Vec<u8>>,
    pub char_width: u32,
    pub char_height: u32,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            async_images: Vec::new(),
            image_data: None,
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
                            self.image_data = Some(buf.to_vec());
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
                    self.image_data = Some(buf.to_vec());
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
