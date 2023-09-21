use crate::Flt;
use glam::Vec3;

pub struct Ray {
  pub orig: Vec3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(orig: Vec3, dir: Vec3) -> Self {
    Self { orig, dir }
  }

  pub fn at(&self, t: Flt) -> Vec3 {
    self.orig + t * self.dir
  }

  pub fn colour(&self) -> Vec3 {
    let unit_dir = self.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.)
  }
}
