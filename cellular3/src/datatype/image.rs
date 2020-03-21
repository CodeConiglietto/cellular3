use std::{
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Seek},
    path::{Path, PathBuf},
    sync::Arc,
};

use failure::{format_err, Fallible};
use image::{gif, imageops, AnimationDecoder, FilterType, ImageFormat, RgbaImage};
use lazy_static::lazy_static;
use log::{debug, error, warn};
use mutagen::{Generatable, Mutatable};
use rand::prelude::*;
use reqwest::blocking::Client as HttpClient;

use crate::{
    constants::*,
    datatype::{colors::ByteColor, continuous::*},
    preloader::{Generator, Preloader},
    util::{self, DeterministicRng},
};

pub const MODULE_PATH: &str = module_path!();

lazy_static! {
    static ref ALL_IMAGES: Vec<PathBuf> = util::collect_filenames(&CONSTS.image_path);
    static ref FALLBACK_IMAGE: Image = Image::load(
        String::from("<FALLBACK>"),
        Cursor::new(FALLBACK_IMAGE_DATA),
        ImageFormat::PNG,
    )
    .unwrap_or_else(|e| {
        error!("Error loading fallback image: {}", e);
        panic!()
    });
}

thread_local! {
    pub static IMAGE_PRELOADER: Preloader<Image> = Preloader::new(32, RandomImageLoader::new());
}

const FALLBACK_IMAGE_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fallback_image.png"));

struct RandomImageLoader {
    rng: DeterministicRng,
    http: HttpClient,
}

impl RandomImageLoader {
    fn new() -> Self {
        Self {
            rng: DeterministicRng::new(),
            http: HttpClient::new(),
        }
    }
}

impl Generator for RandomImageLoader {
    type Output = Image;

    fn generate(&mut self) -> Self::Output {
        if self.rng.gen_bool(0.5) {
            download_random_image(&mut self.http).unwrap_or_else(|e| {
                warn!("Failed to download image: {}", e);
                load_random_image_file(&mut self.rng)
            })
        } else {
            load_random_image_file(&mut self.rng)
        }
    }
}

fn download_random_image(client: &mut HttpClient) -> Fallible<Image> {
    let mut buf = Vec::new();

    let mut response = client
        .get(&format!(
            "https://picsum.photos/{}/{}",
            CONSTS.initial_window_width.floor() as usize,
            CONSTS.initial_window_height.floor() as usize,
        ))
        .send()?
        .error_for_status()?;

    response.copy_to(&mut buf)?;

    let url = response.url();
    let filename = url
        .path_segments()
        .ok_or_else(|| format_err!("Couldn't parse url: {}", url.as_str()))?
        .last()
        .ok_or_else(|| format_err!("Empty url: {}", url.as_str()))?;

    let name = format!("{} (Lorem Picsum)", &filename);
    let format = ImageFormat::from_path(&filename)?;

    debug!("Downloaded image: {}", name);

    Ok(Image::load(name, Cursor::new(&buf), format)?)
}

fn load_random_image_file<R: Rng + ?Sized>(rng: &mut R) -> Image {
    if let Some(filename) = ALL_IMAGES.choose(rng) {
        debug!("Loading image file '{}'", filename.to_string_lossy());
        Image::load_file(&filename).unwrap_or_else(|e| {
            error!(
                "Failed to load image file '{}': {}",
                filename.to_string_lossy(),
                e
            );
            FALLBACK_IMAGE.clone()
        })
    } else {
        debug!("No images found, loading fallback image");
        FALLBACK_IMAGE.clone()
    }
}

#[derive(Clone)]
pub struct Image(Arc<ImageData>);

pub struct ImageData {
    name: String,
    frames: Vec<RgbaImage>,
}

impl Image {
    pub fn new(name: String, frames: Vec<RgbaImage>) -> Self {
        Self(Arc::new(ImageData { name, frames }))
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        Ok(Self::new(
            format!("{} (Local file)", path.as_ref().to_string_lossy()),
            load_frames(
                BufReader::new(File::open(&path)?),
                ImageFormat::from_path(&path)?,
            )?,
        ))
    }

    pub fn load<R: BufRead + Seek>(
        name: String,
        reader: R,
        format: ImageFormat,
    ) -> image::ImageResult<Self> {
        Ok(Self::new(name, load_frames(reader, format)?))
    }

    pub fn get_pixel_wrapped(&self, x: u32, y: u32, t: u32) -> ByteColor {
        let frame_count = self.0.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.0.frames[t_value].width();
        let image_height = self.0.frames[t_value].height();

        //TODO refactor into helper method
        (*self.0.frames[t_value].get_pixel(
            ((x % image_width) + image_width) % image_width,
            ((y % image_height) + image_height) % image_height,
        ))
        .into()
    }

    //get a pixel from coords (-1.0..1.0, -1.0..1.0, 0.0..infinity)
    pub fn get_pixel_normalised(&self, x: SNFloat, y: SNFloat, t: f32) -> ByteColor {
        let frame_count = self.0.frames.len();
        let t_value = ((t as usize % frame_count) + frame_count) % frame_count;

        let image_width = self.0.frames[t_value].width() as f32;
        let image_height = self.0.frames[t_value].height() as f32;

        self.get_pixel_wrapped(
            (x.to_unsigned().into_inner() * image_width) as u32,
            (y.to_unsigned().into_inner() * image_height) as u32,
            t_value as u32,
        )
    }
}

fn load_frames<R: BufRead + Seek>(
    reader: R,
    format: ImageFormat,
) -> image::ImageResult<Vec<RgbaImage>> {
    // Special handling for gifs in case they are animated
    match format {
        ImageFormat::GIF => Ok(gif::Decoder::new(reader)?
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| {
                imageops::resize(
                    &f.into_buffer(),
                    CONSTS.cell_array_width as u32,
                    CONSTS.cell_array_height as u32,
                    FilterType::Gaussian,
                )
            })
            .collect()),

        _ => Ok(vec![imageops::resize(
            &image::load(reader, format)?.to_rgba(),
            CONSTS.cell_array_width as u32,
            CONSTS.cell_array_height as u32,
            FilterType::Gaussian,
        )]),
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("name", &self.0.name)
            .field("frames", &self.0.frames.len())
            .finish()
    }
}

impl Generatable for Image {
    fn generate_rng<R: Rng + ?Sized>(_rng: &mut R, _state: mutagen::State) -> Self {
        IMAGE_PRELOADER
            .with(|p| p.try_get_next())
            .unwrap_or_else(|| FALLBACK_IMAGE.clone())
    }
}

impl Mutatable for Image {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}
