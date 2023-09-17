use itertools::Itertools;

type Flt = f32;

const IMG_W: u32 = 256;
const IMG_H: u32 = 256;

fn main() {
  let mut img = image::RgbImage::new(IMG_W, IMG_H);

  let ib: u8 = 0;
  (0..IMG_H).cartesian_product(0..IMG_W).for_each(|(y, x)| {
    let r = x as Flt / (IMG_W - 1) as Flt;
    let g = y as Flt / (IMG_H - 1) as Flt;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;

    img.put_pixel(x, y, image::Rgb([ir, ig, ib]));
  });

  img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
