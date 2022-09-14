use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenesisUtilError {
    #[error("Weight must be between -1 and 1.")]
    InvalidWeight,

    #[error("Probability must be between 0 and 1.")]
    InvalidProbability,

    #[error("The length provided should be strictly greater than 0.")]
    LengthError,

    #[error("The angle provided should be between 0 and pi radians.")]
    AngleError,
}
