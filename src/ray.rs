use crate::hittable::Hittable;
use crate::Flt;
use crate::V3;

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

  pub fn colour<T>(&self, world: &T) -> V3
  where
    T: Hittable,
  {
    if let Some(rec) = world.hit(self, 0.001..Flt::INFINITY) {
      return 0.5 * (rec.norm + V3::ONE);
    }
    let unit_dir = self.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.);
    (1. - a) * V3::ONE + a * V3::new(0.5, 0.7, 1.)
  }
}
