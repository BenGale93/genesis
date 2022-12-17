#![warn(clippy::all, clippy::nursery)]
use std::{f32::consts::PI, iter::Sum, ops::Div};

use glam::{Quat, Vec3};
use nalgebra::wrap;
use num::{One, Unsigned};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenesisMathsError {
    #[error("The length provided should be strictly greater than 0.")]
    LengthError,

    #[error("The angle provided should be between 0 and pi radians.")]
    AngleError,
}

#[must_use]
pub fn mean<T>(numbers: Vec<T>) -> f32
where
    T: Copy + Sum + Div<f32, Output = f32>,
{
    let len = numbers.len();
    let sum: T = numbers.into_iter().sum();
    if len == 0 {
        return f32::NAN;
    }
    sum / len as f32
}

#[must_use]
pub fn polars_to_cart(r: f32, theta: f32) -> (f32, f32) {
    let (y_ang, x_ang) = theta.sin_cos();
    (r * x_ang, r * y_ang)
}

#[must_use]
pub fn angle_to_point(diff: Vec3) -> f32 {
    diff.y.atan2(diff.x)
}

#[must_use]
pub fn rebased_angle(angle_from_x: f32, angle_from_y: f32) -> f32 {
    (angle_from_x - (PI / 2.0) - angle_from_y).abs()
}

#[must_use]
pub fn linear_interpolate(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
    y_min.mul_add(x_max - x, y_max * (x - x_min)) / (x_max - x_min)
}

#[must_use]
pub fn cantor_pairing<T: Unsigned + One + Copy>(x: T, y: T) -> T {
    let z = (x + y) * (x + y + T::one());
    (z / (T::one() + T::one())) + y
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
    ) -> Result<Self, GenesisMathsError> {
        if length <= 0.0 {
            return Err(GenesisMathsError::LengthError);
        }

        if angle <= 0.0 || angle > f32::to_radians(360.0) {
            return Err(GenesisMathsError::AngleError);
        }

        Ok(Self {
            point,
            rotation,
            fov_angle: angle,
            fov_length: length,
        })
    }

    #[must_use]
    pub const fn point(&self) -> Vec3 {
        self.point
    }

    #[must_use]
    pub const fn angle(&self) -> f32 {
        self.fov_angle
    }

    #[must_use]
    pub const fn length(&self) -> f32 {
        self.fov_length
    }

    #[must_use]
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

    use super::{angle_to_point, rebased_angle, Cone};

    #[test]
    fn angle_between_from_origin() {
        let me = Vec3::new(0.0, 0.0, 0.0);
        let them = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(angle_to_point(them - me), 0.927_295_2);
    }

    #[test]
    fn angle_between_same_x() {
        let me = Vec3::new(1.0, 0.0, 0.0);
        let them = Vec3::new(1.0, 4.0, 0.0);

        assert_eq!(angle_to_point(them - me), 1.570_796_4);
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
    fn angle_to_point_and_rebase_right_angle() {
        let pos_1 = Vec3::new(0.0, 0.0, 0.0);
        let pos_2 = Vec3::new(0.0, 10.0, 0.0);

        let angle = angle_to_point(pos_2 - pos_1);

        let rebased_angle = rebased_angle(angle, f32::to_radians(90.0));

        assert_eq!(rebased_angle, f32::to_radians(90.0));
    }

    #[test]
    fn angle_to_point_and_rebase_forty_five() {
        let pos_1 = Vec3::new(0.0, 0.0, 0.0);
        let pos_2 = Vec3::new(10.0, 10.0, 0.0);

        let angle = angle_to_point(pos_2 - pos_1);

        let rebased_angle = rebased_angle(angle, f32::to_radians(-90.0));

        assert_eq!(rebased_angle, f32::to_radians(45.0));
    }

    #[test]
    fn angle_to_point_and_rebase_facing_away() {
        let pos_1 = Vec3::new(0.0, 0.0, 0.0);
        let pos_2 = Vec3::new(0.0, 10.0, 0.0);

        let angle = angle_to_point(pos_2 - pos_1);

        let rebased_angle = rebased_angle(angle, 1.0_f32.asin() * 2.0);

        assert_eq!(rebased_angle, f32::to_radians(180.0));
    }
}
