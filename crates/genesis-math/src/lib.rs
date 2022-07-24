use std::f32::consts::PI;

use bevy_transform::components::Transform;
mod math_error;
use math_error::MathError;
use nalgebra::wrap;

pub struct DistanceAngle {
    distance: f32,
    angle: f32,
}

impl DistanceAngle {
    pub fn new(distance: f32, angle: f32) -> Self {
        Self { distance, angle }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}

pub fn angle_distance_between(me: Transform, them: Transform) -> DistanceAngle {
    let diff = them.translation - me.translation;

    DistanceAngle::new(diff.length(), diff.y.atan2(diff.x))
}

pub fn closest_object(objects: &[DistanceAngle]) -> Option<usize> {
    objects
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.distance
                .partial_cmp(&b.distance)
                .expect("There should be no NaNs.")
        })
        .map(|(index, _)| index)
}

pub struct Cone {
    point: Transform,
    angle: f32,
    length: f32,
}

impl Cone {
    pub fn new(point: Transform, angle: f32, length: f32) -> Result<Self, MathError> {
        if length <= 0.0 {
            return Err(MathError::LengthError);
        }

        if angle <= 0.0 || angle > f32::to_radians(360.0) {
            return Err(MathError::AngleError);
        }

        Ok(Self {
            point,
            angle,
            length,
        })
    }

    pub fn point(&self) -> Transform {
        self.point
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn is_within_cone(self, target: Transform) -> bool {
        let dist_ang = angle_distance_between(self.point, target);

        if dist_ang.distance() > self.length {
            return false;
        }

        let angle = wrap(dist_ang.angle(), -PI, PI);

        if angle < -self.angle / 2.0 || angle > self.angle / 2.0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use bevy_transform::components::Transform;

    use super::{angle_distance_between, Cone};
    use crate::{closest_object, DistanceAngle};

    #[test]
    fn distance_between_transforms_same_x() {
        let me = Transform::from_xyz(0.0, 0.0, 0.0);
        let them = Transform::from_xyz(0.0, 2.0, 0.0);

        assert_eq!(angle_distance_between(me, them).distance(), 2.0);
    }

    #[test]
    fn distance_between_transforms_diagonal() {
        let me = Transform::from_xyz(0.0, 0.0, 0.0);
        let them = Transform::from_xyz(3.0, 4.0, 0.0);

        assert_eq!(angle_distance_between(me, them).distance(), 5.0);
    }

    #[test]
    fn angle_between_from_origin() {
        let me = Transform::from_xyz(0.0, 0.0, 0.0);
        let them = Transform::from_xyz(3.0, 4.0, 0.0);

        assert_eq!(angle_distance_between(me, them).angle(), 0.9272952);
    }

    #[test]
    fn angle_between_same_x() {
        let me = Transform::from_xyz(1.0, 0.0, 0.0);
        let them = Transform::from_xyz(1.0, 4.0, 0.0);

        assert_eq!(angle_distance_between(me, them).angle(), 1.5707964);
    }

    #[test]
    fn angle_between_same_y() {
        let me = Transform::from_xyz(1.0, 4.0, 0.0);
        let them = Transform::from_xyz(5.0, 4.0, 0.0);

        assert_eq!(angle_distance_between(me, them).angle(), 0.0);
    }

    #[test]
    fn is_within_cone_true() {
        let me = Transform::identity();

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Transform::from_xyz(3.0, 3.0, 0.0);

        assert!(cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_to_far_away() {
        let me = Transform::identity();

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Transform::from_xyz(8.0, 8.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn is_not_within_cone_behind_me() {
        let me = Transform::identity();

        let cone = Cone::new(me, f32::to_radians(180.0), 10.0).unwrap();

        let target = Transform::from_xyz(-1.0, -1.0, 0.0);

        assert!(!cone.is_within_cone(target));
    }

    #[test]
    fn closest_distance_no_draws() {
        let objects = vec![DistanceAngle::new(1.0, 0.0), DistanceAngle::new(10.0, 0.0)];

        assert_eq!(closest_object(&objects).unwrap(), 0);
    }
    #[test]
    fn closest_distance_draw_returns_first() {
        let objects = vec![
            DistanceAngle::new(15.0, 0.0),
            DistanceAngle::new(10.0, 0.0),
            DistanceAngle::new(10.0, 0.0),
        ];

        assert_eq!(closest_object(&objects).unwrap(), 1);
    }
}
