use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{na::Isometry2, prelude::*};

struct Ball;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(ball_setup.system())
        .add_startup_system(bounds_setup.system())
        .run();
}

fn ball_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_desc: Res<WindowDescriptor>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("ball.png");

    let ball_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(-300.0, -300.0).into(),
            ..Default::default()
        },
        forces: RigidBodyForces {
            gravity_scale: 0.0,
            ..Default::default()
        },
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ..Default::default()
        },
        damping: RigidBodyDamping {
            linear_damping: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let ball_sprite = SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    };

    let ball_collider = ColliderBundle {
        collider_type: ColliderType::Solid,
        mass_properties: ColliderMassProps::Density(1.0),
        material: ColliderMaterial::new(0.0, 1.0),
        shape: ColliderShape::cuboid(16.0, 16.0),
        position: Isometry2::new([0.0, 0.0].into(), 0.0).into(),
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(ball_sprite)
        .insert_bundle(ball_collider)
        .insert_bundle(ball_body)
        .insert(ColliderPositionSync::Discrete)
        .insert(Ball);
}

fn bounds_setup(mut commands: Commands, win_desc: ResMut<WindowDescriptor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let bottom_collider = ColliderBundle {
        collider_type: ColliderType::Solid,
        shape: ColliderShape::cuboid(win_desc.width / 2.0, 8.0),
        material: ColliderMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        },
        position: Isometry2::new([0.0, -win_desc.height / 2.0 - 8.0].into(), 0.0).into(),
        ..Default::default()
    };

    let bottom_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(bottom_collider)
        .insert_bundle(bottom_body)
        .insert(ColliderPositionSync::Discrete);

    let top_collider = ColliderBundle {
        collider_type: ColliderType::Solid,
        shape: ColliderShape::cuboid(win_desc.width / 2.0, 8.0),
        material: ColliderMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        },
        position: Isometry2::new([0.0, win_desc.height / 2.0 + 8.0].into(), 0.0).into(),
        ..Default::default()
    };

    let top_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(top_collider)
        .insert_bundle(top_body)
        .insert(ColliderPositionSync::Discrete);

    let left_collider = ColliderBundle {
        collider_type: ColliderType::Solid,
        shape: ColliderShape::cuboid(8.0, win_desc.height / 2.0),
        material: ColliderMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        },
        position: Isometry2::new([-win_desc.width / 2.0 - 8.0, 0.0].into(), 0.0).into(),
        ..Default::default()
    };

    let left_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(left_collider)
        .insert_bundle(left_body)
        .insert(ColliderPositionSync::Discrete);

    let right_collider = ColliderBundle {
        collider_type: ColliderType::Solid,
        shape: ColliderShape::cuboid(8.0, win_desc.height),
        material: ColliderMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        },
        position: Isometry2::new([win_desc.width / 2.0 + 8.0, 0.0].into(), 0.0).into(),
        ..Default::default()
    };

    let right_body = RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(right_collider)
        .insert_bundle(right_body)
        .insert(ColliderPositionSync::Discrete);
}
