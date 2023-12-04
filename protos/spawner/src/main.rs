use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::RenderPlugin;

use utils::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }
                    .into(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Spawner".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_loop)
        .run();
}

#[derive(Component, Debug, Clone, Copy)]
struct Item;

impl Item {
    fn new() -> Self {
        Self
    }
}

#[derive(Component, Clone, Copy)]
struct Spawner {
    item: Item,
}

impl Spawner {
    fn new(item: Item) -> Self {
        Self { item }
    }
    fn spawn(self, commands: &mut Commands, pos: Vec3, q_items: &Query<(&Item, &Transform)>) {
        if q_items
            .iter()
            .any(|item| is_pos_over(&Vec2::new(pos.x, pos.y - 40.0), &item.1.translation, 10.0))
        {
            return;
        };
        println!("Test");
        commands.spawn((
            self.item,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y - 40.0, 0.0)),
                ..default()
            },
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn_batch([(-100.0, 100.0), (100.0, 100.0)].iter().map(|item| {
        (
            Spawner::new(Item::new()),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(item.0, item.1, 0.0)),
                ..default()
            },
        )
    }));
}

fn spawn_loop(
    mut commands: Commands,
    q_spawner: Query<(&Spawner, &Transform)>,
    q_items: Query<(&Item, &Transform)>,
) {
    q_spawner.iter().for_each(|spawner| {
        spawner
            .0
            .spawn(&mut commands, spawner.1.translation, &q_items)
    })
}
