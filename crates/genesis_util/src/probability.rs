use crate::GenesisUtilError;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f32);

impl Probability {
    pub fn new(w: f32) -> Result<Self, GenesisUtilError> {
        if !(0_f32..=1_f32).contains(&w) {
            return Err(GenesisUtilError::InvalidProbability);
        }
        Ok(Self(w))
    }

    #[must_use]
    pub const fn as_float(&self) -> f32 {
        self.0
    }
}
