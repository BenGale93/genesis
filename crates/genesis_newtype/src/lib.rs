#![warn(clippy::all, clippy::nursery)]
use std::ops;

use bevy_reflect::{FromReflect, Reflect};
use rand::Rng;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenesisNewTypeError {
    #[error("Weight must be between -1 and 1.")]
    InvalidWeight,

    #[error("Probability must be between 0 and 1.")]
    InvalidProbability,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Reflect, Default)]
pub struct Probability(f32);

impl Probability {
    pub fn new(w: f32) -> Result<Self, GenesisNewTypeError> {
        if !(0_f32..=1_f32).contains(&w) {
            return Err(GenesisNewTypeError::InvalidProbability);
        }
        Ok(Self(w))
    }

    #[must_use]
    pub const fn as_float(&self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Reflect, Default, FromReflect)]
pub struct Weight(f32);

impl<'de> Deserialize<'de> for Weight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let w: f32 = Deserialize::deserialize(deserializer)?;
        Self::new(w).map_err(Error::custom)
    }
}

pub type Bias = Weight;

impl Weight {
    pub fn new(w: f32) -> Result<Self, GenesisNewTypeError> {
        if !(-1_f32..=1_f32).contains(&w) {
            return Err(GenesisNewTypeError::InvalidWeight);
        }
        Ok(Self(w))
    }

    #[must_use]
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen();
        let w = 2_f32.mul_add(x, -1_f32);

        Self(w)
    }

    #[must_use]
    pub fn abs(&self) -> Self {
        Self::new(self.0.abs()).unwrap()
    }

    #[must_use]
    pub const fn as_float(&self) -> f32 {
        self.0
    }
}

impl ops::Add for Weight {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let result = self.0.add(rhs.0);

        Self::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Sub for Weight {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let result = self.0.sub(rhs.0);

        Self::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Mul for Weight {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = self.0.mul(rhs.0);

        Self::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Div for Weight {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let result = self.0.div(rhs.0);

        Self::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate as util;

    #[test]
    fn valid_probability_back_to_float() {
        let f = 0.5;
        let p = util::Probability::new(f).unwrap();

        assert_eq!(p.as_float(), f);
    }

    #[test]
    fn valid_probability_lower_bound() {
        util::Probability::new(0.0).unwrap();
    }

    #[test]
    fn valid_probability_upper_bound() {
        util::Probability::new(1.0).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidProbability")]
    fn invalid_probability_upper_bound() {
        util::Probability::new(1.1).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidProbability")]
    fn invalid_probability_lower_bound() {
        util::Probability::new(-0.1).unwrap();
    }

    #[test]
    fn valid_weight_back_to_float() {
        let f = 0.5;
        let w = util::Weight::new(f).unwrap();

        assert_eq!(w.as_float(), f);
    }

    #[test]
    fn valid_weight_lower_bound() {
        util::Weight::new(-1.0).unwrap();
    }

    #[test]
    fn valid_weight_upper_bound() {
        util::Weight::new(1.0).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidWeight")]
    fn invalid_weight_upper_bound() {
        util::Weight::new(1.1).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidWeight")]
    fn invalid_weight_lower_bound() {
        util::Weight::new(-1.1).unwrap();
    }

    #[test]
    fn create_random_weight() {
        let w = util::Weight::random();

        assert!(-1.0 <= w.as_float() && 1.0 >= w.as_float());
    }

    #[test]
    fn absolute_weight_returns_valid() {
        let w = util::Weight::new(-0.5).unwrap();

        assert_eq!(w.abs().as_float(), 0.5);
    }

    #[test]
    fn added_weights_are_capped() {
        let w_one = util::Weight::new(0.5).unwrap();
        let w_two = util::Weight::new(0.75).unwrap();

        assert_eq!((w_one + w_two).as_float(), 1.0);
    }

    #[test]
    fn subtracted_weights_are_capped() {
        let w_one = util::Weight::new(-0.5).unwrap();
        let w_two = util::Weight::new(0.75).unwrap();

        assert_eq!((w_one - w_two).as_float(), -1.0);
    }

    #[test]
    fn multiplying_weights_works() {
        let w_one = util::Weight::new(-0.5).unwrap();
        let w_two = util::Weight::new(0.5).unwrap();

        assert_eq!((w_one * w_two).as_float(), -0.25);
    }

    #[test]
    fn divided_weights_are_capped() {
        let w_one = util::Weight::new(0.5).unwrap();
        let w_two = util::Weight::new(0.1).unwrap();

        assert_eq!((w_one / w_two).as_float(), 1.0);
    }
}
