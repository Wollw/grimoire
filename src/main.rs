use std::ops::Deref;

use bevy::input::mouse::{AccumulatedMouseScroll, MouseWheel};
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

mod interface;
mod server;
use crate::interface::*;
use crate::server::*;

fn main() {
    info!("Starting Application");

    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, PanCamPlugin::default()))
        .add_plugins(InterfacePlugin)
        .add_systems(Startup, setup)
        // Server Handling
        .insert_state(ServerState::StopServer)
        .add_systems(OnEnter(ServerState::RunServer), setup_actix)
        .add_systems(OnExit(ServerState::RunServer), stop_actix)
        .add_systems(Update, actix_rx)
        // UI
        //.add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

#[derive(Component, Debug)]
struct GrimoireShape {
    position: Position,
}

#[derive(Component, Debug, Default)]
struct Position {
    x: f32,
    y: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            move_keys: DirectionKeys::NONE,
            ..default()
        },
    ));

    let shapes = vec![meshes.add(Circle::new(50.0)), meshes.add(Circle::new(50.0))];
    let mut i = 0.0;
    for shape in shapes {
        let mut position = Position {
            x: i * 100.0,
            y: 0.0,
        };
        commands
            .spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(Color::hsl(0.5, 0.5, 0.5))),
                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                position,
            ))
            .observe(on_drag);
        i = i + 1.0;
    }
}

fn actix_rx(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    channels: Option<Res<NetChannels>>,
) {
    match channels {
        None => return,
        Some(channels) => {
            while let Ok(msg) = channels.rx.try_recv() {
                info!("Got JSON: {:?}", msg);
                match msg {
                    NetCommand {
                        cmd: cmd,
                        params: Some(params),
                    } => {
                        let shape = meshes.add(Circle::new(50.0));
                        let mut position = Position { x: 0.0, y: 0.0 };
                        position.x = if let Some(StringOrNumber::Number(x)) = params.get("x") {
                            *x
                        } else {
                            0.0
                        };
                        position.y = if let Some(StringOrNumber::Number(y)) = params.get("y") {
                            *y
                        } else {
                            0.0
                        };

                        commands
                            .spawn((
                                Mesh2d(shape),
                                MeshMaterial2d(materials.add(Color::hsl(0.5, 0.5, 0.5))),
                                Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                                position,
                            ))
                            .observe(on_drag);
                    }
                    _ => error!("Command not understood."),
                }
            }
        }
    }
}

fn on_drag(
    drag: On<Pointer<Drag>>,
    mut position: Query<&mut Position>,
    mut transforms: Query<&mut Transform>,
    projection: Single<&Projection, With<Camera>>,
    mut commands: Commands,
) {
    //info!("{:?}", shape.contains(drag.entity));
    //if shape.contains(drag.entity) {
    //info!("{:?}", transform.translation);
    //info!("{:?}", position);
    match *projection {
        Projection::Orthographic(p) => {
            if let Ok(mut transform) = transforms.get_mut(drag.entity) {
                info!("{:?}", position);
                if let Ok(mut position) = position.get_mut(drag.entity) {
                    position.x += drag.delta.x;
                    position.y -= drag.delta.y;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                    info!("{:?} vs {:?}", position, transform.translation)
                }
            }
        }
        _ => info!("no proj"),
    }
    //}
}
