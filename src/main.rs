use bevy::{
    prelude::*,
};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonState;
use std::path::Path;
use std::env;

#[derive(Resource)]
pub struct Filename(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please pass in a filename to input");
        return;
    }

    let filepath = &args[1];
    let binding: String = "assets/gltf/".to_owned() + &filepath.to_string();
    let path = Path::new(&binding);
    if !path.exists() {
        println!("File does not exist");
        return;
    }
    
    App::new()
        .init_resource::<InteractionObject>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_block)
        .add_system(mouse_button_events)
        .add_system(mouse_motion)
        .add_system(touch_events)
        .insert_resource(
            InteractionObject{
                mousedown:false,
                movement_x:0.0,
                movement_y:0.0,
                prev_touch: Vec2::new(0.0,0.0)
            }
        )
        .insert_resource(
            Filename(filepath.to_string())
        )
        .run();
}

#[derive(Component)]
pub struct ViewerObject {
    rot: f32,
    tilt: f32
}

#[derive(Resource, Default)]
pub struct InteractionObject {
    mousedown: bool,
    movement_x: f32,
    movement_y: f32,
    prev_touch: Vec2
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    filename: Res<Filename>
){
    commands
        .spawn(SceneBundle {
            scene: asset_server.load(
                "gltf/".to_owned() + &filename.0.to_string() + "#Scene0",
            ),
            ..default()
        }).insert(ViewerObject{rot: 0.0, tilt: 0.0});

    // Setup a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
}


fn touch_events(
    mut touch_evr: EventReader<TouchInput>,
    mut interact: ResMut<InteractionObject>
) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.iter() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                // println!("Touch {} started at: {:?}", ev.id, ev.position);
                interact.mousedown = true;
                interact.prev_touch.x = ev.position.x;
                interact.prev_touch.y = ev.position.y;
            }
            TouchPhase::Moved => {
                // println!("Touch {} moved to: {:?}", ev.id, ev.position);
                if interact.mousedown {
                    interact.movement_x = (ev.position.x - interact.prev_touch.x) * TOUCH_ADJUST;
                    interact.movement_y = (ev.position.y - interact.prev_touch.y) * TOUCH_ADJUST;
                    interact.prev_touch.x = ev.position.x;
                    interact.prev_touch.y = ev.position.y;
                }
            }
            TouchPhase::Ended => {
                // println!("Touch {} ended at: {:?}", ev.id, ev.position);
                interact.mousedown = false;
                interact.prev_touch.x = 0.0;
                interact.prev_touch.y = 0.0;
            }
            TouchPhase::Cancelled => {
                // println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
                interact.mousedown = false;
            }
        }
    }
}

fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut interact: ResMut<InteractionObject>
) {


    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left{
                    interact.mousedown = true;
                }
            }
            ButtonState::Released => {
                if ev.button == MouseButton::Left{
                    interact.mousedown = false;
                }
            }
        }
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut interact: ResMut<InteractionObject>
) {
    for ev in motion_evr.iter() {
        if interact.mousedown {
            interact.movement_x = ev.delta.x * MOVEMENT_ADJUST; 
            interact.movement_y = ev.delta.y * MOVEMENT_ADJUST;
        }
    
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

const ROTATION_SPEED: f32 = 0.5;
const MOVEMENT_ADJUST: f32 = 0.01;
const TOUCH_ADJUST: f32 = 0.01;

// Rotate a block
pub fn rotate_block(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut objects: Query<(&mut Transform, &mut ViewerObject)>,
    mut interact: ResMut<InteractionObject>
){
    for (mut transform, mut object) in &mut objects {
        object.rot += interact.movement_x;
        interact.movement_x = 0.0;
        object.tilt += interact.movement_y;
        interact.movement_y = 0.0;

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