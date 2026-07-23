use crate::grimoire::{GrimoireMarkRemoveNamed, components::*};
use bevy::prelude::*;
use bevy_hyper::*;
use serde::*;
use serde_json::{Result, *};

#[derive(Event)]
pub struct GrimoireSave;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GrimoireCamera {
    scale: f32,
    position: Vec3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrimoireSaveFormat {
    camera: Option<GrimoireCamera>,
    objects: Vec<GrimoireObjectProps>,
}

use crate::grimoire::components::{GrimoireObject, GrimoireObjectProps};

pub fn spawn_hyper_scene(
    msg: On<HyperReceived>,
    mut transform: Single<&mut Transform, With<Camera>>,
    mut projection: Single<&mut Projection, With<Camera>>,
    mut commands: Commands,
) {
    if let Some(json_str) = msg.0.clone() {
        let Projection::Orthographic(perspective) = projection.as_mut() else {
            return;
        };
        if let Ok(save) = _parse(json_str.as_str()) {
            if let Some(camera) = save.camera {
                transform.translation = camera.position;
                perspective.scale = camera.scale;
            }
            for obj in save.objects.clone() {
                commands.trigger(GrimoireMarkRemoveNamed(obj.name.clone()));
            }
            for obj in save.objects {
                commands.spawn_scene(make_scene(obj));
            }
        }
    };
}

fn _parse(input: &str) -> serde_json::Result<GrimoireSaveFormat> {
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
    transform: Single<&Transform, With<Camera>>,
    mut projection: Single<&mut Projection, With<Camera>>,
    query: Query<(
        &GrimoireObject,
        &Name,
        &GrimoireShape,
        &GrimoireColor,
        &GrimoirePosition,
    )>,
) {
    let Projection::Orthographic(perspective) = projection.as_mut() else {
        return;
    };
    let mut json_string = "{\"camera\":".to_string();
    let camera = GrimoireCamera {
        scale: perspective.scale,
        position: transform.translation,
    };
    json_string.push_str(serde_json::to_string(&camera).unwrap().as_str());
    json_string.push_str(",\"objects\":[");
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
    json_string.push_str("]}");
    info!("{}", json_string);
}
