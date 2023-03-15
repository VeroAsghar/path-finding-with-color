use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use color_tracking::{Processor, BoundingBox, centroid};
use image::{ImageBuffer, Rgb};

pub fn main() -> Result<()> {
    let mut frames = Vec::with_capacity(1);
    let reader = BufReader::new(File::open("calibrate.png")?);
    let frame = image::load(reader, image::ImageFormat::Png)?;
    frames.push(frame);
    let (w, h) = (540, 960);
    let mut output = ImageBuffer::new(w, h);
    let proc = Processor::default();
    let (xs, ys) = proc.process_frame(w, h, frames.remove(0));
    for (x, y) in xs.clone().into_iter().zip(ys.clone()) {
        output.put_pixel(x, y, Rgb([255u8, 255, 255]));
    }
    let (x, y) = centroid((xs, ys));
    output.put_pixel(x, y, Rgb([255u8, 0, 0]));
    let bb = BoundingBox::from_midpoint(x, y, 50, 50);
    bb.render(&mut output);
    println!("meow");
    output.save("calibrated.png")?;
    Ok(())
}
