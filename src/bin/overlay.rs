use image::Rgb;

fn main() {
    let base = image::open("track_after.png").unwrap();
    let second = image::open("car_path.png").unwrap();
    let mut base = base.into_rgb8();
    for (x, y, rgb) in second.into_rgb8().enumerate_pixels() {
       let [r, g, b] = rgb.0;
        if g > 0 {
            base.put_pixel(x, y, Rgb([0, 255, 0]));
        }
    }
    base.save("merged.png").unwrap();
}
