//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod variables;
use variables::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Inspector plugin
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Rectangle
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    },));
    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });

    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.29, 0.0, 0.51))),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Player,
    ));

    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::rgb(0.29, 0.0, 0.51))),
    //         transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //         ..default()
    //     },
    //     Player,
    // ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut paddle_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        paddle_transform.translation.x -= 1.0 * PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        paddle_transform.translation.x += 1.0 * PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        paddle_transform.translation.y -= 1.0 * PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        paddle_transform.translation.y += 1.0 * PLAYER_SPEED;
    }
}
