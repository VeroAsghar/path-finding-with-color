
use anyhow::Result;
use color_tracking::Processor;
use image::{ImageBuffer, Rgb, GenericImageView};

pub fn main() -> Result<()> {
    let mut frames = Vec::with_capacity(1);
    let frame = image::io::Reader::open("track.png")?
        .with_guessed_format()?
        .decode()?;
    frames.push(frame);
    let (w, h) = (540, 960);
    let mut output = ImageBuffer::new(w, h);
    let proc = Processor {
        blur: 1.0,
        rgb1: [120, 120, 0],
        rgb2: [255, 255, 130],
    };
    let (xs, ys) = proc.process_frame(w, h, frames.remove(0));
    for (x, y) in xs.clone().into_iter().zip(ys.clone()) {
        output.put_pixel(x, y, Rgb([255u8, 255, 0]));
    }
    output.save("track_after.png")?;
    Ok(())
}
