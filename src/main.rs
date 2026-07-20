use std::any::Any;
use std::ops::Deref;

use bevy::dev_tools::infinite_grid::*;
use bevy::input::mouse::{AccumulatedMouseScroll, MouseWheel};
use bevy::reflect::DynamicTyped;
use bevy::{
    app::AppLabel,
    camera::{Projection, Viewport},
    ecs::schedule::*,
    picking::pointer::PointerInteraction,
    prelude::*,
    sprite_render::Wireframe2dConfig,
    state::*,
    tasks::IoTaskPool,
    window::PrimaryWindow,
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_pancam::*;
use egui::MouseWheelUnit;
use egui::PointerButton::Primary;
use egui::accesskit::Point;
use egui::{LayerId, Ui, UiBuilder};
use serde::{Deserialize, Serialize};

mod grimoire;
mod interface_plugin;
use crate::drag_plugin::drag_plugin;
use crate::grimoire::*;
use crate::interface_plugin::*;
use crate::parse_json::*;
mod parse_json;
use bevy_hyper::*;

mod drag_plugin;

fn main() {
    info!("Starting Application");


    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, PanCamPlugin::default()))
        .add_plugins(InterfacePlugin)
        .add_plugins(HyperPlugin::default())
        .add_plugins(drag_plugin)
        .add_systems(Startup, setup)
        .add_observer(parse_hyper_json)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        move_keys: DirectionKeys::NONE,
        ..default()
    });
    commands.spawn_scene_list(my_scene());
}

fn my_scene() -> impl SceneList {
    bsn_list! [
        @GrimoireObject {
            @name: "foo",
        }
        Mesh2d(asset_value(Circle::new(50.).mesh().build())),
        @GrimoireObject {
            @name: "bar",
            @color : Color::hsl(0.0, 1.0, 0.5),
            @position: Vec3::new(0.0,10.0,-1000.0)
        }
        Mesh2d(asset_value(Circle::new(50.).mesh().build())),
    ]
}
