use std::ops;

use rand::Rng;

use crate::GenesisUtilError;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Weight(f64);

pub type Bias = Weight;

impl Weight {
    pub fn new(w: f64) -> Result<Self, GenesisUtilError> {
        if !(-1_f64..=1_f64).contains(&w) {
            return Err(GenesisUtilError::InvalidWeight);
        }
        Ok(Self(w))
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let w = 2_f64.mul_add(x, -1_f64);

        Self(w)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.0.abs()).unwrap()
    }

    pub const fn as_float(&self) -> f64 {
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
