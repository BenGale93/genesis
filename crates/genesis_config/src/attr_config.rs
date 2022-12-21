use serde_derive::{Deserialize, Serialize};

use super::validators::{attribute_limit, attribute_overlap};

type MinMax = (Option<f32>, Option<f32>);

struct AttributeConfigValidator {
    hatch_age: MinMax,
    max_speed: MinMax,
    max_rotation: MinMax,
    eye_range: MinMax,
    cost_of_eating: MinMax,
    offspring_energy: MinMax,
    hatch_size: MinMax,
    max_size: MinMax,
    growth_rate: MinMax,
}

impl Default for AttributeConfigValidator {
    fn default() -> Self {
        Self {
            hatch_age: (Some(9.0), Some(60.0)),
            max_speed: (Some(10.0), Some(1000.0)),
            max_rotation: (Some(1.0), Some(100.0)),
            eye_range: (Some(50.0), Some(2000.0)),
            cost_of_eating: (Some(0.0), Some(1.0)),
            offspring_energy: (Some(0.1), Some(1.0)),
            hatch_size: (Some(10.0), Some(50.0)),
            max_size: (Some(50.0), Some(150.0)),
            growth_rate: (Some(0.0), Some(1.0)),
        }
    }
}

type MinMaxLen = (f32, f32, usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeConfig {
    pub hatch_age: MinMaxLen,
    pub max_speed: MinMaxLen,
    pub max_rotation: MinMaxLen,
    pub eye_range: MinMaxLen,
    pub cost_of_eating: MinMaxLen,
    pub offspring_energy: MinMaxLen,
    pub hatch_size: MinMaxLen,
    pub max_size: MinMaxLen,
    pub growth_rate: MinMaxLen,
}

impl Default for AttributeConfig {
    fn default() -> Self {
        Self {
            hatch_age: (10.0, 30.0, 15),
            max_speed: (100.0, 500.0, 100),
            max_rotation: (10.0, 30.0, 20),
            eye_range: (200.0, 700.0, 100),
            cost_of_eating: (0.2, 0.3, 10),
            offspring_energy: (0.5, 1.0, 100),
            hatch_size: (20.0, 35.0, 15),
            max_size: (80.0, 100.0, 20),
            growth_rate: (0.05, 0.1, 20),
        }
    }
}

impl AttributeConfig {
    #[must_use]
    pub(super) fn validate(&self) -> Vec<Option<String>> {
        let validator = AttributeConfigValidator::default();
        macro_rules! attr_overlap {
            ($attr_left:ident, $attr_right:ident) => {
                attribute_overlap(
                    self.$attr_left,
                    self.$attr_right,
                    stringify!($attr_left),
                    stringify!($attr_right),
                )
            };
        }
        let mut messages = vec![attr_overlap!(hatch_size, max_size)];

        macro_rules! attrs_limit {
            ($attr:ident) => {
                messages.extend(attribute_limit(
                    self.$attr,
                    validator.$attr,
                    stringify!($attr),
                ))
            };
            ($attr:ident, $($attrs:ident), +) => {
                attrs_limit!($attr);
                attrs_limit!($($attrs), +)
            }
        }
        attrs_limit!(
            hatch_age,
            max_speed,
            max_rotation,
            eye_range,
            cost_of_eating,
            offspring_energy,
            hatch_size,
            max_size,
            growth_rate
        );
        messages
    }
}
