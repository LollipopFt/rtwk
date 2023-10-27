use std::ops::Range;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::Flt;
use crate::V3;

pub struct Sphere {
  ctr: V3,
  rad: Flt,
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, int: Range<Flt>) -> Option<HitRecord> {
    //    x² + y² + z² = r²
    // => (x - Cx)² + (y - Cy)² + (z - Cz)² = (P - C) ⋅ (P - C)
    // => (P(t) - C) ⋅ (P(t) - C) = r²    P(t) = A + tB
    // => (tB + A - C) ⋅ (tB + A - C) = r²
    // => t²B⋅B + 2tB ⋅ (A - C) + (A - C) ⋅ (A - C) - r² = 0
    // => ∴ a = B ⋅ B; b = 2B ⋅ (A - C); c = (A - C) ⋅ (A - C) - r²
    let oc = ray.orig - self.ctr;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - self.rad.powi(2);

    let discrm = half_b.powi(2) - a * c;
    if discrm < 0. {
      return None;
    }
    let sqrtd = discrm.sqrt();

    // find the nearest root that lies in the acceptable range
    let mut root = (-half_b - sqrtd) / a;
    if !int.contains(&root) {
      root = (-half_b + sqrtd) / a;
      if !int.contains(&root) {
        return None;
      }
    }

    let t = root;
    let p = ray.at(t);
    let norm = (p - self.ctr) / self.rad;
    Some(HitRecord { p, norm, t })
  }
}
