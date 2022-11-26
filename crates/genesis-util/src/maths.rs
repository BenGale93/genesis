use std::{f32::consts::PI, iter::Sum, ops::Div};

use glam::{Quat, Vec3};
use nalgebra::wrap;

use crate::util_error::GenesisUtilError;

pub fn mean<T>(numbers: Vec<T>) -> f32
where
    T: Copy + Sum + Div<f32, Output = f32>,
{
    let len = numbers.len();
    let sum: T = numbers.into_iter().sum();

    sum / len as f32
}

pub fn polars_to_cart(r: f32, theta: f32) -> (f32, f32) {
    let (y_ang, x_ang) = theta.sin_cos();
    (r * x_ang, r * y_ang)
}

pub fn angle_to_point(diff: Vec3) -> f32 {
    diff.y.atan2(diff.x)
}

pub fn rebased_angle(angle_from_x: f32, angle_from_y: f32) -> f32 {
    (angle_from_x - (PI / 2.0) - angle_from_y).abs()
}

pub fn closest_object(distances: &[f32]) -> Option<usize> {
    distances
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("There should be no NaNs."))
        .map(|(index, _)| index)
}

pub fn linear_interpolate(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
    (y_min * (x_max - x) + y_max * (x - x_min)) / (x_max - x_min)
}

#[derive(Debug)]
pub struct Cone {
    point: Vec3,
    rotation: Quat,
    fov_angle: f32,
    fov_length: f32,
}

impl Cone {
    pub fn new(
        point: Vec3,
        rotation: Quat,
        angle: f32,
        length: f32,
    ) -> Result<Self, GenesisUtilError> {
        if length <= 0.0 {
            return Err(GenesisUtilError::LengthError);
        }

        if angle <= 0.0 || angle > f32::to_radians(360.0) {
            return Err(GenesisUtilError::AngleError);
        }

        Ok(Self {
            point,
            rotation,
            fov_angle: angle,
            fov_length: length,
        })
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn angle(&self) -> f32 {
        self.fov_angle
    }

    pub fn length(&self) -> f32 {
        self.fov_length
    }

    pub fn is_within_cone(&self, target: Vec3) -> bool {
        let distance = target - self.point;

        if distance.length() > self.fov_length {
            return false;
        }
        let x_angle = angle_to_point(distance);

        let angle = rebased_angle(x_angle, self.rotation.z.asin() * 2.0);

        let angle = wrap(angle, -PI, PI);

        if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use glam::{Quat, Vec3};

    use super::{angle_to_point, closest_object, Cone};

    #[test]
    fn angle_between_from_origin() {
        let me = Vec3::new(0.0, 0.0, 0.0);
        let them = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(angle_to_point(them - me), 0.9272952);
    }

    #[test]
    fn angle_between_same_x() {
        let me = Vec3::new(1.0, 0.0, 0.0);
        let them = Vec3::new(1.0, 4.0, 0.0);

        assert_eq!(angle_to_point(them - me), 1.5707964);
    }

    #[test]
    fn angle_between_same_y() {
        let me = Vec3::new(1.0, 4.0, 0.0);
        let them = Vec3::new(5.0, 4.0, 0.0);

        assert_eq!(angle_to_point(them - me), 0.0);
    }

    #[test]
    fn is_within_cone_true() {
        let me = Vec3::ZERO;
        let rotation = Quat::IDENTITY;

        let cone = Cone::new(me, rotation, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(3.0, 3.0, 0.0);

        assert!(cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_to_far_away() {
        let me = Vec3::ZERO;
        let rotation = Quat::IDENTITY;

        let cone = Cone::new(me, rotation, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(8.0, 8.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_behind_me() {
        let me = Vec3::ZERO;
        let rotation = Quat::IDENTITY;

        let cone = Cone::new(me, rotation, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(-1.0, -1.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_visible_rotated_cone() {
        let me = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(f32::to_radians(-90.0));

        let cone = Cone::new(me, rotation, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(3.0, -3.0, 0.0);

        assert!(cone.is_within_cone(target));
    }
    #[test]
    fn is_not_visible_rotated_cone() {
        let me = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(f32::to_radians(90.0));

        let cone = Cone::new(me, rotation, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(3.0, -3.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_not_visible_rotated_cone_smaller_fov() {
        let me = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(f32::to_radians(-90.0));

        let cone = Cone::new(me, rotation, f32::to_radians(89.0), 10.0).unwrap();

        let target = Vec3::new(3.0, 3.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_visible_rotated_cone_smaller_fov() {
        let me = Vec3::ZERO;
        let rotation = Quat::from_rotation_z(f32::to_radians(-90.0));

        let cone = Cone::new(me, rotation, f32::to_radians(100.0), 10.0).unwrap();

        let target = Vec3::new(3.0, 3.0, 0.0);

        assert!(cone.is_within_cone(target));
    }

    #[test]
    fn closest_distance_no_draws() {
        let objects = vec![1.0, 10.0];

        assert_eq!(closest_object(&objects).unwrap(), 0);
    }
    #[test]
    fn closest_distance_draw_returns_first() {
        let objects = vec![15.0, 10.0, 10.0];

        assert_eq!(closest_object(&objects).unwrap(), 1);
    }
}
