use bevy::{
    ecs::system::EntityCommands,
    prelude::{
        default, AssetServer, Bundle, Color, Commands, DespawnRecursiveExt, Entity, EventReader,
        Handle, Image, Query, Res, ResMut, Resource, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, Damping, ExternalImpulse, RigidBody, Velocity,
};
use components::body::{OriginalColor, Size};
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{
    body, eat, grab, grow, lay, mind, see, time, BurntEnergy, Generation, SizeMultiplier,
};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_spawners::Spawners;
use genesis_traits::BehaviourTracker;
use rand_distr::{Distribution, Uniform};

type BugParts<'a> = (
    mind::Mind,
    &'a Color,
    &'a attributes::HatchSize,
    &'a attributes::MaxSize,
);

pub fn bug_sprite_bundle(
    asset_server: &Res<AssetServer>,
    size: &Size,
    color: &Color,
    mind_color: Color,
) -> impl Bundle {
    let original_color = body::OriginalColor(mind_color);
    // Allows selected eggs to remain selected on hatching
    let current_color = if *color == Color::RED {
        *color
    } else {
        original_color.0
    };

    let texture: Handle<Image> = asset_server.load("sprite.png");
    let sprite = Sprite {
        custom_size: Some(size.sprite()),
        color: current_color,
        ..default()
    };

    (texture, sprite, original_color)
}

pub fn spawn_bug(
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    bug_parts: BugParts,
    mut hatching_entity: EntityCommands,
) -> ecosystem::Energy {
    let (mind, egg_color, hatch_size, max_size) = bug_parts;
    let mind_bundle = mind::MindBundle::new(&mind);

    let size = body::Size::new(**hatch_size, **max_size);
    let (vitality, leftover_energy) = body::Vitality::new(size, energy);

    hatching_entity
        .insert(bug_sprite_bundle(
            asset_server,
            vitality.size(),
            egg_color,
            mind.color(),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(vitality.size().collider())
        .insert(SizeMultiplier::new(vitality.size().current_size()))
        .insert(components::Juvenile)
        .insert(vitality)
        .insert(mind_bundle)
        .insert(see::Vision::new())
        .insert(time::Age::default())
        .insert(time::Heart::new())
        .insert(time::InternalTimer::new())
        .insert(components::TranslationSum::new())
        .insert(components::RotationSum::new())
        .insert(components::ThinkingSum::new())
        .insert(eat::EatingSum::new())
        .insert(lay::LayingSum::new())
        .insert(grow::GrowingSum::new())
        .insert(grow::SizeSum::new())
        .insert(grab::GrabbingSum::new())
        .insert(eat::EnergyConsumed(0))
        .insert(lay::EggsLaid(0));

    leftover_energy
}

#[derive(Bundle)]
pub struct EggBundle {
    pub egg: components::Egg,
    pub egg_energy: ecosystem::EggEnergy,
    pub sprite: Sprite,
    pub handle: Handle<Image>,
    pub original_color: body::OriginalColor,
    pub collider: Collider,
    pub age: time::Age,
}

pub fn egg_sprite_bundle(
    asset_server: &Res<AssetServer>,
    size: f32,
    original_color: &OriginalColor,
    location: Vec3,
) -> impl Bundle {
    SpriteBundle {
        texture: asset_server.load("egg.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(size, size)),
            color: original_color.0,
            ..default()
        },
        transform: Transform::from_translation(location),
        ..default()
    }
}

pub fn spawn_egg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    genome: &Res<attributes::Genome>,
    energy: ecosystem::Energy,
    location: Vec3,
    dna: attributes::Dna,
    mind: mind::Mind,
    generation: components::Generation,
    parent_id: Option<Entity>,
) -> Entity {
    let size = 16.0;

    let attribute_bundle = attributes::AttributeBundle::new(&dna, genome);
    let original_color = body::OriginalColor(Color::WHITE);

    let mut egg_entity = commands.spawn(egg_sprite_bundle(
        asset_server,
        size,
        &original_color,
        location,
    ));
    let entity = egg_entity.id();

    egg_entity
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(Velocity::zero())
        .insert(ExternalImpulse::default())
        .insert(Collider::ball(size / 2.0))
        .insert(components::Egg)
        .insert(attribute_bundle)
        .insert(ecosystem::EggEnergy(energy))
        .insert(original_color)
        .insert(dna)
        .insert(components::Relations::new(
            (entity, mind.color()),
            parent_id,
        ))
        .insert(mind)
        .insert(time::Age::default())
        .insert(generation)
        .insert(BurntEnergy::new());

    entity
}

pub fn nearest_spawner_system(
    mut spawners: ResMut<Spawners>,
    organisms: Query<&Transform, With<Generation>>,
    plants: Query<(&Transform, &ecosystem::Food), With<components::Plant>>,
) {
    let mut organism_counts = vec![0; spawners.len()];
    for position in organisms.iter() {
        let distances: Vec<f32> = spawners
            .iter()
            .map(|s| s.distance(&position.translation))
            .collect();
        let index = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        organism_counts[index] += 1;
    }
    for (i, spawner) in spawners.iter_mut().enumerate() {
        spawner.set_nearby_organisms(organism_counts[i]);
    }
    let mut food_counts = vec![0; spawners.len()];
    for (transform, plant) in plants.iter() {
        let distances: Vec<f32> = spawners
            .iter()
            .map(|s| s.distance(&transform.translation))
            .collect();
        let index = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        food_counts[index] += plant.energy().amount();
    }
    for (i, spawner) in spawners.iter_mut().enumerate() {
        spawner.set_nearby_food(food_counts[i]);
    }
}

pub fn food_sprite_bundle(
    asset_server: &Res<AssetServer>,
    size: Option<Vec2>,
    location: Vec3,
    color: Color,
) -> impl Bundle {
    let original_color = body::OriginalColor(color);
    let sprite_bundle = SpriteBundle {
        texture: asset_server.load("food.png"),
        sprite: Sprite {
            custom_size: size,
            color: original_color.0,
            ..default()
        },
        transform: Transform::from_translation(location),
        ..default()
    };
    (original_color, sprite_bundle)
}

fn spawn_plant(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
) {
    let food = ecosystem::Food::new(energy);

    commands
        .spawn(food_sprite_bundle(
            &asset_server,
            food.sprite_size(),
            location,
            Color::GREEN,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(food.collider())
        .insert(ColliderMassProperties::Density(
            config::WorldConfig::global().plant_density,
        ))
        .insert(Velocity::zero())
        .insert(ExternalImpulse::default())
        .insert(food)
        .insert(components::Plant);
}

#[derive(Resource)]
pub struct PlantSizeRandomiser(Uniform<f32>);

impl PlantSizeRandomiser {
    pub fn new(bounds: (f32, f32)) -> Self {
        Self(Uniform::new(bounds.0, bounds.1))
    }
    pub fn random_size(&self, rng: &mut rand::rngs::ThreadRng) -> f32 {
        self.0.sample(rng)
    }
}

pub fn spawn_plant_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    spawners: Res<Spawners>,
    plant_size_randomiser: Res<PlantSizeRandomiser>,
) {
    let config_instance = config::WorldConfig::global();
    let available_energy = ecosystem.available_energy().amount();

    if available_energy
        > (config_instance.start_num * config_instance.start_energy)
            .max(config_instance.energy_floor)
    {
        let mut rng = rand::thread_rng();
        let size = plant_size_randomiser.random_size(&mut rng);
        let Some(energy) =
            ecosystem.request_energy(size as usize * config_instance.plant_energy_per_unit) else {return};
        let location = spawners.random_food_position(&mut rng);
        spawn_plant(&mut commands, asset_server, energy, location);
    }
}

pub fn update_food_size(
    mut ev_eaten: EventReader<eat::EatenEvent>,
    mut food_query: Query<(&mut Sprite, &mut Collider, &ecosystem::Food)>,
) {
    for ev in ev_eaten.iter() {
        if let Ok(food_extract) = food_query.get_mut(ev.0) {
            let (mut sprite, mut collider, food) = food_extract;
            sprite.custom_size = food.sprite_size();
            *collider = food.collider();
        }
    }
}

pub fn despawn_food_system(mut commands: Commands, food_query: Query<Entity, With<eat::Eaten>>) {
    for entity in food_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
