use std::any::Any;
use std::ops::Deref;

use bevy::asset::RenderAssetUsages;
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
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_pancam::*;
use bevy_procedural_meshes::*;
use egui::MouseWheelUnit;
use egui::PointerButton::Primary;
use egui::accesskit::Point;
use egui::{LayerId, Ui, UiBuilder};
use serde::{Deserialize, Serialize};

mod grimoire;
mod interface_plugin;
use crate::grimoire::*;
use crate::interface_plugin::*;
use crate::parse_json::*;
mod parse_json;
use bevy_hyper::*;

fn main() {
    info!("Starting Application");

    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, PanCamPlugin::default()))
        .add_plugins(InterfacePlugin)
        .add_plugins(HyperPlugin::default())
        .insert_resource(MeshHandleRes(None))
        .insert_resource(CursorWorldPos(None))
        .add_systems(Startup, setup)
        .add_systems(Update, get_cursor_world_pos)
        .add_systems(Update, grimoire_draw)
        .add_observer(parse_hyper_json)
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

#[derive(Component, Debug, Clone, Default)]
struct NowDrawing;

#[derive(Debug, Resource)]
struct CursorWorldPos(Option<Vec2>);

fn get_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
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

#[derive(Resource)]
struct MeshHandleRes(Option<Handle<Mesh>>);

// Main draw system for the canvas
fn grimoire_draw(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mesh_handle_res: ResMut<MeshHandleRes>,
    query: Query<(&GrimoireShape, &GrimoireColor, &GrimoirePosition, Entity)>,
    mut commands: Commands,
) {
    for (shape, GrimoireColor(color), position, entity) in &query {
        //todo: we don't *really* want to be recreating these resources every frame
        if let Some(mesh) = match shape {
            GrimoireShape::Circle { radius } => Some(meshes.add(Circle::new(*radius))),
            GrimoireShape::Ellipse { half_size } => {
                Some(meshes.add(Ellipse::new(half_size.x, half_size.y)))
            }
            GrimoireShape::Rect { width, height } => {
                Some(meshes.add(Rectangle::new(*width, *height)))
            }
            GrimoireShape::Polygon { vertex_groups } => {
                let mut mesh = PMesh::<u32>::new();
                for vs in vertex_groups {
                    mesh.fill(0.1, |builder| {
                        let mut vs = vs.clone();
                        if let Some(v) = vs.pop() {
                            builder.begin(v);
                        } else {
                            return;
                        }
                        for v in vs {
                            builder.line_to(v);
                        }
                        builder.close();
                    });
                }
                Some(meshes.add(mesh.to_bevy(RenderAssetUsages::all())))
            }
            _ => None,
        } {
            commands.entity(entity).insert(Mesh2d(mesh));
            commands
                .entity(entity)
                .insert(MeshMaterial2d(materials.add(*color)));
            commands
                .entity(entity)
                .insert(Transform::from_xyz((*position).x, (*position).y, 0.));
        }
    }
}
