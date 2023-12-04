use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

use utils::*;

const CONVEYOR_SIZE: f32 = 32.0;
const HALF_CONVEYOR: f32 = CONVEYOR_SIZE / 2.0;
const ITEM_SIZE: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Conveyor".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }
                    .into(),
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, conveyor)
        .insert_resource(MoveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .run();
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
struct ConveyorTile {
    direction: Dir,
}

impl ConveyorTile {
    fn new(direction: Dir) -> Self {
        Self { direction }
    }
}

#[derive(Component)]
struct Item {
    moving: bool,
}

impl Item {
    fn new() -> Self {
        Self { moving: false }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("conveyor.png");
    commands.spawn(Camera2dBundle::default());
    [
        (-HALF_CONVEYOR, HALF_CONVEYOR, Dir::RIGHT),
        (HALF_CONVEYOR, HALF_CONVEYOR, Dir::RIGHT),
        (CONVEYOR_SIZE * 1.5, HALF_CONVEYOR, Dir::DOWN),
        (CONVEYOR_SIZE * 1.5, -HALF_CONVEYOR, Dir::DOWN),
        (CONVEYOR_SIZE * 1.5, -CONVEYOR_SIZE * 1.5, Dir::LEFT),
        (HALF_CONVEYOR, -CONVEYOR_SIZE * 1.5, Dir::LEFT),
        (-HALF_CONVEYOR, -CONVEYOR_SIZE * 1.5, Dir::UP),
        (-HALF_CONVEYOR, -HALF_CONVEYOR, Dir::UP),
    ]
    .iter()
    .for_each(|item| {
        commands.spawn((
            ConveyorTile::new(item.2),
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(item.0, item.1, 0.0),
                    rotation: rotation(item.2),
                    ..default()
                },
                ..default()
            },
        ));
    });
    commands.spawn((
        Item::new(),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(HALF_CONVEYOR, HALF_CONVEYOR, 1.0)),
            ..default()
        },
    ));
}

#[derive(Resource)]
struct MoveTimer(Timer);

fn conveyor(
    mut q_items: Query<(&mut Item, &mut Transform)>,
    q_conveyors: Query<(&ConveyorTile, &Transform), Without<Item>>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        q_items.iter_mut().for_each(|mut item| {
            q_conveyors.iter().for_each(|conveyor| {
                if item.0.moving {
                    item.0.moving = false;
                    return;
                }

                if is_item_over(
                    &conveyor.1.translation,
                    CONVEYOR_SIZE,
                    &item.1.translation,
                    ITEM_SIZE,
                ) {
                    item.0.moving = true;
                    let pos = item.1.translation;
                    item.1.translation = match conveyor.0.direction {
                        Dir::UP => Vec3::new(pos.x, pos.y + CONVEYOR_SIZE, pos.z),
                        Dir::DOWN => Vec3::new(pos.x, pos.y - CONVEYOR_SIZE, pos.z),
                        Dir::LEFT => Vec3::new(pos.x - CONVEYOR_SIZE, pos.y, pos.z),
                        Dir::RIGHT => Vec3::new(pos.x + CONVEYOR_SIZE, pos.y, pos.z),
                    };
                    return;
                }
            });
        });
    }
}

fn rotation(dir: Dir) -> Quat {
    return Quat::from_rotation_z(match dir {
        Dir::UP => PI / 2.0,
        Dir::DOWN => -PI / 2.0,
        Dir::LEFT => PI,
        Dir::RIGHT => 0.0,
    });
}
