/*!
This example is about setting up a physics playground where you can throw
objects or yourself around for fun.
 */

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
#[allow(unused_imports)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{math::*, prelude::*};
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct Attached;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Super om nom".into(),
                // resolution: (500., 600.).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            apply_force_to_attached,
            bevy::window::close_on_esc,
            change_detection
        ))
        .add_plugins(DefaultPickingPlugins)

        // debug systems
        .add_plugins(WorldInspectorPlugin::new())
        // .insert_resource(DebugPickingMode::Normal)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::new(12.5, 20.0)).into(),
            material: materials.add(Color::rgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
        Collider::capsule(20.0, 12.5),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        PickableBundle::default(),
        On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
            target_commands.insert(Attached);
        }),
        Name::new("capsule")
    ));

    // A cube to move around
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(30.0, 30.0)).into(),
            material: materials.add(Color::rgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(50.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),
        PickableBundle::default(),
        On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
            target_commands.insert(Attached);
        }),
        Name::new("square")
    ));

    // Walls


    // Platforms
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(1100.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -175.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(300.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(175.0, -35.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(300.0, 25.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(300.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(-175.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(300.0, 25.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(150.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(475.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(150.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(-475.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));

    // Ramps
    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(-125.0, 80.0),
        Vector::NEG_X * 125.0,
        Vector::X * 125.0,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::rgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(-275.0, -150.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));

    let mut ramp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        Vector::new(20.0, -40.0),
        Vector::new(20.0, 40.0),
        Vector::new(-20.0, -40.0),
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::rgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(380.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));

    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn apply_force_to_attached(
    time: Res<Time>,
    mut attached: Query<(&mut LinearVelocity, &Transform), With<Attached>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((mut linear_velocity, transform)) = attached.get_single_mut() else {
        return;
    };

    // mouse position
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    if let Some(cursor_world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // F=-kx-cv
        // k is spring constantly ('stiffness')
        let kx = 20.0;
        let ky = 100.0;

        // x is distance from spring resting point
        let dist_x = cursor_world_pos.x - transform.translation.x;
        let dist_y = cursor_world_pos.y - transform.translation.y;

        let delta_time = time.delta_seconds();

        // c is the damping amount
        let c = 0.05;

        // v is object velocity
        let damp_x = linear_velocity.x;
        let damp_y = linear_velocity.y;

        let x_force = kx * dist_x;
        let y_force = ky * dist_y;

        // F=-kx-cv (I just don't use the minus)
        linear_velocity.x += (x_force * delta_time) - (c * damp_x);
        linear_velocity.y += (y_force * delta_time) - (c * damp_y);
    }
}

fn change_detection(
    mut commands: Commands,
    query: Query<(Entity, &PickSelection), Changed<PickSelection>>,
) {
    for (entity, component) in &query {
        if component.is_selected == false {
            commands.entity(entity).remove::<Attached>();
            commands.entity(entity).remove::<Pickable>();
        } else {
            commands.entity(entity).insert((Attached, Pickable::IGNORE));
        }
    }
}
