//! A small and minimalistic library to generate a random color palette.
//! The user-facing `Color` struct contains RGB colors ranging from 0 to 1.
//! All colors are of type f32 (no exceptions)
//! 
//! # Usage
//! 
//! ```rust
//! use colourado_iter::{Color, ColorPalette, PaletteType};
//! 
//! let mut palette = ColorPalette::new(PaletteType::Random, false, &mut rand::thread_rng());
//! let random_color = palette.next();
//! let color_array: [f32; 3] = palette.next().unwrap().to_array();
//! let many_colors = palette.take(20);
//! let hue = 315.0;
//! let saturation = 0.5;
//! let value = 0.3;
//! let rgb_color: Color = Color::hsv_to_rgb(hue, saturation, value);
//! ```
//! 
//! The second param to ColorPalette::new() determines the color scheme.  
//! Currently 3 different schemes are supported:  
//! `PaletteType::Random` generates random colors 
//! `PaletteType::Pastel` generates pastel colors 
//! `PaletteType::Dark` generates dark colors  
//! 
//! The third param determines whether colors are generated close to each other
//! or are spread apart. `true` generates adjacent colors while `false` will generate
//! a very spread color palette.
//!
//! Optionally, you can use the `HsvPalette` struct to get a generator which spits out the immediate HSV values as opposed to a `Color` struct.
//! 
//! **WARNING** The `ColorPalette` iterator is infinite! It will never exhaust! As such, you should never
//! use `collect` or `for x in` patterns with it. Instead, always use `take` if you want a certain number of colors. 

use rand::Rng;

mod color;
pub use color::Color;

/// Container for a vector of colors.
/// You can also use it to store your own custom palette of you so desire. 
pub struct HsvPalette {
    iteration: usize,
    base_divergence: f32,
    palette_type: PaletteType,
    hue: Hue,
}

pub struct ColorPalette(HsvPalette);

pub enum PaletteType {
    Random,
    Pastel,
    Dark,
}

pub(crate) type Hue = f32;
pub(crate) type Saturation = f32;
pub(crate) type Value = f32;
pub type Hsv = (Hue, Saturation, Value);

impl ColorPalette {
    pub fn new<T: Rng>(palette_type: PaletteType, adjacent_colors: bool, rng: &mut T) -> Self {

        let hue = rng.gen_range(0.0..360.0);

        let mut base_divergence = 80.0;

        if adjacent_colors {
            base_divergence = 25.0;
        }

        Self(HsvPalette {
            base_divergence,
            palette_type,
            hue,
            iteration: 0
        })
    }

    pub fn get_inner(&self) -> &HsvPalette {
        &self.0
    }

    pub fn into_inner(self) -> HsvPalette {
        self.0
    }
}

impl HsvPalette {
    fn palette_dark(&self) -> Hsv {
        let iteration = self.iteration as f32;
        let f = (iteration * 43.0).cos().abs();
        let mut div = self.base_divergence;

        if div < 15.0 {
            div = 15.0;
        }

        let hue = (self.hue + div + f).abs() % 360.0;
        let saturation = 0.32 + ((iteration * 0.75).sin() / 2.0).abs();
        let value = 0.1 + (iteration.cos() / 6.0).abs();
        (hue, saturation, value)
    }

    fn palette_pastel(&self) -> Hsv  {
        let iteration = self.iteration as f32;
        let f = (iteration * 25.0).cos().abs();
        let mut div = self.base_divergence;

        if div < 15.0 {
            div = 15.0;
        }

        let hue = (self.hue + div + f).abs() % 360.0;
        let saturation = ((iteration * 0.35).cos() / 5.0).abs();
        let value = 0.5 + (iteration.cos() / 2.0).abs();
        (hue, saturation, value)
    }

    fn palette_random(&self) -> Hsv  {
        let iteration = self.iteration as f32;
        let f = (iteration * 55.0).tan().abs();
        let mut div = self.base_divergence;

        if div < 15.0 {
            div = 15.0;
        }

        let hue = (self.hue + div + f).abs() % 360.0;
        let mut saturation = (iteration * 0.35).sin().abs();
        let mut value = ((6.33 * iteration) * 0.5).cos().abs();

        if saturation < 0.4 {
            saturation = 0.4;
        }

        if value < 0.2 {
            value = 0.2;
        } else if value > 0.85 {
            value = 0.85;
        }
        (hue, saturation, value)    
    }

    pub fn get(&self) -> Hsv {
        match self.palette_type {
            PaletteType::Random => self.palette_random(),
            PaletteType::Pastel => self.palette_pastel(),
            PaletteType::Dark => self.palette_dark(),
        }
    }
}

impl Iterator for HsvPalette {
    type Item = Hsv;

    fn next(&mut self) -> Option<Self::Item> {
        let (hue, saturation, value) = self.get();
        self.hue = hue;
        self.iteration += 1;
        Some((hue, saturation, value))
    }
}

impl Iterator for ColorPalette {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((hue, saturation, value)) = self.0.next() {
            Some(Color::hsv_to_rgb(hue, saturation, value))
        } else {
            None
        }
    }
}



#[cfg(test)]
mod tests {
    use super::ColorPalette;
    use super::PaletteType;

    #[test]
    fn generates_palette() {
        let palette = ColorPalette::new(PaletteType::Random, false, &mut rand::thread_rng());

        let colors = palette.take(7);

        for color in colors {
            let (red, green, blue) = color.to_tuple();
            assert!(red >= 0.0);
            assert!(red <= 1.0);

            assert!(green >= 0.0);
            assert!(green <= 1.0);

            assert!(blue >= 0.0);
            assert!(blue <= 1.0);
        }        
    }
}
