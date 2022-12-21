use serde_derive::{Deserialize, Serialize};

use super::validators::attribute_limit;

type MinMax = (Option<f32>, Option<f32>);

struct AttributeConfigValidator {
    hatch_age: MinMax,
    max_speed: MinMax,
    max_rotation: MinMax,
    eye_range: MinMax,
    cost_of_eating: MinMax,
    offspring_energy: MinMax,
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
            max_size: (80.0, 100.0, 20),
            growth_rate: (0.05, 0.1, 20),
        }
    }
}

impl AttributeConfig {
    #[must_use]
    pub(super) fn validate(&self) -> Vec<Option<String>> {
        let validator = AttributeConfigValidator::default();
        let mut messages = vec![];

        macro_rules! attrs_limit {
            ($attr:ident) => {
                messages.extend(attribute_limit(
                    self.$attr.0,
                    self.$attr.1,
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
            max_size,
            growth_rate
        );
        messages
    }
}

struct DependentAttributeConfigValidator {
    adult_age_bounds: MinMax,
    death_age_bounds: MinMax,
    eye_angle_bounds: MinMax,
    mouth_width_bounds: MinMax,
    hatch_size_bounds: MinMax,
}

impl Default for DependentAttributeConfigValidator {
    fn default() -> Self {
        Self {
            adult_age_bounds: (Some(20.0), Some(100.0)),
            death_age_bounds: (Some(350.0), Some(1000.0)),
            eye_angle_bounds: (Some(40.0), Some(360.0)),
            mouth_width_bounds: (Some(20.0), Some(180.0)),
            hatch_size_bounds: (Some(10.0), Some(49.0)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependentAttributeConfig {
    pub adult_age_bounds: (f32, f32),
    pub death_age_bounds: (f32, f32),
    pub eye_angle_bounds: (f32, f32),
    pub mouth_width_bounds: (f32, f32),
    pub hatch_size_bounds: (f32, f32),
}

impl Default for DependentAttributeConfig {
    fn default() -> Self {
        Self {
            adult_age_bounds: (30.0, 60.0),
            death_age_bounds: (400.0, 600.0),
            eye_angle_bounds: (60.0, 330.0),
            mouth_width_bounds: (30.0, 90.0),
            hatch_size_bounds: (20.0, 35.0),
        }
    }
}

impl DependentAttributeConfig {
    #[must_use]
    pub(super) fn validate(&self) -> Vec<Option<String>> {
        let validator = DependentAttributeConfigValidator::default();
        let mut messages = vec![];

        macro_rules! attrs_limit {
            ($attr:ident) => {
                messages.extend(attribute_limit(
                    self.$attr.0,
                    self.$attr.1,
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
            adult_age_bounds,
            death_age_bounds,
            eye_angle_bounds,
            mouth_width_bounds,
            hatch_size_bounds
        );
        messages
    }
}
