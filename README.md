# Colourado-Iter

This is a fork of [Colourado](https://github.com/BrandtM/colourado).

A small and minimalistic library to generate a random color palette.  
The user-facing `Color` struct contains RGB colors ranging from 0 to 1.  
All colors are of type f32 (no exceptions).

This fork implements the `Iterator` trait for the `ColorPalette` struct, allowing it to continuously create new colors every time `next` is called. 
Furthermore, it allows you to supply your own rng to determine the initial hue.

Additionally, the `Color` struct has been extended to allow more representations (such as converting back to HSV, or returning a hex color representation).

# Usage

```rust
use colourado::{Color, ColorPalette, PaletteType};

let palette = ColorPalette::new(PaletteType::Random, false, &mut rand::thread_rng());
let random_color = palette.next();
let color_array: [f32; 3] = palette.next().to_array();
let hue = 315.0;
let saturation = 0.5;
let value = 0.3;
let rgb_color: Color = Color::hsv_to_rgb(hue, saturation, value);
```

Optionally, you can use the `HsvPalette` struct to get a generator which spits out the immediate HSV values as opposed to a `Color` struct.

## Example  

A color palette might look like this when rendered:  

![Example image](https://raw.githubusercontent.com/BrandtM/colourado/master/examples/example.png)  

Test the color palettes for yourself by running  
`cargo run --example preview TYPE NUM adjacent|spread`  
`TYPE` can be one of *random*, *pastel*, or *dark*
`NUM` is the amount of colors to generate and display
`adjacent` or `spread` determine whether the colors are generated close to each other or spread apart.