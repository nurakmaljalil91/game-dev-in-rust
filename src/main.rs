use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::{keyboard::KeyCode, Input},
};

#[derive(Component)]
enum Direction {
    Up
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera 2d
    commands.spawn_bundle(Camera2dBundle::default());

    // Circle
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    }).insert(Direction::Up);
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>
)
{
    for (mut circle, mut transform) in &mut sprite_position{
        if keyboard_input.pressed(KeyCode::A) {
            match *circle {
                Direction::Up => transform.translation.y += 10.0,

            }
        }
    }



    if keyboard_input.just_pressed(KeyCode::A){
        info!("< just pressed")
    }

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}


