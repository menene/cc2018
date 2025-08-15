use raylib::prelude::{Color, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersect {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vector3, normal: Vector3, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zero(),
            normal: Vector3::zero(),
            distance: 0.0,
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0, 0),
            },
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}