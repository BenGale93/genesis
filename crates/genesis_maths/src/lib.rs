#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::approx_constant)]
use std::{f32::consts::PI, iter::Sum, ops::Div};

use glam::{Quat, Vec2, Vec3};
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
pub fn quat_to_angle(rotation: &Quat) -> f32 {
    rotation.z.asin() * 2.0
}

#[must_use]
pub fn angle_to_point(diff: Vec3) -> f32 {
    diff.y.atan2(diff.x)
}

#[must_use]
pub fn point_from_angle(y_angle: f32) -> Vec2 {
    let (y, x) = y_angle_to_x(y_angle).sin_cos();
    Vec2::new(x, y)
}

pub fn y_angle_to_x(angle: f32) -> f32 {
    wrap(angle + PI / 2.0, -PI, PI)
}

#[must_use]
pub fn rebased_angle(angle_from_x: f32, angle_from_y: f32) -> f32 {
    angle_from_x - (PI / 2.0) - angle_from_y
}

#[must_use]
pub fn angle_between(rotation: &Quat, translation: Vec3) -> f32 {
    let angle_to_target = angle_to_point(translation);
    let angle_to_self = quat_to_angle(rotation);
    let angle = rebased_angle(angle_to_target, angle_to_self);
    wrap(angle, -PI, PI)
}

pub fn cast_angles(mid_angle: f32, fov_angle: f32, freq: usize) -> Vec<f32> {
    let left_angle = wrap(mid_angle + fov_angle, -PI, PI);
    let right_angle = wrap(mid_angle - fov_angle, -PI, PI);
    let angle_between = wrap(wrap(left_angle - right_angle + PI, -PI, PI) - PI, -PI, PI);
    let mut angles = vec![];
    for i in 0..=freq {
        let sub_angle = wrap(
            angle_between.mul_add(i as f32 / freq as f32, right_angle),
            -PI,
            PI,
        );
        angles.push(sub_angle);
    }
    angles
}

#[must_use]
pub fn angle_difference(angle_to_self: f32, angle: f32) -> f32 {
    let new_angle = angle - angle_to_self;
    wrap(new_angle, -PI, PI)
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

#[cfg(test)]
mod tests {
    use glam::{Quat, Vec3};

    use super::{angle_to_point, cast_angles, rebased_angle};
    use crate::quat_to_angle;

    #[test]
    fn cast_angles_facing_forward() {
        let rot_z = Quat::from_rotation_z(f32::to_radians(0.0)).z;
        let fov_angle = f32::to_radians(30.0);
        let freq = 9;

        let angles = cast_angles(rot_z, fov_angle, freq);
        let expected = &[
            -0.523_598_8,
            -0.407_243_5,
            -0.29088816,
            -0.174_532_85,
            -0.058_177_534,
            0.058_177_803,
            0.174_533_11,
            0.290_888_43,
            0.407_243_73,
            0.523_599,
        ];

        assert_eq!(angles, expected);
    }

    #[test]
    fn cast_angles_facing_left() {
        let rotation = Quat::from_rotation_z(f32::to_radians(90.0));
        let mid_angle = quat_to_angle(&rotation);
        let fov_angle = f32::to_radians(30.0);
        let freq = 9;

        let angles = cast_angles(mid_angle, fov_angle, freq);
        let expected = &[
            1.047_197_5,
            1.163_552_8,
            1.279_908_1,
            1.396_263_4,
            1.512_618_7,
            1.628_974_1,
            1.745_329_4,
            1.861_684_7,
            1.978_04,
            2.094_395_2,
        ];

        assert_eq!(angles, expected);
    }

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
    fn angle_to_point_and_rebase_right_angle() {
        let pos_1 = Vec3::new(0.0, 0.0, 0.0);
        let pos_2 = Vec3::new(0.0, 10.0, 0.0);

        let angle = angle_to_point(pos_2 - pos_1);

        let rebased_angle = rebased_angle(angle, f32::to_radians(90.0));

        assert_eq!(rebased_angle, f32::to_radians(-90.0));
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

        assert_eq!(rebased_angle, f32::to_radians(-180.0));
    }
}
