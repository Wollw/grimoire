use crate::{interface::toolbox::*, server::ServerState};
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Frame, Layout, Ui},
};
use egui::{LayerId, UiBuilder};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup_gui)
            .add_systems(EguiPrimaryContextPass, gui_system);
    }
}

#[derive(Resource)]
pub struct Gui {
    toolbox_open: bool,
}

fn setup_gui(mut commands: Commands, mut egui_contexts: EguiContexts) {
    commands.insert_resource(Gui { toolbox_open: true });
}

pub fn gui_system(
    mut egui_contexts: EguiContexts,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut gui: ResMut<Gui>,
    mut toolbox: ResMut<Toolbox>,
    mut commands: Commands,
    server_state: Res<State<ServerState>>,
) -> Result {
    let ctx = egui_contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    egui::Panel::left("left_panel")
        .resizable(true)
        .show_collapsible(&mut viewport_ui, &mut gui.toolbox_open, |ui| {
            let mut b = *server_state == ServerState::RunServer;
            if ui.toggle_value(&mut b, "Server").clicked() {
                match server_state.get() {
                    ServerState::StopServer => commands.set_state(ServerState::RunServer),
                    ServerState::RunServer => commands.set_state(ServerState::StopServer),
                }
            }
        });
    /*
    egui::CentralPanel::default().show(&mut viewport_ui, |ui| {
        if ui.toggle_value(&mut gui.toolbox_open, "Panel").clicked() {}
    });
    */
    Ok(())
}
