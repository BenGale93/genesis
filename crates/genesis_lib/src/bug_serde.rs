use std::fs;

use bevy::{log::warn, prelude::Resource};
use derive_getters::Getters;
use genesis_attributes as attributes;
use genesis_components::mind;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BugSerdeError {
    #[error(transparent)]
    MindValidation(#[from] mind::MindValidationError),
    #[error(transparent)]
    DnaValidation(#[from] attributes::DnaValidationError),
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct BugBlueprint {
    mind: mind::Mind,
    dna: attributes::Dna,
}

impl BugBlueprint {
    fn validate(&self, genome: &attributes::Genome) -> Result<(), BugSerdeError> {
        self.mind.validate()?;
        self.dna.validate(genome)?;
        Ok(())
    }
}

#[derive(Debug, Resource, Default)]
pub struct LoadedBlueprint {
    pub blueprint: Option<BugBlueprint>,
}

pub fn save_bug(bug: &(&mind::Mind, &attributes::Dna)) {
    let path = std::env::current_dir().unwrap();
    let Some(res) = rfd::FileDialog::new()
                        .set_file_name("bug.json")
                        .set_directory(path)
                        .save_file() else
                    {
                        return;
                    };
    let bug_info = BugBlueprint {
        mind: bug.0.to_owned(),
        dna: bug.1.to_owned(),
    };
    let bug_json = serde_json::to_string_pretty(&bug_info).unwrap();
    if let Err(e) = fs::write(res, bug_json) {
        warn!("Could not save bug. Please try again. {e}")
    };
}

pub fn load_bug_blueprint(
    genome: &attributes::Genome,
) -> Result<Option<BugBlueprint>, BugSerdeError> {
    let Some(path) = rfd::FileDialog::new().pick_file() else {
        return Ok(None);
    };
    let content = fs::read(path)?;
    let blueprint: BugBlueprint = serde_json::from_slice(&content)?;
    blueprint.validate(genome)?;
    Ok(Some(blueprint))
}
