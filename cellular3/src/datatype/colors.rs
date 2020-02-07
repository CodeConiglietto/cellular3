use mutagen::{Generatable, Mutatable};
use palette::rgb::Rgb;
use rand::prelude::*;

pub type FloatColor = ggez::graphics::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<image::Rgb<u8>> for IntColor {
    fn from(c: image::Rgb<u8>) -> Self {
        Self {
            r: c.0[0],
            g: c.0[1],
            b: c.0[2],
        }
    }
}

impl From<IntColor> for FloatColor {
    fn from(c: IntColor) -> FloatColor {
        FloatColor {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: 1.0,
        }
    }
}

impl From<PalletteColor> for FloatColor {
    fn from(c: PalletteColor) -> FloatColor {
        let color_components = c.to_components();

        FloatColor {
            r: if color_components[0] { 1.0 } else { 0.0 },
            g: if color_components[1] { 1.0 } else { 0.0 },
            b: if color_components[2] { 1.0 } else { 0.0 },
            a: 1.0,
        }
    }
}

pub fn float_color_from_pallette_rgb(rgb: Rgb) -> FloatColor {
    FloatColor {
        r: rgb.red as f32,
        g: rgb.green as f32,
        b: rgb.blue as f32,
        a: 1.0,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PalletteColor {
    Black,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
}

impl PalletteColor {
    pub fn get_color(&self) -> IntColor {
        match self {
            PalletteColor::Black => IntColor { r: 0, g: 0, b: 0 },
            PalletteColor::Red => IntColor { r: 255, g: 0, b: 0 },
            PalletteColor::Green => IntColor { r: 0, g: 255, b: 0 },
            PalletteColor::Blue => IntColor { r: 0, g: 0, b: 255 },
            PalletteColor::Cyan => IntColor {
                r: 0,
                g: 255,
                b: 255,
            },
            PalletteColor::Magenta => IntColor {
                r: 255,
                g: 0,
                b: 255,
            },
            PalletteColor::Yellow => IntColor {
                r: 255,
                g: 255,
                b: 0,
            },
            PalletteColor::White => IntColor {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }

    pub fn from_float_color(c: FloatColor) -> PalletteColor {
        PalletteColor::from_components([c.r >= 0.5, c.g >= 0.5, c.b >= 0.5])
    }

    pub fn to_index(&self) -> usize {
        match self {
            PalletteColor::Black => 0,
            PalletteColor::Red => 1,
            PalletteColor::Green => 2,
            PalletteColor::Blue => 3,
            PalletteColor::Cyan => 4,
            PalletteColor::Magenta => 5,
            PalletteColor::Yellow => 6,
            PalletteColor::White => 7,
        }
    }

    pub fn from_index(index: usize) -> PalletteColor {
        match index {
            0 => PalletteColor::Black,
            1 => PalletteColor::Red,
            2 => PalletteColor::Green,
            3 => PalletteColor::Blue,
            4 => PalletteColor::Cyan,
            5 => PalletteColor::Magenta,
            6 => PalletteColor::Yellow,
            7 => PalletteColor::White,
            _ => {
                dbg!(index);
                panic!()
            }
        }
    }

    pub fn to_components(&self) -> [bool; 3] {
        match self {
            PalletteColor::Black => [false, false, false],
            PalletteColor::Red => [true, false, false],
            PalletteColor::Green => [false, true, false],
            PalletteColor::Blue => [false, false, true],
            PalletteColor::Cyan => [false, true, true],
            PalletteColor::Magenta => [true, false, true],
            PalletteColor::Yellow => [true, true, false],
            PalletteColor::White => [true, true, true],
        }
    }

    pub fn from_components(components: [bool; 3]) -> PalletteColor {
        match components {
            [false, false, false] => PalletteColor::Black,
            [true, false, false] => PalletteColor::Red,
            [false, true, false] => PalletteColor::Green,
            [false, false, true] => PalletteColor::Blue,
            [false, true, true] => PalletteColor::Cyan,
            [true, false, true] => PalletteColor::Magenta,
            [true, true, false] => PalletteColor::Yellow,
            [true, true, true] => PalletteColor::White,
        }
    }

    pub fn has_color(&self, other: PalletteColor) -> bool {
        let mut has_color = false;
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            has_color = has_color || (current_color[i] && other_color[i]);
        }

        has_color
    }

    pub fn give_color(&self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] || other_color[i];
        }

        new_color
    }

    pub fn take_color(&self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] && !other_color[i];
        }

        new_color
    }

    pub fn xor_color(&self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] =
                (current_color[i] || other_color[i]) && !(current_color[i] && other_color[i]);
        }

        new_color
    }

    pub fn eq_color(&self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] == other_color[i];
        }

        new_color
    }
}

impl Generatable for PalletteColor {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::from_components([rng.gen::<bool>(), rng.gen::<bool>(), rng.gen::<bool>()])
    }
}
impl Mutatable for PalletteColor {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let current_color = self.to_components();
        let mut new_color = [rng.gen::<bool>(), rng.gen::<bool>(), rng.gen::<bool>()];

        for i in 0..3 {
            if rng.gen::<bool>() {
                new_color[i] = current_color[i];
            }
        }

        *self = Self::from_components(new_color);
    }
}
