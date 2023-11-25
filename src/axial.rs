use std::{cmp::Ordering, ops::Add};

use bevy_math::{Vec2, Vec3};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Axial {
    q: i32,
    r: i32,
}

pub static SCALE_X: f32 = 60.;
pub static SCALE_Y: f32 = 60.;

impl Axial {
    pub fn new(q: i32, r: i32) -> Self {
        Axial { q, r }
    }

    pub fn to_cartesian(&self) -> Vec3 {
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );

        // Line 1
        let a2: Vec2 = l + Vec2::new(-l.y, l.x) * self.q as f32;

        let m = f32::tan(std::f32::consts::PI / 6.0);
        let y = m * (self.r as f32 - a2.x) + a2.y;

        // scale is the size of the hex from the center to the edge
        Vec3::new(self.r as f32 * SCALE_X, y * SCALE_Y, 0.)
    }

    fn from_cartesian(world_pos: Vec3) -> Self {
        let result = Vec2::new(world_pos.x / SCALE_X, world_pos.z / SCALE_Y);
        let m = f32::tan(std::f32::consts::PI / 6.0);
        let l = Vec2::new(
            f32::cos(std::f32::consts::PI / 6.0),
            f32::sin(std::f32::consts::PI / 6.0),
        );
        let cx = (result.y - l.y - m * result.x + m * l.x) / (-l.y * m - l.x * m);
        Axial::new(cx as i32, result.x as i32)
    }
}
impl Add<Axial> for Axial {
    type Output = Axial;

    fn add(self, other: Axial) -> Axial {
        Axial::new(self.q + other.q, self.r + other.r)
    }
}
impl From<Vec3> for Axial {
    fn from(world_pos: Vec3) -> Self {
        Axial::from_cartesian(world_pos)
    }
}

#[test]
fn test_convert_cartesian() {
    let result = Axial::new(0, 0).to_cartesian();
    assert_eq!(result, Vec3::ZERO)
}
#[test]
fn test_convert_axial() {
    let result = Axial::from(Vec3::ZERO);
    assert_eq!(result, Axial::new(0, 0))
}
