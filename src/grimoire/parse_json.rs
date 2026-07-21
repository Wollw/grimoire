use bevy::prelude::*;
use bevy_hyper::*;
use serde::*;
use serde_json::{Result, *};

use crate::grimoire::components::{GrimoireObject, GrimoireObjectProps};

pub fn spawn_hyper_scene(msg: On<HyperReceived>, mut commands: Commands) {
    if let Some(json_str) = msg.0.clone() {
        match _parse(json_str.as_str()) {
            Ok(obj) => {
                commands.spawn_scene(make_scene(obj));
            }
            Err(e) => {
                error!("{:?}", e);
            }
        };
    };
}

fn _parse(input: &str) -> serde_json::Result<GrimoireObjectProps> {
    serde_json::from_str(input)
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
