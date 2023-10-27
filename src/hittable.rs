use crate::V3;
use crate::{ray::Ray, Flt};
use std::ops::Range;

pub struct HitRecord {
  pub p: V3,
  pub norm: V3,
  pub t: Flt,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, int: Range<Flt>) -> Option<HitRecord>;
}
