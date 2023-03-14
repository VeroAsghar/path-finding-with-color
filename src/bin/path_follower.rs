use anyhow::Result;
use glob::glob;
use image::imageops::blur;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use color_tracking::{Processor, centroid, BoundingBox};

fn main() -> Result<()> {
    let mut frames = Vec::with_capacity(250);

    for entry in glob("photos/frame*")? {
        let reader = BufReader::new(File::open(entry?)?);
        let frame = image::load(reader, image::ImageFormat::Png)?;
        frames.push(frame);
    }
    let (w, h) = frames[0].dimensions();
    let mut output = ImageBuffer::new(w, h);
    let mut start = (0., 0.);
    let mut first = true;
    let proc = Processor::default();
    let centroids: Vec<(u32, u32)> = frames
        .into_par_iter()
        .map(|frame| proc.process_frame(frame))
        .map(centroid)
        .collect();
    for (x, y) in centroids {
        if first {
            output.put_pixel(x, y, Rgb([255u8, 0, 0]));
            start = (x as f32, y as f32);
            first = false;
        } else {
            draw_line_segment_mut(&mut output, start, (x as f32, y as f32), Rgb([0, 255u8, 0]));
            start = (x as f32, y as f32);
        }
        let bb = BoundingBox::from_midpoint(x, y, 50, 50);
        bb.render(&mut output);
    }
    output.save("test.png")?;
    Ok(())
}


