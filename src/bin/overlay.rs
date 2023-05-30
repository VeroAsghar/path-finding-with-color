use clap::Parser;
use image::Rgb;


#[derive(Parser)]
struct Args {
    first: String,
    second: String,
}

fn main() {
    let args = Args::parse();


    let base = image::open("track_contours.png").unwrap();
    let first = image::open(args.first.as_str()).unwrap();
    let second = image::open(args.second.as_str()).unwrap();
    let mut base = base.into_rgb8();
    for (x, y, rgb) in first.into_rgb8().enumerate_pixels() {
       let [r, g, b] = rgb.0;
        if g > 0 {
            base.put_pixel(x, y, Rgb([255u8, 0, 255]));
        }
    }
    for (x, y, rgb) in second.into_rgb8().enumerate_pixels() {
       let [r, g, b] = rgb.0;
        if g > 0 {
            base.put_pixel(x, y, Rgb([0, 255, 0]));
        }
    }
    base.save(format!("better{}", args.first.split("_").last().unwrap()).as_str()).unwrap();
}
