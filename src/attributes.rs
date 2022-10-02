use bevy::prelude::*;
use genesis_genome::Genome;
use genesis_util::Probability;

const GENOME_READ_ERROR: &str = "Expected to be able to read from here";

#[derive(Debug)]
struct AttributeConfig {
    lower: f32,
    upper: f32,
    chromosome: usize,
    start: usize,
    length: usize,
}

impl AttributeConfig {
    fn new(lower: f32, upper: f32, chromosome: usize, start: usize, length: usize) -> Self {
        Self {
            lower,
            upper,
            chromosome,
            start,
            length,
        }
    }

    fn read_genome(&self, genome: &Genome) -> f32 {
        genome
            .read_float(
                self.lower,
                self.upper,
                self.chromosome,
                self.start,
                self.length,
            )
            .expect(GENOME_READ_ERROR)
    }
}

macro_rules! impl_attribute {
    ($name:ident) => {
        impl $name {
            fn new(value: f32, config: AttributeConfig) -> Self {
                Self { value, config }
            }

            pub fn value(&self) -> f32 {
                self.value
            }

            pub fn from_genome(genome: &Genome) -> Self {
                let attribute_config = Self::default_config();
                let value = attribute_config.read_genome(genome);
                Self::new(value, attribute_config)
            }
        }
    };
}

#[derive(Component, Debug)]
pub struct AdultAge {
    value: f32,
    config: AttributeConfig,
}

impl AdultAge {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(30.0, 50.0, 1, 10, 10)
    }
}

impl_attribute!(AdultAge);

#[derive(Component, Debug)]
pub struct DeathAge {
    value: f32,
    config: AttributeConfig,
}

impl DeathAge {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(600.0, 700.0, 1, 10, 10)
    }
}

impl_attribute!(DeathAge);

#[derive(Component, Debug)]
pub struct MutationProbability {
    value: Probability,
    config: AttributeConfig,
}

impl MutationProbability {
    fn new(value: Probability, config: AttributeConfig) -> Self {
        Self { value, config }
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = Probability::new(attribute_config.read_genome(genome) as f64)
            .expect("Expected to be between 0.0 and 1.0.");
        Self::new(value, attribute_config)
    }

    pub fn value(&self) -> &Probability {
        &self.value
    }

    fn default_config() -> AttributeConfig {
        AttributeConfig::new(0.0, 0.2, 2, 0, 100)
    }
}

#[derive(Component, Debug)]
pub struct MaxSpeed {
    value: f32,
    config: AttributeConfig,
}

impl MaxSpeed {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(100.0, 500.0, 0, 0, 100)
    }
}

impl_attribute!(MaxSpeed);

#[derive(Component, Debug)]
pub struct MaxRotationRate {
    value: f32,
    config: AttributeConfig,
}

impl MaxRotationRate {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(10.0, 30.0, 0, 0, 20)
    }
}

impl_attribute!(MaxRotationRate);

#[derive(Component, Debug)]
pub struct EyeRange {
    value: f32,
    config: AttributeConfig,
}

impl EyeRange {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(200.0, 700.0, 2, 20, 50)
    }
}

impl_attribute!(EyeRange);

#[derive(Component, Debug)]
pub struct EyeAngle {
    value: f32,
    config: AttributeConfig,
}

impl EyeAngle {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(f32::to_radians(30.0), f32::to_radians(360.0), 2, 50, 50)
    }
}

impl_attribute!(EyeAngle);

#[derive(Component, Debug)]
pub struct InternalTimerBoundary {
    value: f32,
    config: AttributeConfig,
}

impl InternalTimerBoundary {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(-0.5, 0.5, 0, 70, 10)
    }
}

impl_attribute!(InternalTimerBoundary);

#[derive(Component, Debug)]
pub struct LayEggBoundary {
    value: f32,
    config: AttributeConfig,
}

impl LayEggBoundary {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(0.0, 0.9, 0, 70, 10)
    }
}

impl_attribute!(LayEggBoundary);

#[derive(Component, Debug)]
pub struct OffspringEnergy {
    value: usize,
    config: AttributeConfig,
}

impl OffspringEnergy {
    fn new(value: usize, config: AttributeConfig) -> Self {
        Self { value, config }
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome) as usize;
        Self::new(value, attribute_config)
    }

    pub fn value(&self) -> usize {
        self.value
    }
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(400.0, 600.0, 0, 50, 50)
    }
}

#[derive(Bundle, Debug)]
pub struct AttributeBundle {
    pub adult_age: AdultAge,
    pub death_age: DeathAge,
    pub mutation_probability: MutationProbability,
    pub translation_speed: MaxSpeed,
    pub rotation_speed: MaxRotationRate,
    pub eye_range: EyeRange,
    pub eye_angle: EyeAngle,
    pub internal_timer_boundary: InternalTimerBoundary,
    pub lay_egg_boundary: LayEggBoundary,
    pub offspring_energy: OffspringEnergy,
}

impl AttributeBundle {
    pub fn new(genome: &Genome) -> Self {
        let adult_age = AdultAge::from_genome(genome);
        let death_age = DeathAge::from_genome(genome);
        let mutation_probability = MutationProbability::from_genome(genome);
        let translation_speed = MaxSpeed::from_genome(genome);
        let rotation_speed = MaxRotationRate::from_genome(genome);
        let eye_range = EyeRange::from_genome(genome);
        let eye_angle = EyeAngle::from_genome(genome);
        let internal_timer_boundary = InternalTimerBoundary::from_genome(genome);
        let lay_egg_boundary = LayEggBoundary::from_genome(genome);
        let offspring_energy = OffspringEnergy::from_genome(genome);

        Self {
            adult_age,
            death_age,
            mutation_probability,
            translation_speed,
            rotation_speed,
            eye_range,
            eye_angle,
            internal_timer_boundary,
            lay_egg_boundary,
            offspring_energy,
        }
    }
}

#[derive(Component, Debug)]
pub struct HatchAge {
    value: f32,
    config: AttributeConfig,
}

impl HatchAge {
    fn default_config() -> AttributeConfig {
        AttributeConfig::new(10.0, 30.0, 1, 80, 10)
    }
}

impl_attribute!(HatchAge);

#[derive(Bundle, Debug)]
pub struct EggAttributeBundle {
    pub hatch_age: HatchAge,
}

impl EggAttributeBundle {
    pub fn new(genome: &Genome) -> Self {
        let hatch_age = HatchAge::from_genome(genome);

        Self { hatch_age }
    }
}
