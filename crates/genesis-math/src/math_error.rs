use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("The length provided should be strictly greater than 0.")]
    LengthError,

    #[error("The angle provided should be between 0 and pi radians.")]
    AngleError,
}
