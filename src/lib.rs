use image::imageops::FilterType;
use image::imageops::blur;
use image::DynamicImage;
use image::Rgb;
use image::ImageBuffer;
use image::imageops::resize;



pub fn filter_color(
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

pub struct Processor {
    pub blur: f32,
    pub rgb1: [u8; 3],
    pub rgb2: [u8; 3],
}

impl Default for Processor {
    fn default() -> Self {
        let blur = 1.0;
        let rgb1 = [0, 100, 0];
        let rgb2 = [10, 200, 150];
        Self { blur, rgb1, rgb2 }
    }
}

impl Processor {
    pub fn process_frame(&self, width: u32, height: u32, frame: DynamicImage) -> (Vec<u32>, Vec<u32>) {
        let frame = resize(&frame, width, height, FilterType::Gaussian);
        blur(&frame, self.blur);
        let frame = DynamicImage::ImageRgba8(frame).to_rgb8();
        let xs = Vec::new();
        let ys = Vec::new();
        filter_color(&frame, xs, ys, self.rgb1, self.rgb2)
    }
}

pub fn centroid((x, y): (Vec<u32>, Vec<u32>)) -> (u32, u32) {
    let xlen = x.len() as u32;
    let ylen = y.len() as u32;
    (
        x.into_iter().sum::<u32>() / xlen,
        y.into_iter().sum::<u32>() / ylen,
    )
}

pub struct BoundingBox {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl BoundingBox {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self { x1, y1, x2, y2 }
    }
    pub fn from_midpoint(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            x1: x - w / 2,
            y1: y - h / 2,
            x2: x + w / 2,
            y2: y + h / 2,
        }
    }
    pub fn render(&self, output: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
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
