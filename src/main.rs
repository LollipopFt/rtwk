use colour::Colour;
use glam::Vec3;
use itertools::Itertools;
use std::io::Write;

mod colour;
type Flt = f32;

const IMG_W: u32 = 256;
const IMG_H: u32 = 256;

fn main() {
  let mut handle = std::io::BufWriter::new(std::io::stderr());
  let mut img = image::RgbImage::new(IMG_W, IMG_H);

  (0..IMG_H).cartesian_product(0..IMG_W).for_each(|(y, x)| {
    write!(handle, "\x1b[1K\rscanlines remaining: {}", IMG_H - y).unwrap();
    let px_col = Vec3::new(
      x as Flt / (IMG_W - 1) as Flt,
      y as Flt / (IMG_H - 1) as Flt,
      0.,
    );

    img.put_pixel(x, y, image::Rgb(px_col.colour()));
  });
  write!(handle, "\x1b[1K\rdone.").unwrap();

  img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
