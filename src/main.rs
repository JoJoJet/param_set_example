use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(blue_square) // spawns normally.
        .add_startup_system(red_square) // never appears, the commands get ignored.
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn blue_square(mut commands: Commands) {
    eprintln!("Spawn blue square.");
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.0)),
            color: Color::BLUE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-100.0, 0., 0.),
            ..default()
        },
        ..default()
    });
}
fn red_square(mut params: ParamSet<(Commands,)>) {
    eprintln!("Spawn red square.");
    // if you change this to use raw `Commands` instead of a ParamSet,
    // the square gets spawned as expected.
    let mut commands = params.p0();
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.0)),
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, 0., 0.),
            ..default()
        },
        ..default()
    });
}
