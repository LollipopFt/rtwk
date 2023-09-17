use itertools::Itertools;
use std::io::Write;

type Flt = f32;

const IMG_W: u32 = 256;
const IMG_H: u32 = 256;

fn main() {
  let mut handle = std::io::BufWriter::new(std::io::stderr());
  let mut img = image::RgbImage::new(IMG_W, IMG_H);

  let ib: u8 = 0;
  (0..IMG_H).cartesian_product(0..IMG_W).for_each(|(y, x)| {
    write!(handle, "\x1b[1K\rscanlines remaining: {}", IMG_H - y).unwrap();
    let r = x as Flt / (IMG_W - 1) as Flt;
    let g = y as Flt / (IMG_H - 1) as Flt;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;

    img.put_pixel(x, y, image::Rgb([ir, ig, ib]));
  });
  write!(handle, "\x1b[1K\rdone.").unwrap();

  img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
