use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    window::PrimaryWindow,
};

const SIZE: f32 = 20.0;
const HALF_SIZE: f32 = SIZE / 2.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
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

#[derive(Component)]
struct Item {
    dragging: bool,
}

impl Item {
    fn new() -> Self {
        Self { dragging: false }
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
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_items: Query<(&mut Item, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
) {
    let window = q_windows.single();

    if let Some(cursor_pos) = window.cursor_position() {
        let new_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.0,
            window.height() / 2.0 - cursor_pos.y,
        );

        q_items.iter_mut().for_each(|mut item| {
            if buttons.just_released(MouseButton::Left) {
                item.0.dragging = false;
                return;
            }

            if buttons.just_pressed(MouseButton::Left) && is_over(&item.1.translation, &new_pos) {
                item.0.dragging = true;
                return;
            }

            if item.0.dragging {
                item.1.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
            }
        });
    }
}

fn is_over(item_pos: &Vec3, cursor_pos: &Vec2) -> bool {
    return item_pos.x <= cursor_pos.x + HALF_SIZE
        && item_pos.x >= cursor_pos.x - HALF_SIZE
        && item_pos.y <= cursor_pos.y + HALF_SIZE
        && item_pos.y >= cursor_pos.y - HALF_SIZE;
}
