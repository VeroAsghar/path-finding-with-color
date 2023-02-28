use anyhow::Result;
use glob::glob;
use image::imageops::blur;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;

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
    let centroids: Vec<(u32, u32)> = frames
        .into_par_iter()
        .map(process_frame)
        .map(centroid)
        .collect();
    for (x, y) in centroids {
        if first {
            output.put_pixel(x, y, Rgb([255u8, 0, 0]));
            start = (x as f32, y as f32);
            first = false;
        } else {
            draw_line_segment_mut(
                &mut output,
                start,
                (x as f32, y as f32),
                Rgb([255u8, 255u8, 0]),
            );
            start = (x as f32, y as f32);
        }
        let bb = BoundingBox::from_midpoint(x, y, 50, 50);
        bb.render(&mut output);
    }
    output.save("test.png")?;
    Ok(())
}
fn filter_color(
    frame: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    mut xs: Vec<u32>,
    mut ys: Vec<u32>,
    [r1, g1, b1]: [u8; 3],
    [r2, g2, b2]: [u8; 3],
) -> (Vec<u32>, Vec<u32>) {
    for (x, y, pixel) in frame.enumerate_pixels() {
        let [rp, gp, bp] = pixel.0;
        if rp >= r1 && gp >= g1 && bp >= b1 {
            if rp < r2 && gp < g2 && bp < b2 {
                xs.push(x);
                ys.push(y);
            }
        }
    }
    (xs, ys)
}

fn process_frame(frame: DynamicImage) -> (Vec<u32>, Vec<u32>) {
    blur(&frame, 1.0);
    let frame = frame.as_rgb8().unwrap();
    let xs = Vec::new();
    let ys = Vec::new();
    let rgb1 = [0, 100, 0];
    let rgb2 = [100, 250, 100];
    filter_color(&frame, xs, ys, rgb1, rgb2)
}

fn centroid((x, y): (Vec<u32>, Vec<u32>)) -> (u32, u32) {
    let xlen = x.len() as u32;
    let ylen = y.len() as u32;
    (
        x.into_iter().sum::<u32>() / xlen,
        y.into_iter().sum::<u32>() / ylen,
    )
}

struct BoundingBox {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl BoundingBox {
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self { x1, y1, x2, y2 }
    }
    fn from_midpoint(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            x1: x - w / 2,
            y1: y - h / 2,
            x2: x + w / 2,
            y2: y + h / 2,
        }
    }
    fn render(&self, output: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        for i in self.x1..=self.x2 {
            output.put_pixel(i, self.y1, Rgb([255u8, 0, 0]));
            output.put_pixel(i, self.y2, Rgb([255u8, 0, 0]));
        }
        for i in self.y1..=self.y2 {
            output.put_pixel(self.x1, i, Rgb([255u8, 0, 0]));
            output.put_pixel(self.x2, i, Rgb([255u8, 0, 0]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_color() -> Result<()> {
        let mut frames = Vec::with_capacity(1);
        let reader = BufReader::new(File::open("test_green_before.png")?);
        let frame = image::load(reader, image::ImageFormat::Png)?;
        frames.push(frame);
        let (w, h) = frames[0].dimensions();
        let mut output = ImageBuffer::new(w, h);
        let (xs, ys) = process_frame(frames.remove(0));
        for (x, y) in xs.clone().into_iter().zip(ys.clone()) {
            output.put_pixel(x, y, Rgb([255u8, 255, 255]));
        }
        let (x, y) = centroid((xs, ys));
        output.put_pixel(x, y, Rgb([255u8, 0, 0]));
        let bb = BoundingBox::from_midpoint(x, y, 50, 50);
        bb.render(&mut output);
        output.save("test_green_after.png")?;
        Ok(())
    }
}
