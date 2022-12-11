use std::ops;

use rand::Rng;

use crate::GenesisUtilError;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Weight(f32);

pub type Bias = Weight;

impl Weight {
    pub fn new(w: f32) -> Result<Self, GenesisUtilError> {
        if !(-1_f32..=1_f32).contains(&w) {
            return Err(GenesisUtilError::InvalidWeight);
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
