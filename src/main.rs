use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::{math::*, prelude::*};

#[derive(Component)]
pub struct Attached;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::prelude::WindowPlugin {
            primary_window: Some(Window {
                title: "Super om nom".into(),
                resolution: (500., 600.).into(),
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
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, apply_force_to_attached)
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
    ));

    // A cube to move around
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),
        Attached
    ));

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
        println!("no attached found");
        return;
    };

    // mouse position
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    if let Some(cursor_world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let x_strength = 200.0;
        let y_strength = 1300.0;
        let delta_time = time.delta_seconds();

        let flipped_x = if cursor_world_pos.x > transform.translation.x { 1.0 } else { -1.0 };
        let flipped_y = if cursor_world_pos.y > transform.translation.y { 1.0 } else { -1.0 };

        linear_velocity.x += x_strength * delta_time * flipped_x;
        linear_velocity.y += y_strength * delta_time * flipped_y;

        // if object is close to mouse, set forces to 0
        let pos = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, 0.0);

        if transform.translation.distance(pos) < 20.0 {
            linear_velocity.x *= 0.1;
            linear_velocity.y *= 0.1;
        }
    }
}
