use crate::drag_plugin::*;
use bevy::dev_tools::infinite_grid::*;
use bevy::pbr::StandardMaterialUniform;
use bevy::prelude::*;
use bevy::sprite_render::*;
use bevy_color;
use bevy_pancam::*;
use serde;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Debug, Clone, Default, SceneComponent)]
#[scene(GrimoireSceneProps)]
#[derive(Reflect)]
#[reflect(Component, Clone, Default)]
pub struct GrimoireScene;
pub struct GrimoireSceneProps {
    pub background_color: Color,
}
impl Default for GrimoireSceneProps {
    fn default() -> Self {
        Self {
            background_color: Color::BLACK,
        }
    }
}

impl GrimoireScene {
    fn scene(props: GrimoireSceneProps) -> impl Scene {
        bsn! {
            Camera2d
        }
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let hex = String::deserialize(deserializer)?;
    let color = bevy_color::Srgba::hex(hex).unwrap();
    Ok(Color::srgba(
        color.red,
        color.green,
        color.blue,
        color.alpha,
    ))
}

#[derive(Debug, Clone, Default, SceneComponent)]
#[scene(GrimoireObjectProps)]
#[derive(Reflect)]
#[reflect(Component, Clone, Default)]
pub struct GrimoireObject;
#[derive(Debug, Clone, Component, Deserialize, Serialize)]
pub struct GrimoireObjectProps {
    pub name: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
    pub position: Vec3,
    pub size: f32,
}
impl Default for GrimoireObjectProps {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            color: Color::WHITE,
            position: Vec3::new(0., 0., 0.),
            size: 50.,
        }
    }
}

impl GrimoireObject {
    pub fn scene(props: GrimoireObjectProps) -> impl Scene {
        bsn! {
            GrimoireObjectProps {
                name: {props.name},
                color: {props.color},
                position: {props.position},
                size: {props.size}
            }
            MeshMaterial2d::<ColorMaterial>(asset_value(props.color))
            Transform::from_translation(props.position)
            Pickable
            Draggable
        }
    }
}
