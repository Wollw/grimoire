use self::Tools::*;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use std::slice::Iter;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tools {
    Rectangle,
    Circle,
}

impl Tools {
    pub fn interator() -> Iter<'static, Tools> {
        static TOOLS: [Tools; 2] = [Rectangle, Circle];
        TOOLS.iter()
    }
}

#[derive(Resource, Debug)]
pub struct Toolbox {
    pub tool: Tools,
}

impl Default for Toolbox {
    fn default() -> Self {
        Self {
            tool: Tools::Circle,
        }
    }
}

pub fn draw_toolbox(mut toolbox: ResMut<Toolbox>, ui: &mut Ui) {
    ui.radio_value(&mut toolbox.tool, Circle, "Circle");
    ui.radio_value(&mut toolbox.tool, Rectangle, "Rect");
}
