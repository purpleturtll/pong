use bevy::prelude::*;
use bevy_rapier2d::{na::Isometry2, prelude::*};

struct Ball;

/// System for creating the main ball
pub fn create_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
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