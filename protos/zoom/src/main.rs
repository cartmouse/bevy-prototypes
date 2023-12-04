use bevy::{
    input::mouse::MouseWheel,
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
                        title: "Zoom".to_string(),
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
        .add_systems(Update, scroll_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for item in 0..400 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                item as f32 * 30.0,
                item as f32 * 30.0,
                0.0,
            )),
            ..default()
        });
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                -item as f32 * 30.0,
                -item as f32 * 30.0,
                0.0,
            )),
            ..default()
        });
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                item as f32 * 30.0,
                -item as f32 * 30.0,
                0.0,
            )),
            ..default()
        });
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                -item as f32 * 30.0,
                item as f32 * 30.0,
                0.0,
            )),
            ..default()
        });
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut OrthographicProjection>,
) {
    for ev in scroll_evr.read() {
        let new_value = 10.0_f32.min(0.5_f32.max(q_camera.single_mut().scale - ev.y * 0.3));
        q_camera.single_mut().scale = new_value;

        println!("{}", new_value);
    }
}
