use bevy::prelude::*;

pub struct LightPlugin;


impl Plugin for LightPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup,spawn);
  }
}


fn spawn(mut commands: Commands) {
  let light = PointLightBundle {
      point_light: PointLight {
          intensity: 2000.0,
          ..default()
      },
      transform: Transform::from_xyz(0.0, 5.0, 0.0),
      ..default()
  };

  commands.spawn(light);
}