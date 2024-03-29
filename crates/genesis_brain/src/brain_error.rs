use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainError {
    #[error("Synapses cannot start and end with the same Neuron.")]
    InvalidFromTo,

    #[error("A synapse cannot be added here.")]
    SynapseError,

    #[error("The index '{0}' given is out of bounds.")]
    OutOfBounds(usize),

    #[error("A neuron cannot be added here.")]
    NeuronError,

    #[error("This neuron cannot be remove")]
    NeuronRemovalError,

    #[error("Input array is of incorrect length")]
    InputArrayError,
}
