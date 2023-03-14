
use anyhow::Result;
use color_tracking::Processor;
use image::{ImageBuffer, Rgb, GenericImageView};

pub fn main() -> Result<()> {
    let mut frames = Vec::with_capacity(1);
    let frame = image::io::Reader::open("test_track_before.png")?
        .with_guessed_format()?
        .decode()?;
    frames.push(frame);
    let (w, h) = frames[0].dimensions();
    let mut output = ImageBuffer::new(w, h);
    let proc = Processor {
        blur: 1.0,
        rgb1: [150, 150, 0],
        rgb2: [250, 250, 100],
    };
    let (xs, ys) = proc.process_frame(frames.remove(0));
    for (x, y) in xs.clone().into_iter().zip(ys.clone()) {
        output.put_pixel(x, y, Rgb([255u8, 255, 0]));
    }
    output.save("test_track_after.png")?;
    Ok(())
}
