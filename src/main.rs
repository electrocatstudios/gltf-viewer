use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_block)
        .run();
}

#[derive(Component)]
pub struct ViewerObject {
    rot: f32,
    tilt: f32
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands
        .spawn(SceneBundle {
            scene: asset_server.load(
                "gltf/puck.gltf#Scene0",
            ),
            ..default()
        }).insert(ViewerObject{rot: 0.0, tilt: 0.0});

    // Setup a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
}


const ROTATION_SPEED: f32 = 0.5;

// Rotate a block
pub fn rotate_block(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut objects: Query<(&mut Transform, &mut ViewerObject)>,
){
    for (mut transform, mut object) in &mut objects {
        if keys.pressed(KeyCode::D) {
            object.rot += ROTATION_SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            object.rot -= ROTATION_SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::W) {
            object.tilt -= ROTATION_SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            object.tilt += ROTATION_SPEED * time.delta_seconds();
        }
        if object.tilt > std::f32::consts::PI {
            object.tilt = std::f32::consts::PI;
        }
        if object.tilt < -std::f32::consts::PI {
            object.tilt = -std::f32::consts::PI;
        }

        if object.rot < 0.0 {
            object.rot += std::f32::consts::PI * 2.0;
        }
        
        if object.rot > std::f32::consts::PI * 2.0 {
            object.rot -=  std::f32::consts::PI * 2.0;
        }
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            object.tilt,
            object.rot,
            0.0,
        );
        // transform.rotation = Quat::from_rotation_y(object.rot); // 0
    }
}