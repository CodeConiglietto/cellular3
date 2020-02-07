use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

use image::{gif, AnimationDecoder, DynamicImage, RgbImage};

use crate::datatype::colors::IntColor;

pub struct Image {
    frames: Vec<RgbImage>,
}

impl Image {
    pub fn new(frames: Vec<RgbImage>) -> Self {
        Self { frames }
    }

    pub fn load_file<P: AsRef<Path>>(filename: P) -> Self {
        let frames = load_frames(filename.as_ref()).unwrap_or_else(|e| {
            panic!(
                "Error loading image '{}': {}",
                filename.as_ref().to_string_lossy(),
                e
            )
        });

        Self::new(frames)
    }

    pub fn get_pixel(&self, x: u32, y: u32, t: u32) -> IntColor {
        (*self.frames[t as usize % self.frames.len()].get_pixel(x, y)).into()
    }
}

fn load_frames(filename: &Path) -> image::ImageResult<Vec<RgbImage>> {
    // Special handling for gifs in case they are animated
    if filename.extension() == Some(OsStr::new("gif")) {
        Ok(gif::Decoder::new(BufReader::new(File::open(filename)?))?
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| DynamicImage::ImageRgba8(f.into_buffer()).to_rgb())
            .collect())
    } else {
        Ok(vec![image::open(&filename)?.to_rgb()])
    }
}
