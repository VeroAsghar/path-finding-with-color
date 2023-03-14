use crate::*;
use anyhow::Result;
    pub fn main() -> Result<()> {
        let mut frames = Vec::with_capacity(1);
        let reader = BufReader::new(File::open("test_green_before.png")?);
        let frame = image::load(reader, image::ImageFormat::Png)?;
        frames.push(frame);
        let (w, h) = frames[0].dimensions();
        let mut output = ImageBuffer::new(w, h);
        let proc = Processor::default();
        let (xs, ys) = proc.process_frame(frames.remove(0));
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
