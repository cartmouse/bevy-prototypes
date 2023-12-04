use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("conveyor.png");
    commands.spawn(Camera2dBundle::default());
    [
        (-16.0, 16.0, Dir::RIGHT),
        (16.0, 16.0, Dir::DOWN),
        (-16.0, -16.0, Dir::UP),
        (16.0, -16.0, Dir::LEFT),
    ]
    .iter()
    .for_each(|x| {
        commands.spawn((
            ConveyorTile::new(x.2),
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x.0, x.1, 0.0),
                    rotation: rotation(x.2),
                    ..default()
                },
                ..default()
            },
        ));
    });
}

fn conveyor() {}

fn rotation(dir: Dir) -> Quat {
    return Quat::from_rotation_z(match dir {
        Dir::UP => PI / 2.0,
        Dir::DOWN => -PI / 2.0,
        Dir::LEFT => PI,
        Dir::RIGHT => 0.0,
    });
}
