use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::incoming_projectile::{IncomingConfig, IncomingProjectile};

#[derive(Resource, Default)]
struct DebugPanelState {
    visible: bool,
}

pub struct DebugPanelPlugin;

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugPanelState>();
        app.add_systems(Update, (quit_on_esc, draw_egui_panel));
    }
}

fn quit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn draw_egui_panel(
    mut contexts: EguiContexts,
    projectiles: Query<&IncomingProjectile>,
    mut state: ResMut<IncomingConfig>,
    mut panel_state: ResMut<DebugPanelState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        panel_state.visible = !panel_state.visible;
    }

    if !panel_state.visible {
        return;
    }

    let num_projectile = projectiles.iter().len();
    egui::Window::new("Debug Panel")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.checkbox(&mut state.show_trajectories, "trajectories");
            ui.checkbox(&mut state.show_projectiles, "projectiles");

            ui.separator();
            let options = [8usize, 64, 128, 256, 512, 1024, 16 * 1024];
            let selected_label = state.num_target_projectiles.to_string();
            egui::ComboBox::from_label("target")
                .selected_text(&selected_label)
                .show_ui(ui, |ui| {
                    for &option in &options {
                        ui.selectable_value(
                            &mut state.num_target_projectiles,
                            option,
                            option.to_string(),
                        );
                    }
                });
            ui.label(format!("total {num_projectile}"));
        });
}
