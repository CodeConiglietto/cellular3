use std::{
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Seek},
    path::{Path, PathBuf},
};

use image::{gif, imageops, AnimationDecoder, DynamicImage, FilterType, ImageFormat, RgbImage};
use lazy_static::lazy_static;
use mutagen::{Generatable, Mutatable};
use rand::prelude::*;

use crate::{constants::*, datatype::colors::IntColor, preloader::Preloader, util};

lazy_static! {
    static ref ALL_IMAGES: Vec<PathBuf> = util::collect_filenames(IMAGE_PATH);
}

thread_local! {
    static IMAGE_PRELOADER: Preloader<Image> = Preloader::new(5, load_random_image);
}

fn load_random_image() -> Image {
    if let Some(filename) = ALL_IMAGES.choose(&mut thread_rng()) {
        Image::load_file(&filename).unwrap_or_else(|e| {
            panic!(
                "Error loading image '{}': {}",
                filename.to_string_lossy(),
                e
            )
        })
    } else {
        Image::load(Cursor::new(FALLBACK_IMAGE), ImageFormat::PNG)
            .unwrap_or_else(|e| panic!("Error loading fallback image: {}", e))
    }
}

const FALLBACK_IMAGE: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fallback_image.png"));

pub struct Image {
    frames: Vec<RgbImage>,
}

impl Image {
    pub fn new(frames: Vec<RgbImage>) -> Self {
        Self { frames }
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        Ok(Self::new(load_frames(
            BufReader::new(File::open(&path)?),
            ImageFormat::from_path(&path)?,
        )?))
    }

    pub fn load<R: BufRead + Seek>(reader: R, format: ImageFormat) -> image::ImageResult<Self> {
        Ok(Self::new(load_frames(reader, format)?))
    }

    pub fn get_pixel(&self, x: u32, y: u32, t: u32) -> IntColor {
        (*self.frames[t as usize % self.frames.len()].get_pixel(x, y)).into()
    }
}

fn load_frames<R: BufRead + Seek>(
    reader: R,
    format: ImageFormat,
) -> image::ImageResult<Vec<RgbImage>> {
    // Special handling for gifs in case they are animated
    match format {
        ImageFormat::GIF => Ok(gif::Decoder::new(reader)?
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
            .collect()),

        _ => Ok(vec![imageops::resize(
            &image::load(reader, format)?.to_rgb(),
            CELL_ARRAY_WIDTH as u32,
            CELL_ARRAY_HEIGHT as u32,
            FilterType::Gaussian,
        )]),
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("frames", &self.frames.len())
            .finish()
    }
}

impl Generatable for Image {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        IMAGE_PRELOADER.with(|p| p.get_next())
    }
}

impl Mutatable for Image {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, _rng: &mut R) {
        *self = Self::generate();
    }
}
