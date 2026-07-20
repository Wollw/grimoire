use bevy::prelude::Resource;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Rectangle,
    Circle,
}

#[derive(Resource)]
pub struct Toolbox {
    pub tool: Tool,
}

impl Default for Toolbox {
    fn default() -> Self {
        Self { tool: Tool::Circle }
    }
}
