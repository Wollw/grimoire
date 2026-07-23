use std::range::RangeInclusive;

use crate::{
    grimoire::{
        GrimoireClear, GrimoireColor, GrimoireObjectProps, GrimoirePosition,
        components::{GrimoireObject, GrimoireRedraw, GrimoireShape},
        parse_json::GrimoireSave,
    },
    ui::{IsWindowFocused, color32_to_grim_color, grim_color_to_color32, toolbox::*},
};
use bevy::color::{palettes::basic::*, prelude::*};
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

fn setup_gui(mut commands: Commands) {
    commands.insert_resource(Gui { toolbox_open: true });
}

pub fn gui_system(
    mut egui_contexts: EguiContexts,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut gui: ResMut<Gui>,
    mut toolbox: ResMut<Toolbox>,
    window_focused: Res<IsWindowFocused>,
    //server_state: Res<State<ServerState>>,
    mut query: Query<(
        Entity,
        &GrimoireObject,
        &mut Name,
        &mut GrimoireShape,
        &mut GrimoireRedraw,
        &mut GrimoireColor,
        &mut GrimoirePosition,
    )>,
    mut commands: Commands,
) -> Result {
    let IsWindowFocused(focused) = *window_focused;
    if !focused {
        return Ok(());
    }

    let ctx = egui_contexts.ctx_mut()?;

    egui::Window::new("Toolbox").scroll(true).show(ctx, |ui| {
        if ui.button("Save").clicked() {
            commands.trigger(GrimoireSave);
        }
        if ui.button("Clear").clicked() {
            commands.trigger(GrimoireClear);
        }

        draw_toolbox(toolbox, ui);
        for (entity, _, mut n, mut shape, mut redraw, mut color, mut position) in query {
            let mut name = String::from(n.as_str());
            egui::CollapsingHeader::new(entity.to_string())
                .default_open(false)
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut name));
                    (*shape, *redraw) = shape_change(ui, shape.clone());
                    let mut rgba = grim_color_to_color32(color.clone());
                    if ui.color_edit_button_srgba(&mut rgba).changed() {
                        *redraw = GrimoireRedraw(true);
                        *color = color32_to_grim_color(rgba);
                    }
                    let mut z_index = position.z as u32;
                    if ui
                        .add(egui::Slider::new(&mut z_index, 1..=100).text("z-index"))
                        .changed()
                    {
                        *redraw = GrimoireRedraw(true);
                        position.z = z_index as f32;
                    }
                });
            n.set(name);
        }
    });

    Ok(())
}
