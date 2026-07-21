mod grimoire;
mod ui;
use crate::grimoire::*;
use crate::ui::*;

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
use bevy_color::palettes::css::WHITE;
use bevy_hyper::*;
use bevy_pancam::*;
use egui::MouseWheelUnit;
use egui::PointerButton::Primary;
use egui::accesskit::Point;
use egui::{LayerId, Ui, UiBuilder};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::ops::Deref;

fn main() {
    info!("Starting Application");

    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, PanCamPlugin::default()))
        .add_plugins(GrimoirePlugin)
        .add_plugins(InterfacePlugin)
        .add_plugins(HyperPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, get_cursor_world_pos)
        .add_systems(Update, draw::grimoire_draw)
        .add_observer(parse_json::spawn_hyper_scene)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        move_keys: DirectionKeys::NONE,
        ..default()
    });

    commands.spawn_scene_list(bsn_list! {
        @GrimoireObject {
            @name: "foo",
            @shape: GrimoireShape::Rect{width:50.,height:50.}
        },
        @GrimoireObject {
            @name: "bar",
            @color: WHITE,
            @position: Vec3::new(-100.,50.,0.),
        },
        @GrimoireObject {
            @name: "baz",
            @position: Vec3::new(100.,0.,0.),
        }
        on(|event:On<Pointer<Over>>| info!("Over"))

    });
}

fn get_cursor_world_pos(
    mut cursor_world_pos: ResMut<grimoire::CursorWorldPos>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (main_camera, main_camera_transform) = *q_camera;
    // Get the cursor position in the world
    cursor_world_pos.0 = primary_window.cursor_position().and_then(|cursor_pos| {
        main_camera
            .viewport_to_world_2d(main_camera_transform, cursor_pos)
            .ok()
    });
}
