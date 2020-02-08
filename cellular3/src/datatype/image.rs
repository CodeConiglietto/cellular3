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

use crate::{
    constants::*,
    datatype::colors::IntColor,
    preloader::{Generator, Preloader},
    util::{self, DeterministicRng},
};

lazy_static! {
    static ref ALL_IMAGES: Vec<PathBuf> = util::collect_filenames(IMAGE_PATH);
    static ref FALLBACK_IMAGE: Image =
        Image::load(Cursor::new(FALLBACK_IMAGE_DATA), ImageFormat::PNG)
            .unwrap_or_else(|e| panic!("Error loading fallback image: {}", e));
}

thread_local! {
    static IMAGE_PRELOADER: Preloader<Image> = Preloader::new(2, RandomImageLoader::new());
}

const FALLBACK_IMAGE_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fallback_image.png"));

struct RandomImageLoader {
    rng: DeterministicRng,
}

impl RandomImageLoader {
    fn new() -> Self {
        Self {
            rng: DeterministicRng::new(),
        }
    }
}

impl Generator for RandomImageLoader {
    type Output = Image;

    fn generate(&mut self) -> Self::Output {
        if let Some(filename) = ALL_IMAGES.choose(&mut self.rng) {
            println!("Loading image from file: {}", filename.to_string_lossy());

            Image::load_file(&filename).unwrap_or_else(|e| {
                panic!(
                    "Error loading image '{}': {}",
                    filename.to_string_lossy(),
                    e,
                )
            })
        } else {
            println!("Loading image from fallback data");
            (*FALLBACK_IMAGE).clone()
        }
    }
}

#[derive(Clone)]
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

    pub fn get_pixel_wrapped(&self, x: u32, y: u32, t: u32) -> IntColor {
        let frame_count = self.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.frames[t_value].width();
        let image_height = self.frames[t_value].height();

        //TODO refactor into helper method
        (*self.frames[t_value].get_pixel(
            ((x % image_width) + image_width) % image_width,
            ((y % image_height) + image_height) % image_height,
        ))
        .into()
    }

    //get a pixel from coords (-1.0..1.0, -1.0..1.0, 0.0..infinity)
    pub fn get_pixel_normalised(&self, x: f32, y: f32, t: f32) -> IntColor {
        let frame_count = self.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.frames[t_value].width() as f32;
        let image_height = self.frames[t_value].height() as f32;

        self.get_pixel_wrapped(
            ((x + 1.0) * 0.5 * image_width) as u32,
            ((y + 1.0) * 0.5 * image_height) as u32,
            t_value as u32,
        )
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
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R, state: mutagen::State) -> Self {
        println!("Generating new image");
        IMAGE_PRELOADER.with(|p| p.get_next())
    }
}

impl Mutatable for Image {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        println!("Mutating image");
        *self = Self::generate_rng(rng, state);
    }
}
