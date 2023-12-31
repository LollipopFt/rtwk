use crate::V3;
use crate::{ray::Ray, Flt};
use std::ops::Range;

#[derive(Default)]
pub struct HitRecord {
  // line = p + norm * t
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

impl<T> Hittable for Vec<T>
where
  T: Hittable,
{
  fn hit(&self, ray: &Ray, int: Range<Flt>) -> Option<HitRecord> {
    let (_closest, hit_record) =
      self.iter().fold((int.end, None), |acc, record| {
        if let Some(temp_rec) = record.hit(ray, int.start..acc.0) {
          (temp_rec.t, Some(temp_rec))
        } else {
          acc
        }
      });
    hit_record
  }
}
