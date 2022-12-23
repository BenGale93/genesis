use std::fs;

use bevy::{log::warn, prelude::Resource};
use derive_getters::Getters;
use genesis_attributes as attributes;
use genesis_components::mind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct BugBlueprint {
    mind: mind::Mind,
    genome: attributes::Genome,
}

#[derive(Debug, Resource, Default)]
pub struct LoadedBlueprint {
    pub blueprint: Option<BugBlueprint>,
}

pub fn save_bug(bug: &(&mind::Mind, &attributes::Genome)) {
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
        genome: bug.1.to_owned(),
    };
    let bug_json = serde_json::to_string_pretty(&bug_info).unwrap();
    if let Err(e) = fs::write(res, bug_json) {
        warn!("Could not save bug. Please try again. {e}")
    };
}

#[must_use]
pub fn load_bug_blueprint() -> Option<BugBlueprint> {
    let Some(path) = rfd::FileDialog::new().pick_file() else {
        return None;
    };
    let content = fs::read(path).unwrap();
    let blueprint: BugBlueprint = serde_json::from_slice(&content).unwrap();
    Some(blueprint)
}
