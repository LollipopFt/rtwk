use colour::Colour;
use glam::Vec3 as V3;
use itertools::Itertools;
use ray::Ray;
use std::io::Write;

mod colour;
mod hittable;
mod ray;
mod sphere;
type Flt = f32;

const A_R: Flt = 16. / 9.;
const IMG_W: u32 = 400;
const IMG_H: u32 = {
  let imgh = (IMG_W as Flt / A_R) as u32;
  if imgh < 1 {
    1
  } else {
    imgh
  }
};

const FOC_L: Flt = 1.;
const VWPT_H: Flt = 2.;
const VWPT_W: Flt = VWPT_H * IMG_W as Flt / IMG_H as Flt;
const CAM_CTR: V3 = V3::ZERO;

const VWPT_U: V3 = V3::new(VWPT_W, 0., 0.);
const VWPT_V: V3 = V3::new(0., -VWPT_H, 0.);

fn hit_sphere(ctr: &V3, rad: Flt, ray: &Ray) -> Flt {
  //    x² + y² + z² = r²
  // => (x - Cx)² + (y - Cy)² + (z - Cz)² = (P - C) ⋅ (P - C)
  // => (P(t) - C) ⋅ (P(t) - C) = r²    P(t) = A + tB
  // => (tB + A - C) ⋅ (tB + A - C) = r²
  // => t²B⋅B + 2tB ⋅ (A - C) + (A - C) ⋅ (A - C) - r² = 0
  // => ∴ a = B ⋅ B; b = 2B ⋅ (A - C); c = (A - C) ⋅ (A - C) - r²
  let oc = ray.orig - *ctr;
  let a = ray.dir.length_squared();
  let half_b = oc.dot(ray.dir);
  let c = oc.length_squared() - rad * rad;
  let discrm = half_b.powi(2) - a * c;
  if discrm < 0. {
    -1.
  } else {
    (-half_b - discrm.sqrt()) / a
  }
}

fn main() {
  let mut handle = std::io::BufWriter::new(std::io::stderr());
  let mut img = image::RgbImage::new(IMG_W, IMG_H);

  let px_delta_u: V3 = VWPT_U / IMG_W as Flt;
  let px_delta_v: V3 = VWPT_V / IMG_H as Flt;

  let vwpt_up_l: V3 =
    CAM_CTR - V3::new(0., 0., FOC_L) - VWPT_U / 2. - VWPT_V / 2.;
  let px_loc: V3 = vwpt_up_l + 0.5 * (px_delta_u + px_delta_v);

  (0..IMG_H).cartesian_product(0..IMG_W).for_each(|(y, x)| {
    write!(handle, "\x1b[1K\rscanlines remaining: {}", IMG_H - y).unwrap();
    let px_ctr = px_loc + x as Flt * px_delta_u + y as Flt * px_delta_v;
    let ray_dir = px_ctr - CAM_CTR;
    let ray = Ray::new(CAM_CTR, ray_dir);
    img.put_pixel(x, y, image::Rgb(ray.colour().colour()));
  });
  write!(handle, "\x1b[1K\rdone.").unwrap();

  img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
