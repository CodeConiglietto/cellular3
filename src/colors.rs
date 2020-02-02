use ggez::graphics::Color as GGColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

impl From<Color> for GGColor {
    fn from(c: Color) -> GGColor {
        GGColor {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: 1.0,
        }
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
    pub fn get_color(&self) -> Color {
        match self {
            PalletteColor::Black => Color { r: 0, g: 0, b: 0 },
            PalletteColor::Red => Color { r: 255, g: 0, b: 0 },
            PalletteColor::Green => Color { r: 0, g: 255, b: 0 },
            PalletteColor::Blue => Color { r: 0, g: 0, b: 255 },
            PalletteColor::Cyan => Color {
                r: 0,
                g: 255,
                b: 255,
            },
            PalletteColor::Magenta => Color {
                r: 255,
                g: 0,
                b: 255,
            },
            PalletteColor::Yellow => Color {
                r: 255,
                g: 255,
                b: 0,
            },
            PalletteColor::White => Color {
                r: 255,
                g: 255,
                b: 255,
            },
        }
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
            _ => panic!(),
        }
    }

    pub fn to_composites(&self) -> [bool; 3] {
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

    pub fn from_composites(composites: [bool; 3]) -> PalletteColor {
        match composites {
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
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            has_color = has_color || (current_color[i] && other_color[i]);
        }

        has_color
    }

    pub fn give_color(&mut self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            new_color[i] = current_color[i] || other_color[i];
        }

        new_color
    }

    pub fn take_color(&mut self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            new_color[i] = !(current_color[i] && other_color[i]);
        }

        new_color
    }
}
