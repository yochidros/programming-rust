extern crate num;
extern crate image;
extern crate crossbeam;

mod images;
mod render;
mod parse;
mod mandle;

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: mandlebrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse::parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse::parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse::parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;

    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = render::pixel_to_point(bounds, (0,top), upper_left, lower_right);
                let band_lower_right = render::pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {
                    println!("u");
                    render::render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
    println!("\nexporting....");

    images::write_image(&args[1], &pixels, bounds).expect("error writing png file");
}
