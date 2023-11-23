use camera::Camera;
use glam::Vec3 as V3;
use ray::Ray;

mod camera;
mod colour;
mod hittable;
mod ray;
mod sphere;
type Flt = f32;

const A_R: Flt = 16. / 9.;
const IMG_W: u32 = 400;

fn main() {
  let cam = Camera::init(IMG_W, A_R);

  let world = vec![
    sphere::Sphere::new(V3::NEG_Z, 0.5),
    sphere::Sphere::new(V3::new(0., -100.5, -1.), 100.),
  ];

  cam.render(world);
}
