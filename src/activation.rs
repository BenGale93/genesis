use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub enum ActivationFunctionKind {
    Identity,
    Sigmoid,
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
