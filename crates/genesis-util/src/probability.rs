use crate::GenesisUtilError;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    pub fn new(w: f64) -> Result<Self, GenesisUtilError> {
        if !(0_f64..=1_f64).contains(&w) {
            return Err(GenesisUtilError::InvalidProbability);
        }
        Ok(Probability(w))
    }

    pub fn as_float(&self) -> f64 {
        self.0
    }
}
