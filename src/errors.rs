use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenesisError {
    #[error("Weight must be between -1 and 1.")]
    InvalidWeight,

    #[error("Synapses cannot start and end with the same Neuron.")]
    InvalidFromTo,

    #[error("A synapse cannot be added here.")]
    SynapseError,

    #[error("The index given is out of bounds.")]
    OutOfBounds,

    #[error("A neuron cannot be added here.")]
    NeuronError,
}
