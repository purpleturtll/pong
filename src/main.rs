use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod ball;
mod arena;

use camera::create_camera;
use ball::create_ball;
use arena::create_arena;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            height: 400.0,
            width:800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(create_camera.system())
        .add_startup_system(create_ball.system())
        .add_startup_system(create_arena.system())
        .run();
}
