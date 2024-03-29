/*!
This example is about setting up a physics playground where you can throw
objects or yourself around for fun.
 */

use bevy::asset::AssetMetaCheck;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{PrimaryWindow, WindowResolution};
#[allow(unused_imports)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{math::*, prelude::*};
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct Attached;

#[derive(Component)]
struct OmNom;

#[derive(Resource)]
struct PointerLocation(Vec2);

pub fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Super om nom".into(),
                resolution: WindowResolution::new(1280.0, 720.0),
                canvas: Some("#bevy".to_string()),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(PointerLocation(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            bevy::window::close_on_esc,
            (store_pointer_location,
             apply_force_to_attached).chain(),
            draw_line_to_attached
        ))
        .add_plugins(DefaultPickingPlugins
            .build()
            .disable::<DefaultHighlightingPlugin>())

        // debug systems
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(PhysicsDebugPlugin::default())
        // .insert_resource(DebugPickingMode::Normal)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // region background
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            texture: asset_server.load("playground_background.png"),
            ..default()
        },
        Pickable::IGNORE,
        Name::new("background")
    ));
    //endregion

    // region Player
    let player_size = 40.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(player_size)),
                ..default()
            },
            texture: asset_server.load("om_nom.png"),
            transform: Transform::from_xyz(500.0, 120.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.1),
        Collider::rectangle(player_size, player_size),
        Name::new("player"),
        OmNom
    ));
    //endregion

    //region objects
    for i in 1..10 {
        // heavy capsule
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Capsule2d::new(25.0, 40.0)).into(),
                material: materials.add(Color::rgb(0.31, 0.54, 0.98)),
                transform: Transform::from_xyz(80.0 + (i as f32), 0.0, 0.0),
                ..default()
            },
            Friction::new(0.05).with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(10.0),
            GravityScale(1.5),
            Collider::capsule(40.0, 25.0),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            PickableBundle::default(),
            On::<Pointer<DragStart>>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(Attached);
            }),
            On::<Pointer<DragEnd>>::target_commands_mut(|_click, target_commands| {
                target_commands.remove::<Attached>();
            }),
            Name::new("heavy capsule")
        ));

        // light cube
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(30.0, 30.0)).into(),
                material: materials.add(Color::rgb(0.35, 0.69, 0.99)),
                transform: Transform::from_xyz(-50.0, 100.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Friction::new(0.05).with_combine_rule(CoefficientCombine::Min),
            Collider::rectangle(30.0, 30.0),
            PickableBundle::default(),
            ColliderDensity(0.5),
            On::<Pointer<DragStart>>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(Attached);
            }),
            On::<Pointer<DragEnd>>::target_commands_mut(|_click, target_commands| {
                target_commands.remove::<Attached>();
            }),
            Name::new("light square"),
            LockedAxes::ROTATION_LOCKED,
        ));
    }
    //endregion

    //region static
    // Walls
    let width_wall = 50.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(1145.0, width_wall)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -350.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1145.0, width_wall),
        Name::new("Floor")
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(1145.0, width_wall)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 365.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1145.0, width_wall),
        Name::new("Ceiling")
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(width_wall,  741.0)),
                ..default()
            },
            transform: Transform::from_xyz(-595.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(width_wall, 741.0),
        Name::new("Left wall")
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(width_wall,  741.0)),
                ..default()
            },
            transform: Transform::from_xyz(595.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(width_wall, 741.0),
        Name::new("Left wall")
    ));

    // Buckets
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(25.0, 160.0)),
                ..default()
            },
            transform: Transform::from_xyz(-350.0, -246.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(25.0, 160.0),
        Name::new("bucket left")
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(25.0, 160.0)),
                ..default()
            },
            transform: Transform::from_xyz(350.0, -246.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(25.0, 160.0),
        Name::new("bucket left")
    ));

    // Platforms
    let position_x = 500.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(150.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(position_x, 80.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(150.0, 25.0),
        Name::new("platform right")
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::SILVER,
                custom_size: Some(Vec2::new(150.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(-position_x, 80.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(150.0, 25.0),
        Name::new("platform left")
    ));

    // monkey bars
    let handle_size = 40.0;
    let start_x = -220.0;
    let increment_x = 150.0;
    for i in 0..4 {
        let x = start_x + (increment_x * i as f32);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::TOMATO,
                    custom_size: Some(Vec2::splat(handle_size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 215.0, 0.0),
                ..default()
            },
            RigidBody::Static,
            ColliderDensity(10.0),
            Collider::rectangle(handle_size, handle_size),
            Name::new(format!("monkey bar {}", i)),
            PickableBundle::default(),
            On::<Pointer<DragStart>>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(Attached);
            }),
            On::<Pointer<DragEnd>>::target_commands_mut(|_click, target_commands| {
                target_commands.remove::<Attached>();
            }),
        ));
    }
    //endregion

    // Camera
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedHorizontal(1280.0);
    commands.spawn(camera_bundle);
}

fn apply_force_to_attached(
    time: Res<Time>,
    mut attached: Query<(&mut LinearVelocity, &Transform, &ColliderDensity), With<Attached>>,
    mut om_nom: Query<&mut LinearVelocity, (With<OmNom>, Without<Attached>)>,
    pointer_location: Res<PointerLocation>
) {
    let Ok((mut linear_velocity, transform, collider_density)) = attached.get_single_mut() else {
        return;
    };

    let Ok(mut lv_om_nom) = om_nom.get_single_mut() else {
        panic!("no om nom")
    };
    // F=-kx-cv
    // k is spring constantly ('stiffness')
    let kx = 20.0;
    let ky = 100.0;

    // x is distance from spring resting point
    let dist_x = pointer_location.0.x - transform.translation.x;
    let dist_y = pointer_location.0.y - transform.translation.y;

    let delta_time = time.delta_seconds();

    // c is the damping amount
    let c = 0.05;

    // v is object velocity
    let damp_x = linear_velocity.x;
    let damp_y = linear_velocity.y;

    let hookes_x = kx * dist_x;
    let hookes_y = ky * dist_y;

    // F=-kx-cv (I just don't use the minus)
    let force_x = (hookes_x * delta_time) - (c * damp_x);
    let force_y = (hookes_y * delta_time) - (c * damp_y);

    // F=ma, so object acceleration = F/m
    linear_velocity.x += force_x / collider_density.0;
    linear_velocity.y += force_y / collider_density.0;

    // apply a fraction of the force back to om nom
    let force_const = 0.1;
    let force_max_x = 15.0;
    let force_max_y = 20.0;

    let mut omnom_force_x = -force_const * force_x * collider_density.0;
    if omnom_force_x.abs() > force_max_x {
        omnom_force_x = force_max_x * omnom_force_x.signum()
    }

    lv_om_nom.x += omnom_force_x;

    let mut omnom_force_y = -force_const * force_y * collider_density.0;
    if omnom_force_y.abs() > force_max_y {
        omnom_force_y = force_max_y * omnom_force_y.signum()
    }
    lv_om_nom.y += omnom_force_y;
}

fn draw_line_to_attached(
    mut gizmos: Gizmos,
    om_nom: Query<&Transform, With<OmNom>>,
    attached: Query<&Transform, With<Attached>>,
) {
    let Ok(attached) = attached.get_single() else {
        return;
    };

    let Ok(om_nom) = om_nom.get_single() else {
        panic!("no om nom!")
    };

    let pos_om_nom = Vec2::new(om_nom.translation.x, om_nom.translation.y);
    let pos_attached = Vec2::new(attached.translation.x, attached.translation.y);
    gizmos.line_2d(pos_om_nom, pos_attached, Color::RED);
}

fn store_pointer_location(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut touch_events: EventReader<TouchInput>,
    mut pointer_location: ResMut<PointerLocation>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();

    for event in touch_events.read() {
        if event.phase == TouchPhase::Moved {
            let Some(pos) = camera.viewport_to_world_2d(camera_transform, event.position) else {
                return
            };
            pointer_location.0 = pos;
            return
        }
    }

    // we'll only get to here if there's no touch events
    let window = windows.single();
    let Some(cursor_world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else {
        return
    };
    pointer_location.0 = cursor_world_pos;
}