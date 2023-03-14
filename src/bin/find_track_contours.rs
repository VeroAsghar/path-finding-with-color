use anyhow::Result;
use color_tracking::Processor;
use image::{ImageBuffer, Rgb, GenericImageView, Luma, DynamicImage};
use imageproc::contours::{find_contours, Contour, BorderType};
use imageproc::point::Point;

pub fn main() -> Result<()> {
    let mut frames = Vec::with_capacity(1);
    let frame = image::io::Reader::open("test_track_before.png")?
        .with_guessed_format()?
        .decode()?;
    frames.push(frame);
    let (w, h) = (400, 300);
    let mut output = ImageBuffer::new(w, h);
    let proc = Processor {
        blur: 1.0,
        rgb1: [150, 150, 0],
        rgb2: [250, 250, 100],
    };
    let (xs, ys) = proc.process_frame(w, h, frames.remove(0));
    for (x, y) in xs.clone().into_iter().zip(ys.clone()) {
        output.put_pixel(x, y, Luma::<u8>([255]));
    }
    let contours = find_contours::<u32>(&output);
    let mut output = DynamicImage::ImageLuma8(output).to_rgb8();
    for Contour { points, border_type, .. } in contours.into_iter() {
        if let BorderType::Outer = border_type {
            points.into_iter().for_each(|Point{ x, y }| {
                output.put_pixel(x, y, Rgb([255, 0, 0]));
            })
        }
    }
    output.save("test_track_contours_after.png")?;
    Ok(())
}
