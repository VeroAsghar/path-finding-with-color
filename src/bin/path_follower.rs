use anyhow::Result;
use clap::Parser;
use glob::glob;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;
use rayon::prelude::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator};
use std::fs::File;
use std::io::BufReader;
use color_tracking::{Processor, centroid};

#[derive(Parser)]
struct Args {
    input: String,
    output: String,

}

fn main() -> Result<()> {
    
    let args = Args::parse();


    let mut frames = Vec::with_capacity(250);

    for entry in glob(format!("{}/*.png", args.input).as_str())? {
        let reader = BufReader::new(File::open(entry?)?);
        let frame = image::load(reader, image::ImageFormat::Png)?;
        frames.push(frame);
    }
    let (w, h) = (640, 360);
    let mut output = ImageBuffer::new(w, h);
    let mut start = (0., 0.);
    let mut first = true;
    let proc = Processor::default();
    let centroids: Vec<(u32, u32)> = frames
        .into_par_iter()
        .enumerate()
        .map(|(i, frame)| {
            print!("{}\r", i); 
            proc.process_frame(w, h, frame)
        })
        .map(centroid)
        .collect();
    for (x, y) in centroids {
        if first {
            output.put_pixel(x, y, Rgb([0, 255u8, 0]));
            start = (x as f32, y as f32);
            first = false;
        } else {
            draw_line_segment_mut(&mut output, start, (x as f32, y as f32), Rgb([0, 255u8, 0]));
            start = (x as f32, y as f32);
        }
        //let bb = BoundingBox::from_midpoint(x, y, 50, 50);
        //bb.render(&mut output);
    }
    output.save(format!("{}.png", args.output).as_str())?;
    Ok(())
}


