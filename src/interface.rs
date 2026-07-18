use crate::interface::gui::GuiPlugin;
use crate::interface::toolbox::Toolbox;
use bevy::prelude::*;

mod gui;
mod toolbox;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin).init_resource::<Toolbox>();
    }
}
