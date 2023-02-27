
use anyhow::Result;
use glob::glob;
use image::{DynamicImage, ImageBuffer, Rgb, Pixel, GenericImageView};
use image::imageops::blur;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
fn main() -> Result<()> {

    let mut frames = Vec::with_capacity(250);

    for entry in glob("photos/frame*")? {
        let reader = BufReader::new(File::open(entry?)?);
        let frame = image::load(reader, image::ImageFormat::Png)?;
        frames.push(frame);
    }
    let (w, h) = frames[0].dimensions();
    let mut output = ImageBuffer::new(w, h);
    for frame in frames {
        let (x, y) = process_frame(frame)?;
        //output.put_pixel(x, y, Rgb([255u8,0,0]));
        let bb = BoundingBox::from_midpoint(x, y, 50, 50);
        for i in bb.x1..=bb.x2 {
            output.put_pixel(i, bb.y1, Rgb([255u8,0,0]));
            output.put_pixel(i, bb.y2, Rgb([255u8,0,0]));
        }
        for i in bb.y1..=bb.y2 {
            output.put_pixel(bb.x1, i, Rgb([255u8,0,0]));
            output.put_pixel(bb.x2, i, Rgb([255u8,0,0]));
        }
    }
    output.save("test.png")?;

    
    Ok(())
}


fn process_frame(mut frame: DynamicImage) -> Result<(u32, u32)> {
    blur(&frame, 1.0);
    let frame = frame.as_mut_rgb8().unwrap();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y, pixel) in frame.enumerate_pixels() {
        let [r, g, _] = pixel.0;
        if r > 200 && g > 200 {
            xs.push(x);
            ys.push(y);
        }
    }
    Ok(centroid(xs, ys))
}


fn centroid(x: Vec<u32>, y: Vec<u32>) -> (u32, u32) {
    let xlen = x.len() as u32;
    let ylen = y.len() as u32;
    (x.into_iter().sum::<u32>()/xlen, y.into_iter().sum::<u32>()/ylen)
}

struct BoundingBox {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32, 
    pub y2: u32,
}

impl BoundingBox {
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self { Self { x1, y1, x2, y2 } }
    fn from_midpoint(x: u32, y: u32, w: u32, h: u32) -> Self { 
        Self { x1: x - w/2, y1: y - h/2, x2: x + w/2, y2: y + h/2 }
    }


}
