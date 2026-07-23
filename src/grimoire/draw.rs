use crate::{
    grimoire::{self, components::*},
    ui::toolbox::*,
};
use bevy::prelude::*;
use bevy_egui::{input::EguiWantsInput, prelude::*};

pub fn draw_new_shape(
    click: On<Pointer<Click>>,
    world_pos: Res<grimoire::CursorWorldPos>,
    is_over_out: Res<grimoire::IsOverOrOut>,
    toolbox: Res<Toolbox>,
    mut commands: Commands,
    egui_wants_input: Res<EguiWantsInput>,
) {
    if egui_wants_input.wants_any_pointer_input() {
        return;
    }
    if *is_over_out == IsOverOrOut::Over {
        return;
    }
    if click.button != PointerButton::Primary {
        return;
    }
    if let Some(pos) = world_pos.0 {
        let shape = match toolbox.tool {
            Tools::Circle => GrimoireShape::Circle { radius: 25. },
            Tools::Rectangle => GrimoireShape::Rect {
                width: 20.,
                height: 20.,
            },
        };
        commands.spawn_scene(bsn! {
                @GrimoireObject {
                @name: "new_circle",
                @position: Vec3::new(pos.x,pos.y,0.),
                @shape: shape
                }
        });
    }
}
