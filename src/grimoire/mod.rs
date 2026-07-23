use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::reflect::array::Array;
use bevy::sprite_render::*;
use bevy_pancam::*;
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
pub mod render;

pub struct GrimoirePlugin;

#[derive(Debug, Resource)]
pub struct CursorWorldPos(pub Option<Vec2>);

impl Plugin for GrimoirePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(render::MeshHandleRes(None))
            .insert_resource(CursorWorldPos(None))
            .insert_resource(IsOverOrOut::Out)
            .add_observer(draw::draw_new_shape)
            .add_observer(parse_json::save)
            .add_observer(clear_named)
            .add_observer(clear)
            .add_systems(Update, remove_marked);
    }
}

#[derive(Event)]
pub struct GrimoireClear;

pub fn clear(
    _event: On<GrimoireClear>,
    mut commands: Commands,
    query: Query<Entity, With<GrimoireObject>>,
) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

#[derive(Event, Debug)]
pub struct GrimoireMarkRemoveNamed(String);

#[derive(Component, Debug)]
pub struct GrimoireRemove;

pub fn clear_named(
    clear_named: On<GrimoireMarkRemoveNamed>,
    mut commands: Commands,
    query: Query<(Entity, &Name), With<GrimoireObject>>,
) {
    let name = (*clear_named).0.to_string();
    for (entity, entity_name) in query.clone() {
        if entity_name.as_str() == name.as_str() {
            commands.entity(entity).insert(GrimoireRemove);
        }
    }
}

fn remove_marked(query: Query<Entity, With<GrimoireRemove>>, mut commands: Commands) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
