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
mod interface;
mod server;
use crate::drag_plugin::drag_plugin;
use crate::grimoire::*;
use crate::interface::*;
use crate::server::*;

mod drag_plugin;
fn main() {
    info!("Starting Application");

    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, PanCamPlugin::default()))
        .add_plugins(InterfacePlugin)
        .add_plugins(drag_plugin)
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        move_keys: DirectionKeys::NONE,
        ..default()
    });
    commands.spawn_scene_list(my_scene());
}

fn my_scene() -> impl SceneList {
    bsn_list! [
        @GrimoireObject { name:"foo" },
        @GrimoireObject {
            name: "bar",
            @color : Color::hsl(0.0, 1.0, 0.5),
            @position: Vec3::new(0.0,0.0,-1000.0)
        }

    ]
}

fn actix_rx(mut commands: Commands, channels: Option<Res<NetChannels>>) {
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
                        let mesh = Circle::new(50.0).mesh().build();
                        let mut position = Vec2 { x: 0.0, y: 0.0 }.extend(0.0);
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

                        let color = Color::hsl(100., 0.5, 0.5);

                        let name = if let Some(StringOrNumber::String(name)) = params.get("name") {
                            name
                        } else {
                            ""
                        };

                        commands.spawn_scene(bsn! {
                            @GrimoireObject {
                                name: name,
                                @position: position,
                                @color:color,
                                @mesh:mesh
                            }
                        });
                    }
                    _ => error!("Command not understood."),
                }
            }
        }
    }
}

/*
fn on_drag(
    drag: On<Pointer<Drag>>,
    mut grim_obj: Query<&mut GrimoireObject>,
    mut transforms: Query<&mut Transform>,
    projection: Single<&Projection, With<Camera>>,
) {
    match *projection {
        Projection::Orthographic(p) => {
            if let Ok(mut transform) = transforms.get_mut(drag.entity) {
                info!("{:?}", grim_obj);
                if let Ok(mut grim_obj) = grim_obj.get_mut(drag.entity) {
                    grim_obj.position.x += drag.delta.x * p.scale;
                    grim_obj.position.y -= drag.delta.y * p.scale;
                    transform.translation.x = grim_obj.position.x;
                    transform.translation.y = grim_obj.position.y;
                }
            }
        }
        _ => error!("no proj"),
    }
    //}
}
*/
