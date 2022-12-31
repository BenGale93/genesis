use rand::{distributions::Standard, Rng};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Deserialize, Serialize)]
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
    Latch(u8),
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
            10 => ActivationFunctionKind::Latch(0),
            _ => ActivationFunctionKind::Identity,
        }
    }
}

impl ActivationFunctionKind {
    pub const fn display(&self) -> &str {
        match self {
            Self::Identity => "Identity",
            Self::Sigmoid => "Sigmoid",
            Self::Tanh => "Tanh",
            Self::Relu => "Relu",
            Self::Step => "Step",
            Self::Softsign => "SoftSign",
            Self::Sin => "Sin",
            Self::Gaussian => "Gaussian",
            Self::BentIdentity => "BentIdentity",
            Self::Selu => "Selu",
            Self::Latch(_) => "Latch",
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

fn latch(x: f32, s: u8) -> (f32, u8) {
    if x <= 0.0 {
        (0.0, 0)
    } else if x >= 1.0 {
        (1.0, 1)
    } else {
        (s as f32, s)
    }
}

pub fn activate(x: f32, kind: &mut ActivationFunctionKind) -> f32 {
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
        ActivationFunctionKind::Latch(s) => {
            let (out, new_s) = latch(x, *s);
            *s = new_s;
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Standard, prelude::StdRng, Rng, SeedableRng};

    use crate::activation;

    #[test]
    fn identity_returns_input() {
        assert_eq!(
            activation::activate(1.5, &mut activation::ActivationFunctionKind::Identity),
            1.5
        );
    }

    #[test]
    fn sigmoid_test() {
        assert_eq!(
            activation::activate(0.0, &mut activation::ActivationFunctionKind::Sigmoid),
            0.5
        );
    }

    #[test]
    fn tanh_test() {
        assert_eq!(
            activation::activate(0.0, &mut activation::ActivationFunctionKind::Tanh),
            0.0
        );
    }

    #[test]
    fn relu_test() {
        assert_eq!(
            activation::activate(0.5, &mut activation::ActivationFunctionKind::Relu),
            0.5
        );
        assert_eq!(
            activation::activate(-0.5, &mut activation::ActivationFunctionKind::Relu),
            -0.005
        );
    }

    #[test]
    fn step_test() {
        assert_eq!(
            activation::activate(0.5, &mut activation::ActivationFunctionKind::Step),
            1.0
        );
        assert_eq!(
            activation::activate(-0.5, &mut activation::ActivationFunctionKind::Step),
            0.0
        );
    }

    #[test]
    fn softsign_test() {
        assert_eq!(
            activation::activate(-1.0, &mut activation::ActivationFunctionKind::Softsign),
            -0.5
        );
    }

    #[test]
    fn sin_test() {
        assert_eq!(
            activation::activate(0.0, &mut activation::ActivationFunctionKind::Sin),
            0.0
        );
    }

    #[test]
    fn gaussian_test() {
        assert_eq!(
            activation::activate(0.0, &mut activation::ActivationFunctionKind::Gaussian),
            1.0
        );
    }

    #[test]
    fn bent_iden_test() {
        assert_eq!(
            activation::activate(1.0, &mut activation::ActivationFunctionKind::BentIdentity),
            1.207_106_8
        );
    }

    #[test]
    fn selu_test() {
        assert_eq!(
            activation::activate(1.0, &mut activation::ActivationFunctionKind::Selu),
            1.050_701
        );
        assert_eq!(
            activation::activate(-1.0, &mut activation::ActivationFunctionKind::Selu),
            -1.111_330_7
        );
    }

    #[test]
    fn latch_test_change() {
        let mut function = activation::ActivationFunctionKind::Latch(0);
        assert_eq!(activation::activate(1.0, &mut function), 1.0);
        assert_eq!(function, activation::ActivationFunctionKind::Latch(1));
    }

    #[test]
    fn latch_test_no_change() {
        let mut function = activation::ActivationFunctionKind::Latch(0);
        assert_eq!(activation::activate(0.5, &mut function), 0.0);
        assert_eq!(function, activation::ActivationFunctionKind::Latch(0));
    }

    #[test]
    fn test_random_activation() {
        let mut rng = StdRng::seed_from_u64(2);
        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Step);

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Latch(0));

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Sigmoid);

        let act_func: activation::ActivationFunctionKind = rng.sample(Standard);
        assert_eq!(act_func, activation::ActivationFunctionKind::Sin);
    }
}
