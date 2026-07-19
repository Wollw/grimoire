use crate::drag_plugin::*;
use bevy::dev_tools::infinite_grid::*;
use bevy::pbr::StandardMaterialUniform;
use bevy::prelude::*;
use bevy::sprite_render::*;
use bevy_pancam::*;

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

#[derive(Debug, Clone, Default, SceneComponent)]
#[scene(GrimoireObjectProps)]
#[derive(Reflect)]
#[reflect(Component, Clone, Default)]
pub struct GrimoireObject {
    pub name: String,
}
pub struct GrimoireObjectProps {
    pub color: Color,
    pub position: Vec3,
    pub size: f32,
    pub mesh: Mesh,
}
impl Default for GrimoireObjectProps {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            position: Vec3::new(0., 0., 0.),
            size: 50.,
            mesh: Circle::new(50.).mesh().build(),
        }
    }
}

impl GrimoireObject {
    pub fn scene(props: GrimoireObjectProps) -> impl Scene {
        bsn! {
            Mesh2d(asset_value(props.mesh))
            MeshMaterial2d::<ColorMaterial>(asset_value(props.color))
            Transform::from_translation(props.position)
            Pickable{should_block_lower: true,is_hoverable:true}
            Draggable
        }
    }
}
