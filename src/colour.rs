use glam::Vec3;

pub trait Colour {
  fn colour(&self) -> [u8; 3];
}

impl Colour for Vec3 {
  fn colour(&self) -> [u8; 3] {
    [
      (self.x * 255.999) as u8,
      (self.y * 255.999) as u8,
      (self.z * 255.999) as u8,
    ]
  }
}
