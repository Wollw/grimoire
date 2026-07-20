use bevy::prelude::*;
use bevy_hyper::*;
use serde::*;
use serde_json::{Result, *};

use crate::grimoire::{GrimoireObject, GrimoireObjectProps};

pub fn parse_hyper_json(msg: On<HyperReceived>, mut commands: Commands) {
    if let Some(json_str) = msg.0.clone() {
        match _parse(json_str.as_str()) {
            Ok(obj) => make_obj(commands, obj),
            Err(e) => error!("{:?}", e),
        }
    }
}

fn _parse(input: &str) -> serde_json::Result<GrimoireObjectProps> {
    serde_json::from_str(input)
}

fn make_obj(mut commands: Commands, props: GrimoireObjectProps) {
    let mesh = Circle::new(props.size).mesh().build();
    let position = props.position;
    let color = props.color;
    commands.spawn_scene(bsn! {
        Mesh2d(asset_value(mesh))
        @GrimoireObject {
            @name: {props.name}
            @position: {props.position},
            @color: {props.color},
        }
    });
}
