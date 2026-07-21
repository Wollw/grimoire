use bevy::color::palettes::css::*;
use bevy::dev_tools::infinite_grid::*;
use bevy::pbr::StandardMaterialUniform;
use bevy::prelude::*;
use bevy::reflect::array::Array;
use bevy::sprite_render::*;
use bevy_color;
use bevy_pancam::*;
use bevy_prototype_lyon::prelude::tess::geom::arrayvec::ArrayVec;
use bevy_prototype_lyon::prelude::tess::path::Position;
use bevy_scene::prelude::*;
use bevy_scene::{ResolveContext, ResolveSceneError, ResolvedScene};
use serde;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub use crate::components::*;
pub mod components;
pub mod draw;
pub mod parse_json;

pub struct GrimoirePlugin;

#[derive(Debug, Resource)]
pub struct CursorWorldPos(pub Option<Vec2>);

impl Plugin for GrimoirePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(draw::MeshHandleRes(None))
            .insert_resource(CursorWorldPos(None));
    }
}
