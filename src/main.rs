use bevy::prelude::*;
use bevy_third_person_camera::*;	

mod systems {
    pub mod player;
    pub mod camera;
    pub mod light;
    pub mod world;
}

use systems::{
    player::PlayerPlugin, 
    camera::CameraPlugin, 
    light::LightPlugin, 
    world::WorldPlugin,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, LightPlugin, WorldPlugin))
        .add_plugins(ThirdPersonCameraPlugin)
        .run();
}








