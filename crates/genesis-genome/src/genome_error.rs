use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenomeError {
    #[error("The Chromosome requested could not be found.")]
    ChromosomeNotFoundError,

    #[error("Invalid DNA segment read.")]
    DnaReadError,
}
