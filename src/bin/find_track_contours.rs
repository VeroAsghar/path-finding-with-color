use anyhow::Result;
use color_tracking::Processor;
use image::{ImageBuffer, Rgb, GenericImageView, Luma, DynamicImage, RgbImage};
use imageproc::contours::{find_contours, Contour, BorderType};
use imageproc::point::Point;

pub fn main() -> Result<()> {
    let frame = image::io::Reader::open("track_after.png")?
        .with_guessed_format()?
        .decode()?;
    let contours = find_contours::<u32>(&frame.to_luma8());
    let (w, h) = frame.dimensions();
    let mut output = RgbImage::new(w, h);
    let Contour { mut points, border_type, .. } = contours.into_iter().next().unwrap(); 
    if let BorderType::Outer = border_type {
        points.sort_by(|a, b| a.x.cmp(&b.x)); 
        for (i, &Point{ x: x0, y: y0 }) in points.iter().enumerate() {
            for &Point{ x: x1, y: y1 } in points[i+1..].iter() {
                if x0 == x1 {
                    let y = (y1 + y0)/2;
                    output.put_pixel(x0, y, Rgb([255, 255, 0]));
                }

            }
        }
    }
    output.save("track_contours_after.png")?;
    Ok(())
}
