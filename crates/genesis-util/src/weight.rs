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
        Ok(Weight(w))
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let w = 2_f64 * x - 1_f64;

        Weight(w)
    }

    pub fn abs(&self) -> Self {
        Weight::new(self.0.abs()).unwrap()
    }

    pub fn as_float(&self) -> f64 {
        self.0
    }
}

impl ops::Add for Weight {
    type Output = Weight;

    fn add(self, rhs: Weight) -> Self {
        let result = self.0.add(rhs.0);

        Weight::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Sub for Weight {
    type Output = Weight;

    fn sub(self, rhs: Weight) -> Self {
        let result = self.0.sub(rhs.0);

        Weight::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Mul for Weight {
    type Output = Weight;

    fn mul(self, rhs: Weight) -> Self {
        let result = self.0.mul(rhs.0);

        Weight::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}

impl ops::Div for Weight {
    type Output = Weight;

    fn div(self, rhs: Weight) -> Self {
        let result = self.0.div(rhs.0);

        Weight::new(result.clamp(-1.0, 1.0)).unwrap()
    }
}
