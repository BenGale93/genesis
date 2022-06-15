mod graph;
pub mod neuron;
pub mod synapse;

pub use neuron::{Neuron, NeuronKind, Neurons, NeuronsExt};
pub use synapse::{create_synapses, Synapse, Synapses};
