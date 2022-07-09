use thiserror::Error;

use crate::brain;

#[derive(Error, Debug)]
pub enum GenesisError {
    #[error("Weight must be between -1 and 1.")]
    InvalidWeight,

    #[error("Probability must be between 0 and 1.")]
    InvalidProbability,

    #[error(transparent)]
    BrainError(#[from] brain::BrainError),
}
