use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

use utils::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Collector".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::DX12),
                        ..default()
                    }
                    .into(),
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_item, collect_item, drop_off_item))
        .run();
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Collector { ..default() },
        SpriteBundle {
            texture: asset.load("racoon.png"),
            ..default()
        },
    ));
    commands.spawn((
        Source::new(Item),
        SpriteBundle {
            texture: asset.load("onion_spawner.png"),
            transform: Transform::from_translation(Vec3::new(300.0, 300.0, 0.0)),
            ..default()
        },
    ));
    commands.spawn((
        Target::new(Item),
        SpriteBundle {
            texture: asset.load("target.png"),
            transform: Transform::from_translation(Vec3::new(-300.0, -300.0, 0.0)),

            ..default()
        },
    ));
}

fn spawn_item(
    q_sources: Query<(&Source, &Transform)>,
    q_items: Query<&Item>,
    asset: Res<AssetServer>,
    mut commands: Commands,
) {
    if q_items.iter().count() != 0 {
        return;
    }

    for source in q_sources.iter() {
        commands.spawn((
            source.0.item.clone(),
            SpriteBundle {
                texture: asset.load("onion.png"),
                transform: Transform::from_translation(
                    source.1.translation + Vec3::new(0.0, -32.0, 0.0),
                ),
                ..default()
            },
        ));
    }
}

fn collect_item(
    mut q_collectors: Query<(&mut Collector, &mut Transform, Entity), Without<Item>>,
    mut q_items: Query<(&mut Item, &Transform, Entity), Without<Collector>>,
) {
    for mut collector in q_collectors.iter_mut().filter(|x| x.0.holding.is_none()) {
        if let Ok(mut item) = q_items.get_single_mut() {
            if is_item_over(&collector.1.translation, 16.0, &item.1.translation, 16.0) {
                collector.0.holding = Some(item.2);
                return;
            }

            let lerp = (item.1.translation - collector.1.translation) * 0.05;
            collector.1.translation += lerp;
        }
    }
}

fn drop_off_item(
    mut q_collectors: Query<(&mut Collector, &mut Transform), (Without<Target>, Without<Item>)>,
    mut q_items: Query<(&Item, &mut Transform, Entity), (Without<Target>, Without<Collector>)>,
    q_targets: Query<(&Target, &Transform), (Without<Collector>, Without<Item>)>,
    mut commands: Commands,
) {
    for mut collector in q_collectors.iter_mut().filter(|x| x.0.holding.is_some()) {
        if let Some(mut item) = q_items
            .iter_mut()
            .filter(|x| collector.0.holding.is_some_and(|y| y == x.2))
            .last()
        {
            if let Ok(target) = q_targets.get_single() {
                if is_item_over(&collector.1.translation, 16.0, &target.1.translation, 16.0) {
                    commands.entity(item.2).despawn();
                    collector.0.holding = None;
                    return;
                }

                let lerp = (target.1.translation - collector.1.translation) * 0.05;
                collector.1.translation += lerp;
                item.1.translation += lerp;
            }
        }
    }
}

// A _Collector_ picks up an _Item_ from a _Source_ and moves it to the corresponding _Target_
#[derive(Component, Debug, Clone)]
struct Collector {
    holding: Option<Entity>,
}

impl Default for Collector {
    fn default() -> Self {
        Self { holding: None }
    }
}

// A source spawns an _Item_ for the _Collector_ to collect
#[derive(Component, Debug, Clone)]
struct Source {
    item: Item,
}

impl Source {
    fn new(item: Item) -> Self {
        Self { item }
    }
}

// A target recieves an _Item_ from a _Collector_
#[derive(Component, Debug, Clone)]
struct Target {
    item: Item,
}

impl Target {
    fn new(item: Item) -> Self {
        Self { item }
    }
}

// An _Item_ is collected by a _Collector_ from a _Source_ and moved to a _Target_
#[derive(Component, Debug, Clone)]
struct Item;
