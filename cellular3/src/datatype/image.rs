use std::{path::PathBuf, ffi::OsStr, fs::File, io::BufReader, path::Path};
use rand::prelude::*;

use image::{gif, imageops, AnimationDecoder, DynamicImage, FilterType, RgbImage};

use mutagen::{Generatable, Mutatable};

use crate::{
    datatype::colors::IntColor, 
    constants::*
};

#[derive(Debug)]
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

impl Generatable for Image {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        Self::load_file(PathBuf::from(IMAGE_PATH).join("test.gif"))
    }
}

impl Mutatable for Image {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R) {
        *self = Self::generate();
    }
}

fn load_frames(filename: &Path) -> image::ImageResult<Vec<RgbImage>> {
    // Special handling for gifs in case they are animated
    if filename.extension() == Some(OsStr::new("gif")) {
        Ok(gif::Decoder::new(BufReader::new(File::open(filename)?))?
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| {
                imageops::resize(
                    &DynamicImage::ImageRgba8(f.into_buffer()).to_rgb(),
                    CELL_ARRAY_WIDTH as u32,
                    CELL_ARRAY_HEIGHT as u32,
                    FilterType::Gaussian,
                )
            })
            .collect())
    } else {
        Ok(vec![imageops::resize(
            &image::open(&filename)?.to_rgb(),
            CELL_ARRAY_WIDTH as u32,
            CELL_ARRAY_HEIGHT as u32,
            FilterType::Gaussian,
        )])
    }
}
