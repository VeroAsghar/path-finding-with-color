
use anyhow::Result;
use glob::glob;
use image::imageops::blur;
use std::fs::File;
use std::io::BufReader;
fn main() -> Result<()> {

    glob("frame0001*").expect("failed to read glob pattern").into_iter().for_each(|entry| {
        match entry {
            Ok(path) => process_frame(path).unwrap(),
            Err(_) => return,
        }
    });

    
    Ok(())
}

fn process_frame(path: std::path::PathBuf) -> Result<()> {
    let reader = BufReader::new(File::open(path)?);
    let mut frame = image::load(reader, image::ImageFormat::Png)?;
    blur(&frame, 1.0);
    let frame = frame.as_mut_rgb8().unwrap();
    for pixel in frame.pixels_mut() {
        let [r, g, b] = pixel.0;
        if r < 200 && g < 200 {
            pixel.0 = [0, 0, 0];
        }
    }
    frame.save("test.png")?;
    Ok(())
}
