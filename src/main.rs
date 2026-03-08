mod debug_panel;
mod incoming_projectile;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use debug_panel::DebugPanelPlugin;
use incoming_projectile::IncomingProjectilePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .add_plugins(IncomingProjectilePlugin)
        .add_plugins(DebugPanelPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}