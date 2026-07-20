use crate::interface_plugin::gui::GuiPlugin;
use crate::interface_plugin::toolbox::Toolbox;
use bevy::prelude::*;

mod gui;
mod toolbox;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin).init_resource::<Toolbox>();
    }
}
