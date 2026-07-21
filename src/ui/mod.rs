use bevy::prelude::*;

pub mod gui;
pub mod toolbox;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(gui::GuiPlugin)
            .init_resource::<toolbox::Toolbox>();
    }
}
