use std::{fs, io::Write};

use bevy::{
    log::{info, warn},
    prelude::{AppTypeRegistry, EventReader, Query, Res, ResMut, Resource, With, World},
    scene::DynamicScene,
    tasks::IoTaskPool,
    time::Time,
};
use derive_getters::Getters;
use genesis_attributes as attributes;
use genesis_components::{mind, time::SimulationTime};
use genesis_config::WorldConfig;
use genesis_ecosystem::Ecosystem;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    statistics::{BugPerformance, CountStats, EnergyStats, FamilyTree},
    ui::{LoadBugEvent, SaveBugEvent, Selected},
};

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

fn save_bug(bug: &(&mind::Mind, &attributes::Dna)) {
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

pub fn save_bug_system(
    mut time: ResMut<Time>,
    bug_query: Query<(&mind::Mind, &attributes::Dna), With<Selected>>,
    ev_save_bug: EventReader<SaveBugEvent>,
) {
    if ev_save_bug.is_empty() {
        return;
    };
    let Ok(bug) = bug_query.get_single() else {
        return;
    };
    save_bug(&bug);
    time.update();
}

fn load_bug_blueprint(genome: &attributes::Genome) -> Result<Option<BugBlueprint>, BugSerdeError> {
    let Some(path) = rfd::FileDialog::new().pick_file() else {
        return Ok(None);
    };
    let content = fs::read(path)?;
    let blueprint: BugBlueprint = serde_json::from_slice(&content)?;
    blueprint.validate(genome)?;
    Ok(Some(blueprint))
}

pub fn load_bug_system(
    mut time: ResMut<Time>,
    ev_load_bug: EventReader<LoadBugEvent>,
    genome: Res<attributes::Genome>,
    mut loaded_blueprint: ResMut<LoadedBlueprint>,
) {
    if ev_load_bug.is_empty() {
        return;
    };
    match load_bug_blueprint(&genome) {
        Ok(x) => loaded_blueprint.blueprint = x,
        Err(e) => warn!("{e}"),
    };
    time.update();
}

#[derive(Serialize, Deserialize, Getters)]
pub struct SimulationSerializer {
    config: WorldConfig,
    sim_time: SimulationTime,
    ecosystem: Ecosystem,
    count_stats: CountStats,
    energy_stats: EnergyStats,
    bug_performance: BugPerformance,
    family_tree: FamilyTree,
}

impl SimulationSerializer {
    pub fn new(world: &World) -> Self {
        let config = genesis_config::WorldConfig::global().to_owned();
        let sim_time = world.get_resource::<SimulationTime>().unwrap().to_owned();
        let ecosystem = world.get_resource::<Ecosystem>().unwrap().to_owned();
        let count_stats = world.get_resource::<CountStats>().unwrap().to_owned();
        let energy_stats = world.get_resource::<EnergyStats>().unwrap().to_owned();
        let bug_performance = world.get_resource::<BugPerformance>().unwrap().to_owned();
        let family_tree = world.get_resource::<FamilyTree>().unwrap().to_owned();
        Self {
            config,
            sim_time,
            ecosystem,
            count_stats,
            energy_stats,
            bug_performance,
            family_tree,
        }
    }
}

fn serialize_simulation(world: &World) -> String {
    let simulation = SimulationSerializer::new(world);
    let pretty_config = ron::ser::PrettyConfig::default()
        .indentor("  ".to_string())
        .new_line("\n".to_string());
    ron::ser::to_string_pretty(&simulation, pretty_config).unwrap()
}

pub fn save_simulation_system(world: &World) {
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(world, type_registry);
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    let serialized_sim = serialize_simulation(world);
    let path = std::env::current_dir().unwrap();
    let Some(res) = rfd::FileDialog::new()
                        .set_directory(path)
                        .pick_folder() else
                    {
                        return;
                    };

    IoTaskPool::get()
        .spawn(async move {
            match fs::File::create(res.join("scene.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            {
                Ok(_) => info!("Saved simulation."),
                Err(e) => warn!("Could not save simulation. Please try again: {e}."),
            };
            match fs::File::create(res.join("resources.ron"))
                .and_then(|mut file| file.write(serialized_sim.as_bytes()))
            {
                Ok(_) => info!("Saved simulation resources."),
                Err(e) => warn!("Could not save simulation. Please try again: {e}."),
            };
        })
        .detach();
}
