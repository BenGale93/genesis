use rand::{distributions::Standard, Rng};
use rand_distr::Distribution;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum ActivationFunctionKind {
    Identity,
    Sigmoid,
    Tanh,
    Relu,
    Step,
    Softsign,
    Sin,
    Gaussian,
    BentIdentity,
    Selu,
}

impl Distribution<ActivationFunctionKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ActivationFunctionKind {
        match rng.gen_range(0..=10) {
            0 => ActivationFunctionKind::Sigmoid,
            1 => ActivationFunctionKind::Tanh,
            2 => ActivationFunctionKind::Relu,
            3 => ActivationFunctionKind::Step,
            4 => ActivationFunctionKind::Softsign,
            5 => ActivationFunctionKind::Sin,
            6 => ActivationFunctionKind::Gaussian,
            8 => ActivationFunctionKind::BentIdentity,
            9 => ActivationFunctionKind::Selu,
            _ => ActivationFunctionKind::Identity,
        }
    }
}

const fn identity(x: f32) -> f32 {
    x
}

fn sigmoid(x: f32) -> f32 {
    1. / (1. + (-x).exp())
}

fn tanh(x: f32) -> f32 {
    x.tanh()
}

fn relu(x: f32) -> f32 {
    if x > 0. {
        x
    } else {
        0.01 * x
    }
}

fn step(x: f32) -> f32 {
    if x > 0. {
        1.
    } else {
        0.
    }
}

fn softsign(x: f32) -> f32 {
    x / (1. + x.abs())
}

fn sin(x: f32) -> f32 {
    x.sin()
}

fn gaussian(x: f32) -> f32 {
    (-x.powi(2)).exp()
}

fn bent_iden(x: f32) -> f32 {
    ((x.mul_add(x, 1.).sqrt() - 1.) / 2.) + x
}

fn selu(x: f32) -> f32 {
    let alpha = 1.673_263_2;
    let scale = 1.050_701;

    let fx = if x > 0. { x } else { alpha * x.exp_m1() };

    fx * scale
}

pub fn activate(x: f32, kind: &ActivationFunctionKind) -> f32 {
    match kind {
        ActivationFunctionKind::Identity => identity(x),
        ActivationFunctionKind::Sigmoid => sigmoid(x),
        ActivationFunctionKind::Tanh => tanh(x),
        ActivationFunctionKind::Relu => relu(x),
        ActivationFunctionKind::Step => step(x),
        ActivationFunctionKind::Softsign => softsign(x),
        ActivationFunctionKind::Sin => sin(x),
        ActivationFunctionKind::Gaussian => gaussian(x),
        ActivationFunctionKind::BentIdentity => bent_iden(x),
        ActivationFunctionKind::Selu => selu(x),
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Standard, prelude::StdRng, Rng, SeedableRng};

    use crate::activation;

    #[test]
    fn identity_returns_input() {
        assert_eq!(
            activation::activate(1.5, &activation::ActivationFunctionKind::Identity),
            1.5
        );
    }

    #[test]
    fn sigmoid_test() {
        assert_eq!(
            activation::activate(0.0, &activation::ActivationFunctionKind::Sigmoid),
            0.5
        );
    }

    #[test]
    fn tanh_test() {
        assert_eq!(
            activation::activate(0.0, &activation::ActivationFunctionKind::Tanh),
            0.0
        );
    }

    #[test]
    fn relu_test() {
        assert_eq!(
            activation::activate(0.5, &activation::ActivationFunctionKind::Relu),
            0.5
        );
        assert_eq!(
            activation::activate(-0.5, &activation::ActivationFunctionKind::Relu),
            -0.005
        );
    }

    #[test]
    fn step_test() {
        assert_eq!(
            activation::activate(0.5, &activation::ActivationFunctionKind::Step),
            1.0
        );
        assert_eq!(
            activation::activate(-0.5, &activation::ActivationFunctionKind::Step),
            0.0
        );
    }

    #[test]
    fn softsign_test() {
        assert_eq!(
            activation::activate(-1.0, &activation::ActivationFunctionKind::Softsign),
            -0.5
        );
    }

    #[test]
    fn sin_test() {
        assert_eq!(
            activation::activate(0.0, &activation::ActivationFunctionKind::Sin),
            0.0
        );
    }

    #[test]
    fn gaussian_test() {
        assert_eq!(
            activation::activate(0.0, &activation::ActivationFunctionKind::Gaussian),
            1.0
        );
    }

    #[test]
    fn bent_iden_test() {
        assert_eq!(
            activation::activate(1.0, &activation::ActivationFunctionKind::BentIdentity),
            1.207_106_8
        );
    }

    #[test]
    fn selu_test() {
        assert_eq!(
            activation::activate(1.0, &activation::ActivationFunctionKind::Selu),
            1.050_701
        );
        assert_eq!(
            activation::activate(-1.0, &activation::ActivationFunctionKind::Selu),
            -1.111_330_7
        );
    }

    #[test]
    fn test_random_activation() {
        let mut rng = StdRng::seed_from_u64(2);
        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Step);

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Identity);

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Sigmoid);

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Sin);
    }
}
