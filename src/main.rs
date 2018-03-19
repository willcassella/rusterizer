#[macro_use]
mod strider;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero(
    ) -> Self {
        Vec3::new(0_f32, 0_f32, 0_f32)
    }
}

fn main() {
    let array = [Vec3::new(0_f32, 1_f32, 3_f32), Vec3::new(4_f32, 5_f32, 6_f32), Vec3::new(7_f32, 8_f32, 9_f32)];
    let stride = strider!(array, x);

    println!("Hello, world!");
}
