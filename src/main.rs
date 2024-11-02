use bevy::color::palettes::css::{LIGHT_GRAY, WHITE_SMOKE, BLUE, LIMEGREEN};
use bevy::prelude::*;

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::Srgba(WHITE_SMOKE)))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .add_systems(Update, move_ball)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Kamera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Licht
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });

    // Grundplatte
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(5.0,0.1,5.0))),
        material: materials.add(Color::Srgba(LIGHT_GRAY)),
        transform: Transform::from_xyz(1.0, -0.5, -1.0),
        ..default()
    });


    // Blauer Würfel
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::Srgba(BLUE)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Cube,
    ));

    // Grüne Kugel
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Sphere{
            radius: 0.5
        })),
        material: materials.add(Color::Srgba(LIMEGREEN)),
        transform: Transform::from_xyz(1.5, 0.5, 0.0),
        ..default()
    }).insert(Ball{});
}

fn rotate_cube(
    mut query: Query<&mut Transform, With<Cube>>,
    time: Res<Time>
) {
    for mut transform in &mut query {
        transform.rotate_y(0.5 * time.delta_seconds());
    }
}

const SPEED:f32 = 1.0;

fn move_ball(
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform),With<Ball>>,
    time: Res<Time>
){
    for mut transform in &mut query {
        let vertical: f32 = if keyboard_input.pressed(KeyCode::ArrowDown) {
            -1.
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            1.
        } else {
            0.0
        };

        transform.translation.y = transform.translation.y + vertical * time.delta_seconds() * SPEED;

    }
}
