use self::Tools::*;
use crate::grimoire::components::*;
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
        static TOOLS: [Tools; 2] = [Tools::Rectangle, Tools::Circle];
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
    ui.radio_value(&mut toolbox.tool, Tools::Circle, "Circle");
    ui.radio_value(&mut toolbox.tool, Tools::Rectangle, "Rect");
}

pub fn shape_change(ui: &mut Ui, shape: GrimoireShape) -> (GrimoireShape, GrimoireRedraw) {
    let mut shape_type = match shape {
        GrimoireShape::Circle { radius: _radius } => Tools::Circle,
        GrimoireShape::Rect {
            width: _width,
            height: _height,
        } => Tools::Rectangle,
        _ => Tools::Circle,
    };

    if ui
        .radio_value(&mut shape_type, Tools::Circle, "Circle")
        .clicked()
    {
        return (GrimoireShape::Circle { radius: 30. }, GrimoireRedraw(true));
    }
    if ui
        .radio_value(&mut shape_type, Tools::Rectangle, "Rect")
        .clicked()
    {
        return (
            GrimoireShape::Rect {
                width: 20.,
                height: 20.,
            },
            GrimoireRedraw(true),
        );
    }
    return (shape, GrimoireRedraw(false));
}
