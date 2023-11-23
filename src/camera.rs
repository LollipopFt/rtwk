use crate::colour::Colour as _;
use crate::Ray;
use crate::{hittable::Hittable, Flt, V3};
use itertools::Itertools;
use std::io::Write;

pub struct Camera {
  img_w: u32,
  img_h: u32,
  ctr: V3,
  px_loc: V3,
  px_delta_u: V3,
  px_delta_v: V3,
}

impl Camera {
  pub fn init(img_w: u32, a_r: Flt) -> Self {
    let img_h: u32 = {
      let imgh = (img_w as Flt / a_r) as u32;
      if imgh < 1 {
        1
      } else {
        imgh
      }
    };

    let ctr = V3::ZERO;

    let foc_l = 1.;
    let vwpt_h = 2.;
    let vwpt_w = vwpt_h * img_w as Flt / img_h as Flt;

    let vwpt_u = V3::new(vwpt_w, 0., 0.);
    let vwpt_v = V3::new(0., -vwpt_h, 0.);

    let px_delta_u = vwpt_u / img_w as Flt;
    let px_delta_v = vwpt_v / img_h as Flt;
    let vwpt_up_l = ctr - V3::new(0., 0., foc_l) - vwpt_u / 2. - vwpt_v / 2.;
    let px_loc = vwpt_up_l + 0.5 * (px_delta_u + px_delta_v);
    Self { img_w, img_h, ctr, px_loc, px_delta_u, px_delta_v }
  }

  pub fn render(&self, world: impl Hittable) {
    let mut handle = std::io::BufWriter::new(std::io::stderr());
    let mut img = image::RgbImage::new(self.img_w, self.img_h);

    (0..self.img_h).cartesian_product(0..self.img_w).for_each(|(y, x)| {
      write!(handle, "\x1b[1K\rscanlines remaining: {}", self.img_h - y)
        .unwrap();
      let px_ctr =
        self.px_loc + x as Flt * self.px_delta_u + y as Flt * self.px_delta_v;
      let ray_dir = px_ctr - self.ctr;
      let ray = Ray::new(self.ctr, ray_dir);
      img.put_pixel(x, y, image::Rgb(ray.colour(&world).colour()));
    });
    write!(handle, "\x1b[1K\rdone.").unwrap();

    img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
  }
}
