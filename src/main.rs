mod debug_panel;
mod incoming_projectile;
mod sink_and_source;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use debug_panel::DebugPanelPlugin;
use incoming_projectile::IncomingProjectilePlugin;
use sink_and_source::SinkAndSourcePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .add_plugins(IncomingProjectilePlugin)
        .add_plugins(DebugPanelPlugin)
        .add_plugins(SinkAndSourcePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}