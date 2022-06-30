use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum ActivationFunctionKind {
    Identity,
    Sigmoid,
}

impl Distribution<ActivationFunctionKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ActivationFunctionKind {
        match rng.gen_range(0..2) {
            0 => ActivationFunctionKind::Sigmoid,
            _ => ActivationFunctionKind::Identity,
        }
    }
}

pub fn identity(x: f64) -> f64 {
    x
}

pub fn sigmoid(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}

pub fn random_activation() -> ActivationFunctionKind {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..2) {
        0 => ActivationFunctionKind::Identity,
        _ => ActivationFunctionKind::Sigmoid,
    }
}

pub fn activate(x: f64, kind: &ActivationFunctionKind) -> f64 {
    match kind {
        ActivationFunctionKind::Identity => identity(x),
        ActivationFunctionKind::Sigmoid => sigmoid(x),
    }
}
