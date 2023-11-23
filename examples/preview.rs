
use piston_window::*;
use colourado_iter::{PaletteType, ColorPalette};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		panic!("You must supply exactly 3 parameters. The palette type, the type of spread and the number of colors to generate!");
	}

	let palette_type = match args[1].to_lowercase().as_ref() {
		"pastel" => PaletteType::Pastel,
		"dark" => PaletteType::Dark,
		_ => PaletteType::Random
	};

	let num_colors = args[2].parse().unwrap_or(4);
	
	let adjacent = matches!(args[3].to_lowercase().as_ref(), "adjacent");

	let palette = ColorPalette::new(palette_type, adjacent, &mut rand::thread_rng());

	let colors: Vec<_> = palette.take(num_colors).collect();

    let mut window: PistonWindow = WindowSettings::new("Color palette preview", [1280, 720]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics, _| {
			clear([1.0; 4], graphics);

			for (i, color) in colors.iter().enumerate() {
				rectangle(color.to_rgba_array(), 
					  [(120.0 * i as f64) % 1200.0, ((120.0 * i as f64) / 1200.0).floor() * 120.0, 100.0, 100.0],
                      context.transform,
                      graphics);
			}
        });
    }
}