pub mod maths;
pub mod probability;
pub mod util_error;
pub mod weight;

pub use crate::{
    probability::Probability,
    util_error::GenesisUtilError,
    weight::{Bias, Weight},
};

#[cfg(test)]
mod tests {
    use crate as util;

    #[test]
    fn valid_probability_back_to_float() {
        let f = 0.5;
        let p = util::Probability::new(f).unwrap();

        assert_eq!(p.as_float(), f);
    }

    #[test]
    fn valid_probability_lower_bound() {
        util::Probability::new(0.0).unwrap();
    }

    #[test]
    fn valid_probability_upper_bound() {
        util::Probability::new(1.0).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidProbability")]
    fn invalid_probability_upper_bound() {
        util::Probability::new(1.1).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidProbability")]
    fn invalid_probability_lower_bound() {
        util::Probability::new(-0.1).unwrap();
    }

    #[test]
    fn valid_weight_back_to_float() {
        let f = 0.5;
        let w = util::Weight::new(f).unwrap();

        assert_eq!(w.as_float(), f);
    }

    #[test]
    fn valid_weight_lower_bound() {
        util::Weight::new(-1.0).unwrap();
    }

    #[test]
    fn valid_weight_upper_bound() {
        util::Weight::new(1.0).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidWeight")]
    fn invalid_weight_upper_bound() {
        util::Weight::new(1.1).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: InvalidWeight")]
    fn invalid_weight_lower_bound() {
        util::Weight::new(-1.1).unwrap();
    }

    #[test]
    fn create_random_weight() {
        let w = util::Weight::random();

        assert!(-1.0 <= w.as_float() && 1.0 >= w.as_float());
    }

    #[test]
    fn absolute_weight_returns_valid() {
        let w = util::Weight::new(-0.5).unwrap();

        assert_eq!(w.abs().as_float(), 0.5);
    }

    #[test]
    fn added_weights_are_capped() {
        let w_one = util::Weight::new(0.5).unwrap();
        let w_two = util::Weight::new(0.75).unwrap();

        assert_eq!((w_one + w_two).as_float(), 1.0);
    }

    #[test]
    fn subtracted_weights_are_capped() {
        let w_one = util::Weight::new(-0.5).unwrap();
        let w_two = util::Weight::new(0.75).unwrap();

        assert_eq!((w_one - w_two).as_float(), -1.0);
    }

    #[test]
    fn multiplying_weights_works() {
        let w_one = util::Weight::new(-0.5).unwrap();
        let w_two = util::Weight::new(0.5).unwrap();

        assert_eq!((w_one * w_two).as_float(), -0.25);
    }

    #[test]
    fn divided_weights_are_capped() {
        let w_one = util::Weight::new(0.5).unwrap();
        let w_two = util::Weight::new(0.1).unwrap();

        assert_eq!((w_one / w_two).as_float(), 1.0);
    }
}
