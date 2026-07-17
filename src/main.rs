use bevy::{
    app::AppLabel, camera::Viewport, ecs::schedule::*, prelude::*, state::*, tasks::IoTaskPool,
    window::PrimaryWindow,
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use egui::{LayerId, Ui, UiBuilder};
use serde::{Deserialize, Serialize};

mod server;
use crate::server::*;

fn main() {
    info!("Starting Application");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .insert_state(ServerState::StopServer)
        .add_systems(OnEnter(ServerState::RunServer), setup_actix)
        .add_systems(OnExit(ServerState::RunServer), handle_stop_actix)
        .add_systems(Update, handle_actix_rx)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ui_example_system(
    server_state: Res<State<ServerState>>,
    mut commands: Commands,
    mut contexts: EguiContexts,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    let mut panel_open = true;
    egui::Panel::left("left_panel")
        .resizable(true)
        .show_collapsible(&mut viewport_ui, &mut panel_open, |ui| {
            let mut b = *server_state == ServerState::RunServer;
            if ui.toggle_value(&mut b, "Server").clicked() {
                match server_state.get() {
                    ServerState::StopServer => commands.set_state(ServerState::RunServer),
                    ServerState::RunServer => commands.set_state(ServerState::StopServer),
                }
            }
        });
    egui::CentralPanel::default().show(&mut viewport_ui, |_ui| {});

    Ok(())
}
