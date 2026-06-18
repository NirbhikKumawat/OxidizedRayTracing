use image::{ImageReader, RgbImage};
use std::path::Path;

pub struct RtwImage {
    width: u32,
    height: u32,
    pub(crate) image_data: Option<RgbImage>,
}
impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let mut img = Self {
            image_data: None,
            width: 0,
            height: 0,
        };
        if !img.load(Path::new(filename)) {
            eprintln!("Could not load image: {}", filename);
        }
        img
    }
    fn load(&mut self, path: &Path) -> bool {
        match ImageReader::open(path) {
            Ok(reader) => match reader.decode() {
                Ok(dynamic_image) => {
                    let rgb_image = dynamic_image.into_rgb8();
                    self.width = rgb_image.width();
                    self.height = rgb_image.height();
                    self.image_data = Some(rgb_image);
                    true
                }
                Err(e) => {
                    eprintln!("ERROR: Failed to decode image at {:?}: {}", path, e);
                    false
                }
            },
            Err(e) => {
                eprintln!("ERROR: Failed to decode image at {:?}: {}", path, e);
                false
            }
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        let magenta = [255, 0, 255];
        if let Some(ref img) = self.image_data {
            let clamped_x = x.clamp(0, self.width as i32 - 1) as u32;
            let clamped_y = y.clamp(0, self.height as i32 - 1) as u32;
            let pixel = img.get_pixel(clamped_x, clamped_y);
            pixel.0
        } else {
            magenta
        }
    }
}
