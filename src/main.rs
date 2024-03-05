use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        // ...your other plugins, systems and resources
        .run();
}
