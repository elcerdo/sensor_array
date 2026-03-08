use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::incoming_projectile::{IncomingProjectile, IncomingState};

pub struct DebugPanelPlugin;

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_egui_panel);
    }
}

fn draw_egui_panel(
    mut contexts: EguiContexts,
    projectiles: Query<&IncomingProjectile>,
    mut state: ResMut<IncomingState>,
) {
    let num_projectile = projectiles.iter().len();
    egui::SidePanel::right("debug_panel")
        .default_width(260.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Debug Panel");
            ui.separator();
            ui.label(format!("Total projectiles: {num_projectile}"));
            ui.separator();
            ui.checkbox(&mut state.show_trajectories, "Show trajectories");
            ui.checkbox(&mut state.show_blast_radii, "Show blast radii");
        });
}
