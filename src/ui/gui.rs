use crate::{
    grimoire::{
        GrimoireColor, GrimoireObjectProps, GrimoirePosition,
        components::{GrimoireObject, GrimoireRedraw, GrimoireShape},
        parse_json::GrimoireSave,
    },
    ui::toolbox::*,
};
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Frame, Layout, Ui},
};
use egui::{CollapsingResponse, LayerId, UiBuilder};

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
    //server_state: Res<State<ServerState>>,
    mut query: Query<(
        &GrimoireObject,
        &mut Name,
        &mut GrimoireShape,
        &mut GrimoireRedraw,
        &mut GrimoireColor,
        &mut GrimoirePosition,
    )>,
    mut commands: Commands,
) -> Result {
    let ctx = egui_contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    gui.toolbox_open = egui::CollapsingHeader::new("Toolbox")
        .show_background(true)
        .show(&mut viewport_ui, |ui| {
            if ui.button("Save").clicked() {
                commands.trigger(GrimoireSave);
            }

            draw_toolbox(toolbox, ui);
            for (_, mut n, mut shape, mut redraw, color, position) in query {
                let mut name = String::from(n.as_str());
                egui::CollapsingHeader::new(n.as_str())
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(egui::TextEdit::singleline(&mut name));
                        *shape = shape_change(ui, shape.clone(), &mut redraw)
                    });
                n.set(name);
            }
        })
        .fully_open();

    Ok(())
}
