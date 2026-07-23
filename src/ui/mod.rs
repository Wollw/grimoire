use bevy::{prelude::*, window::WindowFocused};

use crate::grimoire::GrimoireColor;

pub mod gui;
pub mod toolbox;

pub struct GrimoireInterfacePlugin;

#[derive(Resource)]
pub struct IsWindowFocused(bool);

impl Plugin for GrimoireInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(gui::GuiPlugin)
            .insert_resource(IsWindowFocused(true))
            .add_systems(
                Update,
                |mut focused: ResMut<IsWindowFocused>,
                 mut messages: MessageReader<WindowFocused>| {
                    for message in messages.read() {
                        *focused = IsWindowFocused(message.focused);
                    }
                },
            )
            .init_resource::<toolbox::Toolbox>();
    }
}

pub fn grim_color_to_color32(grim_color: GrimoireColor) -> egui::Color32 {
    let GrimoireColor(color) = grim_color;
    egui::Color32::from_rgba_unmultiplied(
        (color.to_srgba().red * 255.) as u8,
        (color.to_srgba().green * 255.) as u8,
        (color.to_srgba().blue * 255.) as u8,
        (color.to_srgba().alpha * 255.) as u8,
    )
}

pub fn color32_to_grim_color(rgba: egui::Color32) -> GrimoireColor {
    GrimoireColor(Color::srgba(
        ((rgba.r()) as f32) / 255.,
        ((rgba.g()) as f32) / 255.,
        ((rgba.b()) as f32) / 255.,
        ((rgba.a()) as f32) / 255.,
    ))
}
