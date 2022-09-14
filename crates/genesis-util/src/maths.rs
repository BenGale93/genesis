use std::f32::consts::PI;

use glam::Vec3;
use nalgebra::wrap;

use crate::util_error::GenesisUtilError;

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

pub struct Cone {
    point: Vec3,
    angle: f32,
    length: f32,
}

impl Cone {
    pub fn new(point: Vec3, angle: f32, length: f32) -> Result<Self, GenesisUtilError> {
        if length <= 0.0 {
            return Err(GenesisUtilError::LengthError);
        }

        if angle <= 0.0 || angle > f32::to_radians(360.0) {
            return Err(GenesisUtilError::AngleError);
        }

        Ok(Self {
            point,
            angle,
            length,
        })
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn is_within_cone(self, target: Vec3) -> bool {
        let distance = target - self.point;
        let angle = angle_to_point(distance);

        if distance.length() > self.length {
            return false;
        }

        let angle = wrap(angle, -PI, PI);

        if angle < -self.angle / 2.0 || angle > self.angle / 2.0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use glam::Vec3;

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

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(3.0, 3.0, 0.0);

        assert!(cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_to_far_away() {
        let me = Vec3::ZERO;

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(8.0, 8.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_behind_me() {
        let me = Vec3::ZERO;

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Vec3::new(-1.0, -1.0, 0.0);

        assert!(!cone.is_within_cone(target));
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
