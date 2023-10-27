use crate::V3;
use crate::{hit_sphere, Flt};

pub struct Ray {
  pub orig: V3,
  pub dir: V3,
}

impl Ray {
  pub fn new(orig: V3, dir: V3) -> Self {
    Self { orig, dir }
  }

  pub fn at(&self, t: Flt) -> V3 {
    self.orig + t * self.dir
  }

  pub fn colour(&self) -> V3 {
    let t = hit_sphere(&V3::NEG_Z, 0.5, self);
    if t > 0. {
      let n = (self.at(t) - V3::NEG_Z).normalize();
      return 0.5 * (n + V3::ONE);
    }
    let unit_dir = self.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * V3::ONE + a * V3::new(0.5, 0.7, 1.)
  }
}
