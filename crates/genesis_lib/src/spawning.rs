use bevy::{
    ecs::system::EntityCommands,
    prelude::{
        default, AssetServer, Bundle, Color, Commands, DespawnRecursiveExt, Entity, Handle, Image,
        Query, Res, ResMut, Resource, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, Damping, RigidBody, Velocity,
};
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{
    body, eat, grow, lay, mind, see, time, BurntEnergy, Generation, SizeMultiplier,
};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_spawners::Spawners;
use rand_distr::{Distribution, Uniform};

type BugParts<'a> = (
    attributes::Genome,
    mind::Mind,
    &'a Color,
    &'a attributes::HatchSize,
    &'a attributes::MaxSize,
);

pub fn spawn_bug(
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    bug_parts: BugParts,
    mut hatching_entity: EntityCommands,
) -> ecosystem::Energy {
    let (bug_body, mind, color, hatch_size, max_size) = bug_parts;
    let mind_bundle = mind::MindBundle::new(&mind);

    let original_color = body::OriginalColor(mind.color());
    // Allows selected eggs to remain selected on hatching
    let current_color = if *color == Color::RED {
        *color
    } else {
        original_color.0
    };

    let size = body::Size::new(**hatch_size, **max_size);
    let (vitality, leftover_energy) = body::Vitality::new(size, energy);

    let sprite_image: Handle<Image> = asset_server.load("sprite.png");
    let sprite = Sprite {
        custom_size: Some(vitality.size().sprite()),
        color: current_color,
        ..default()
    };

    hatching_entity
        .insert(sprite_image)
        .insert(sprite)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(vitality.size().collider())
        .insert(SizeMultiplier::new(vitality.size().current_size()))
        .insert(components::Juvenile)
        .insert(original_color)
        .insert(bug_body)
        .insert(vitality)
        .insert(mind_bundle)
        .insert(see::Vision::new())
        .insert(time::Age::default())
        .insert(time::Heart::new())
        .insert(time::InternalTimer::new())
        .insert(components::MovementSum::new())
        .insert(components::ThinkingSum::new())
        .insert(eat::EatingSum::new())
        .insert(grow::GrowingSum::new())
        .insert(grow::SizeSum::new())
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

pub fn spawn_egg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
    genome: attributes::Genome,
    mind: mind::Mind,
    generation: components::Generation,
    parent_id: Option<Entity>,
) -> Entity {
    let size = 16.0;

    let attribute_bundle = attributes::AttributeBundle::new(&genome);
    let original_color = body::OriginalColor(Color::WHITE);
    let sprite = SpriteBundle {
        texture: asset_server.load("egg.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(size, size)),
            color: original_color.0,
            ..default()
        },
        transform: Transform::from_translation(location),
        ..default()
    };

    let mut egg_entity = commands.spawn(sprite);
    let entity = egg_entity.id();

    egg_entity
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(Velocity::zero())
        .insert(Collider::ball(size / 2.0))
        .insert(components::Egg)
        .insert(attribute_bundle)
        .insert(ecosystem::EggEnergy(energy))
        .insert(original_color)
        .insert(genome)
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
    plants: Query<(&Transform, &ecosystem::Plant)>,
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

fn spawn_plant(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
) {
    let original_color = body::OriginalColor(Color::GREEN);
    let plant = ecosystem::Plant::new(energy);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("food.png"),
            sprite: Sprite {
                custom_size: plant.sprite_size(),
                color: original_color.0,
                ..default()
            },
            ..default()
        })
        .insert(original_color)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(TransformBundle::from(Transform::from_translation(location)))
        .insert(plant.collider())
        .insert(ColliderMassProperties::Density(
            config::WorldConfig::global().plant_density,
        ))
        .insert(Velocity::zero())
        .insert(plant);
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

    if available_energy > (config_instance.start_num * config_instance.start_energy) {
        let mut rng = rand::thread_rng();
        let size = plant_size_randomiser.random_size(&mut rng);
        let Some(energy) =
            ecosystem.request_energy(size as usize * config_instance.plant_energy_per_unit) else {return};
        let location = spawners.random_food_position(&mut rng);
        spawn_plant(&mut commands, asset_server, energy, location);
    }
}

pub fn update_plant_size(mut plant_query: Query<(&mut Sprite, &mut Collider, &ecosystem::Plant)>) {
    // Might be able to improve this using bevy events.
    // Basically listen for changes to plants and only then update.
    for (mut sprite, mut collider, plant) in plant_query.iter_mut() {
        sprite.custom_size = plant.sprite_size();
        *collider = plant.collider();
    }
}

pub fn despawn_plants_system(mut commands: Commands, plant_query: Query<Entity, With<eat::Eaten>>) {
    for entity in plant_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
