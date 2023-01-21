use serde_derive::{Deserialize, Serialize};

use super::validators::attribute_limit;

type MinMax = (Option<f32>, Option<f32>);

struct AttributeConfigValidator {
    hatch_age: MinMax,
    eye_range: MinMax,
    cost_of_eating: MinMax,
    offspring_energy: MinMax,
    max_size: MinMax,
    growth_rate: MinMax,
    grab_angle: MinMax,
    food_preference: MinMax,
    base_attack: MinMax,
}

impl Default for AttributeConfigValidator {
    fn default() -> Self {
        Self {
            hatch_age: (Some(9.0), Some(60.0)),
            eye_range: (Some(50.0), Some(2000.0)),
            cost_of_eating: (Some(0.0), Some(1.0)),
            offspring_energy: (Some(0.1), Some(1.0)),
            max_size: (Some(50.0), Some(150.0)),
            growth_rate: (Some(0.0), Some(1.0)),
            grab_angle: (Some(20.0), Some(90.0)),
            food_preference: (Some(0.0), Some(1.0)),
            base_attack: (Some(20.0), Some(200.0)),
        }
    }
}

type MinMaxLen = (f32, f32, usize);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeConfig {
    pub hatch_age: MinMaxLen,
    pub eye_range: MinMaxLen,
    pub cost_of_eating: MinMaxLen,
    pub offspring_energy: MinMaxLen,
    pub max_size: MinMaxLen,
    pub growth_rate: MinMaxLen,
    pub grab_angle: MinMaxLen,
    pub food_preference: MinMaxLen,
    pub base_attack: MinMaxLen,
}

impl Default for AttributeConfig {
    fn default() -> Self {
        Self {
            hatch_age: (10.0, 30.0, 15),
            eye_range: (200.0, 700.0, 100),
            cost_of_eating: (0.2, 0.3, 10),
            offspring_energy: (0.5, 1.0, 100),
            max_size: (80.0, 100.0, 20),
            growth_rate: (0.05, 0.1, 20),
            grab_angle: (30.0, 60.0, 10),
            food_preference: (0.0, 1.0, 100),
            base_attack: (20.0, 200.0, 40),
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
            eye_range,
            cost_of_eating,
            offspring_energy,
            max_size,
            growth_rate,
            grab_angle,
            food_preference,
            base_attack
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
    grab_strength_bounds: MinMax,
    base_defence_bounds: MinMax,
}

impl Default for DependentAttributeConfigValidator {
    fn default() -> Self {
        Self {
            adult_age_bounds: (Some(20.0), Some(100.0)),
            death_age_bounds: (Some(350.0), Some(1000.0)),
            eye_angle_bounds: (Some(40.0), Some(360.0)),
            mouth_width_bounds: (Some(20.0), Some(180.0)),
            hatch_size_bounds: (Some(10.0), Some(49.0)),
            grab_strength_bounds: (Some(0.0), Some(1.0)),
            base_defence_bounds: (Some(0.2), Some(0.8)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependentAttributeConfig {
    pub adult_age_bounds: (f32, f32),
    pub death_age_bounds: (f32, f32),
    pub eye_angle_bounds: (f32, f32),
    pub mouth_width_bounds: (f32, f32),
    pub hatch_size_bounds: (f32, f32),
    pub grab_strength_bounds: (f32, f32),
    pub base_defence_bounds: (f32, f32),
}

impl Default for DependentAttributeConfig {
    fn default() -> Self {
        Self {
            adult_age_bounds: (30.0, 60.0),
            death_age_bounds: (400.0, 600.0),
            eye_angle_bounds: (60.0, 330.0),
            mouth_width_bounds: (30.0, 90.0),
            hatch_size_bounds: (20.0, 35.0),
            grab_strength_bounds: (0.01, 0.05),
            base_defence_bounds: (0.3, 0.7),
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
            hatch_size_bounds,
            grab_strength_bounds,
            base_defence_bounds
        );
        messages
    }
}
