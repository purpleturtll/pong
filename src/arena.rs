use bevy::prelude::*;
use bevy_rapier2d::{na::Isometry2, prelude::*};

pub fn create_arena(mut commands: Commands, win_desc: ResMut<WindowDescriptor>){
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