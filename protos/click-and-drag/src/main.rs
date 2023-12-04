use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    window::PrimaryWindow,
};

use utils::*;

// TODO: Handle drag when items are on top of one another

const SIZE: f32 = 20.0;
const TARGET_SIZE: f32 = SIZE * 1.5;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Click and Drag".to_string(),
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
        .add_systems(Update, cursor_position)
        .run();
}

#[derive(Component, Debug, Clone)]
struct Item {
    dragging: bool,
}

impl Item {
    fn new() -> Self {
        Self { dragging: false }
    }
}

#[derive(Component)]
struct Target {
    filled: Option<Entity>,
}

impl Target {
    fn new() -> Self {
        Self { filled: None }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let items = [0.0, 30.0, 60.0, 90.0, 120.0].iter().map(|x| {
        (
            Item::new(),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SIZE, SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(*x, 0.0, 0.0)),
                ..default()
            },
        )
    });
    commands.spawn_batch(items);

    let targets = [(100.0, 100.0)].iter().map(|coords| {
        (
            Target::new(),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TARGET_SIZE, TARGET_SIZE)),
                    color: Color::GRAY,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(coords.0, coords.1, -1.0)),
                ..default()
            },
        )
    });
    commands.spawn_batch(targets);
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_items: Query<(&mut Item, &mut Transform, Entity), Without<Target>>,
    mut q_targets: Query<(&mut Target, &mut Transform), Without<Item>>,
    buttons: Res<Input<MouseButton>>,
) {
    let window = q_windows.single();

    if let Some(cursor_pos) = window.cursor_position() {
        let new_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.0,
            window.height() / 2.0 - cursor_pos.y,
        );

        q_items.iter_mut().for_each(|mut item| {
            if buttons.just_pressed(MouseButton::Left)
                && is_pos_over(&new_pos, &item.1.translation, SIZE)
            {
                item.0.dragging = true;
            }

            if item.0.dragging {
                item.1.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
                q_targets.iter_mut().for_each(|mut target| {
                    if buttons.just_released(MouseButton::Left) {
                        if is_item_over(
                            &target.1.translation,
                            TARGET_SIZE,
                            &item.1.translation,
                            SIZE,
                        ) {
                            let id = Some(item.2);
                            if target.0.filled.is_none() || target.0.filled == id {
                                item.1.translation =
                                    Vec3::new(target.1.translation.x, target.1.translation.y, 0.0);
                                target.0.filled = id;
                            }
                        }
                        item.0.dragging = false;
                    }
                });
            }
        });
        q_targets.iter_mut().for_each(|mut target| {
            if !q_items.iter().any(|item| {
                is_item_over(
                    &target.1.translation,
                    TARGET_SIZE,
                    &item.1.translation,
                    SIZE,
                )
            }) {
                target.0.filled = None;
            }
        });
    }
}
