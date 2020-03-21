use mutagen::{Generatable, Mutatable};
use palette::rgb::Rgb;
use rand::prelude::*;

use crate::datatype::{continuous::*, discrete::*};

pub type FloatColor = ggez::graphics::Color;

pub fn get_average(c: FloatColor) -> f32 {
    (c.r + c.b + c.g) / 3.0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NibbleColor {
    pub r: Nibble,
    pub g: Nibble,
    pub b: Nibble,
    pub a: Nibble,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<FloatColor> for NibbleColor {
    fn from(other: FloatColor) -> Self {
        Self {
            r: Nibble::new((other.r * 16.0) as u8),
            g: Nibble::new((other.g * 16.0) as u8),
            b: Nibble::new((other.b * 16.0) as u8),
            a: Nibble::new((other.a * 16.0) as u8),
        }
    }
}

impl Generatable for ByteColor {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Self {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
            a: rng.gen(),
        }
    }
}

impl Mutatable for ByteColor {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, state: mutagen::State) {
        *self = Self::generate_rng(rng, state);
    }
}

impl From<image::Rgba<u8>> for ByteColor {
    fn from(c: image::Rgba<u8>) -> Self {
        Self {
            r: c.0[0],
            g: c.0[1],
            b: c.0[2],
            a: c.0[3],
        }
    }
}

impl From<FloatColor> for ByteColor {
    fn from(other: FloatColor) -> Self {
        Self {
            r: (other.r * 255.0) as u8,
            g: (other.g * 255.0) as u8,
            b: (other.b * 255.0) as u8,
            a: (other.a * 255.0) as u8,
        }
    }
}

impl From<ByteColor> for FloatColor {
    fn from(c: ByteColor) -> FloatColor {
        FloatColor {
            r: c.r as f32 / 256.0,
            g: c.g as f32 / 256.0,
            b: c.b as f32 / 256.0,
            a: c.a as f32 / 256.0,
        }
    }
}

impl From<BitColor> for FloatColor {
    fn from(c: BitColor) -> FloatColor {
        let color_components = c.to_components();

        FloatColor {
            r: if color_components[0] { 1.0 } else { 0.0 },
            g: if color_components[1] { 1.0 } else { 0.0 },
            b: if color_components[2] { 1.0 } else { 0.0 },
            a: 1.0,
        }
    }
}

pub fn float_color_from_pallette_rgb(rgb: Rgb, alpha: f32) -> FloatColor {
    FloatColor {
        r: rgb.red as f32,
        g: rgb.green as f32,
        b: rgb.blue as f32,
        a: alpha,
    }
}

//Translated to rust from an answer here here: https://stackoverflow.com/questions/23090019/fastest-formula-to-get-hue-from-rgb
pub fn get_hue_unfloat(c: FloatColor) -> UNFloat {
    let r = c.r;
    let g = c.g;
    let b = c.b;

    let min = r.min(g.min(b));
    let max = r.min(g.min(b));

    if min == max {
        UNFloat::new(0.0)
    } else {
        let mut hue;
        if max == r {
            hue = (g - b) / (max - min);
        } else if max == g {
            hue = 2.0 + (b - r) / (max - min);
        } else {
            hue = 4.0 + (r - g) / (max - min);
        }

        hue = hue * 60.0;

        if hue < 0.0 {
            hue += 360.0;
        }

        UNFloat::new(hue / 360.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BitColor {
    Black,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
}

impl BitColor {
    pub fn get_color(self) -> ByteColor {
        match self {
            BitColor::Black => ByteColor { r: 0, g: 0, b: 0, a: 255 },
            BitColor::Red => ByteColor { r: 255, g: 0, b: 0, a: 255 },
            BitColor::Green => ByteColor { r: 0, g: 255, b: 0, a: 255 },
            BitColor::Blue => ByteColor { r: 0, g: 0, b: 255, a: 255 },
            BitColor::Cyan => ByteColor {
                r: 0,
                g: 255,
                b: 255,
                a: 255,
            },
            BitColor::Magenta => ByteColor {
                r: 255,
                g: 0,
                b: 255,
                a: 255,
            },
            BitColor::Yellow => ByteColor {
                r: 255,
                g: 255,
                b: 0,
                a: 255,
            },
            BitColor::White => ByteColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        }
    }

    pub fn from_float_color(c: FloatColor) -> BitColor {
        BitColor::from_components([c.r >= 0.5, c.g >= 0.5, c.b >= 0.5])
    }

    pub fn from_byte_color(c: ByteColor) -> BitColor {
        BitColor::from_components([c.r >= 127, c.g >= 127, c.b >= 127])
    }

    pub fn to_index(self) -> usize {
        match self {
            BitColor::Black => 0,
            BitColor::Red => 1,
            BitColor::Green => 2,
            BitColor::Blue => 3,
            BitColor::Cyan => 4,
            BitColor::Magenta => 5,
            BitColor::Yellow => 6,
            BitColor::White => 7,
        }
    }

    pub fn from_index(index: usize) -> BitColor {
        match index {
            0 => BitColor::Black,
            1 => BitColor::Red,
            2 => BitColor::Green,
            3 => BitColor::Blue,
            4 => BitColor::Cyan,
            5 => BitColor::Magenta,
            6 => BitColor::Yellow,
            7 => BitColor::White,
            _ => {
                dbg!(index);
                panic!()
            }
        }
    }

    pub fn to_components(self) -> [bool; 3] {
        match self {
            BitColor::Black => [false, false, false],
            BitColor::Red => [true, false, false],
            BitColor::Green => [false, true, false],
            BitColor::Blue => [false, false, true],
            BitColor::Cyan => [false, true, true],
            BitColor::Magenta => [true, false, true],
            BitColor::Yellow => [true, true, false],
            BitColor::White => [true, true, true],
        }
    }

    pub fn from_components(components: [bool; 3]) -> BitColor {
        match components {
            [false, false, false] => BitColor::Black,
            [true, false, false] => BitColor::Red,
            [false, true, false] => BitColor::Green,
            [false, false, true] => BitColor::Blue,
            [false, true, true] => BitColor::Cyan,
            [true, false, true] => BitColor::Magenta,
            [true, true, false] => BitColor::Yellow,
            [true, true, true] => BitColor::White,
        }
    }

    pub fn has_color(self, other: BitColor) -> bool {
        let mut has_color = false;
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            has_color = has_color || (current_color[i] && other_color[i]);
        }

        has_color
    }

    pub fn give_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] || other_color[i];
        }

        new_color
    }

    pub fn take_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] && !other_color[i];
        }

        new_color
    }

    pub fn xor_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] =
                (current_color[i] || other_color[i]) && !(current_color[i] && other_color[i]);
        }

        new_color
    }

    pub fn eq_color(self, other: BitColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_components();
        let other_color = other.to_components();

        for i in 0..3 {
            new_color[i] = current_color[i] == other_color[i];
        }

        new_color
    }
}

impl Generatable for BitColor {
    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _state: mutagen::State) -> Self {
        Self::from_components([rng.gen(), rng.gen(), rng.gen()])
    }
}

impl Mutatable for BitColor {
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _state: mutagen::State) {
        let current_color = self.to_components();
        let mut new_color = [rng.gen(), rng.gen(), rng.gen()];

        for i in 0..3 {
            if rng.gen::<bool>() {
                new_color[i] = current_color[i];
            }
        }

        *self = Self::from_components(new_color);
    }
}

impl From<ByteColor> for BitColor {
    fn from(other: ByteColor) -> Self {
        Self::from_components([other.r > 127, other.g > 127, other.b > 127])
    }
}
