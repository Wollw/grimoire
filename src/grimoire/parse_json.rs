use crate::grimoire::components::*;
use bevy::prelude::*;
use bevy_hyper::*;
use serde::*;
use serde_json::{Result, *};

#[derive(Event)]
pub struct GrimoireSave;

use crate::grimoire::components::{GrimoireObject, GrimoireObjectProps};

pub fn spawn_hyper_scene(msg: On<HyperReceived>, mut commands: Commands) {
    if let Some(json_str) = msg.0.clone() {
        if let Ok(objs) = _parse(json_str.as_str()) {
            for obj in objs {
                commands.spawn_scene(make_scene(obj));
            }
        }
    };
}

fn _parse(input: &str) -> serde_json::Result<Vec<GrimoireObjectProps>> {
    match serde_json::from_str(input) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

fn make_scene(grim_obj: GrimoireObjectProps) -> impl Scene {
    bsn! {
            @GrimoireObject {
                @name: {grim_obj.name},
                @color: {grim_obj.color},
                @position: {grim_obj.position},
                @shape: {grim_obj.shape}
            }
    }
}

pub fn save(
    _save_event: On<GrimoireSave>,
    query: Query<(
        &GrimoireObject,
        &Name,
        &GrimoireShape,
        &GrimoireColor,
        &GrimoirePosition,
    )>,
) {
    let mut json_string = "[".to_string();
    for (_, name, shape, color, position) in query {
        let shape = shape.clone();
        let grim_obj_props = GrimoireObjectProps {
            name: name.to_string(),
            color: color.0,
            position: Vec3::new(position.x, position.y, position.z),
            shape: shape,
        };
        json_string.push_str(serde_json::to_string(&grim_obj_props).unwrap().as_str());
        json_string.push_str(",");
    }
    json_string.pop();
    json_string.push_str("]");
    info!("{}", json_string);
}
