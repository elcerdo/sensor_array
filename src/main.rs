mod incoming_projectile;

use bevy::prelude::*;
use incoming_projectile::IncomingProjectilePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(IncomingProjectilePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}