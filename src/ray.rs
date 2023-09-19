use crate::Flt;
use glam::Vec3;

pub struct Ray {
  pub orig: Vec3,
  pub dir: Vec3,
}

impl Ray {
  fn at(&self, t: Flt) -> Vec3 {
    self.orig + t * self.dir
  }
}
