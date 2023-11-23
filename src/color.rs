use std::format;

use crate::{Hue, Saturation, Value, Hsv};

/// A simple struct containing the three main color components of RGB color space.
/// Colors are stored as f32 values ranging from 0.0 to 1.0 
#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}

impl Color {
    /// Convert to an array of 3 floats
    pub fn to_array(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    /// Convert to a tuple of 3 floats
    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.red, self.green, self.blue)
    }

    /// Convert to an array for rgba (meaning it will just append 1.0 as the alpha value)
    pub fn to_rgba_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, 1.0]
    }

    /// Convert HSV to RGB. Plain and simple
    pub fn hsv_to_rgb(hue: Hue, saturation: Saturation, value: Value) -> Self {
        let chroma = value * saturation;
        let hue2 = hue / 60.0;
        let tmp = chroma * (1.0 - ((hue2 % 2.0) - 1.0).abs());

        let color2 = match hue2 {
            h if (0.0..1.0).contains(&h) => (chroma, tmp, 0.0),
            h if (1.0..2.0).contains(&h) => (tmp, chroma, 0.0),
            h if (2.0..3.0).contains(&h) => (0.0, chroma, tmp),
            h if (3.0..4.0).contains(&h) => (0.0, tmp, chroma),
            h if (4.0..5.0).contains(&h) => (tmp, 0.0, chroma),
            h if (5.0..6.0).contains(&h) => (chroma, 0.0, tmp),
            _ => (0.0, 0.0, 0.0)
        };

        let m = value - chroma;
        let red = color2.0 + m;
        let green = color2.1 + m;
        let blue = color2.2 + m;

        Color {
            red, 
            green, 
            blue
        }
    }

    /// Convert RGB to HSV
    pub fn to_hsv(&self) -> Hsv {
        let (r, g, b) = self.to_tuple();

        let mut cmax = r;
        let mut cmin = r;
        if g > cmax { // f32 does not implement Ord so if tree it is
            cmax = g;
        } else if g < cmin {
            cmin = g;
        }
        if b > cmax {
            cmax = b;
        } else if b < cmin {
            cmin = b;
        }
        let delta = cmax - cmin;



        let hue = if cmax == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if cmax == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let saturation = if cmax == 0.0 {
            0.0
        } else {
            delta / cmax
        };
        (hue, saturation, cmax)
    }

    /// Convert the color to a hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", (self.red * 255.0).round() as u32, (self.green * 255.0).round() as u32, (self.blue * 255.0).round() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_convert_hsv_rgb() {
        let colors = [
            (20.85, 0.51, 0.7051166), 
            (130.67574, 0.85, 0.51), 
            (7.302415, 0.85, 0.7659915), 
            (0.43018022, 0.11269033, 0.85)
        ];

        for (hue, saturation, value) in colors {
            let color_obj = Color::hsv_to_rgb(hue, saturation, value);
            let (hue2, saturation2, value2) = color_obj.to_hsv();
            assert_approx_eq!(f32, hue, hue2, epsilon = 0.00003);
            assert_approx_eq!(f32, saturation, saturation2, epsilon = 0.00003);
            assert_approx_eq!(f32, value, value2, epsilon = 0.00003);
        }
    }

    #[test]
    fn test_convert_hex() {
        let mapping = [
            ((0.0, 0.0, 1.0), "#FFFFFF"),
            ((0.0, 0.0, 0.0), "#000000"),
            ((0.0, 1.0, 1.0), "#FF0000"),
            ((0.482 * 360.0, 0.714, 0.878), "#40E0CF"),
            ((0.051 * 360.0, 0.718, 0.627), "#A0502D"),
        ];

        for ((hue, saturation, value), hex) in mapping {
            let color_obj = Color::hsv_to_rgb(hue, saturation, value);
            let hex2 = color_obj.to_hex();
            assert_eq!(hex, &hex2);
        }
    }
}
