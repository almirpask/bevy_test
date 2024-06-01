use bevy::prelude::*;
use bevy_third_person_camera::*;


pub struct CameraPlugin;


impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup,spawn);
  }
}


fn spawn(mut commands: Commands) {
  let camera =  (
    ThirdPersonCamera::default(),
    Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    }
    
  );
  

  commands.spawn(camera);
}   