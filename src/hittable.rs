use crate::V3;
use crate::{ray::Ray, Flt};
use std::ops::Range;

#[derive(Default)]
pub struct HitRecord {
  pub p: V3,
  pub norm: V3,
  pub t: Flt,
  pub front_face: bool,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, int: Range<Flt>) -> Option<HitRecord>;
}

impl HitRecord {
  pub fn set_face_normal(&mut self, ray: &Ray, out_norm: &V3) {
    self.front_face = ray.dir.dot(*out_norm) < 0.;
    self.norm = if self.front_face { *out_norm } else { -*out_norm };
  }
}
