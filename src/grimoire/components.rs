use crate::grimoire;
use bevy::{color::palettes::basic::*, prelude::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Component, Debug, Clone, Default)]
struct NowDrawing;

#[derive(Clone, Default, Component, Debug)]
pub struct GrimoireVisible;

#[derive(Clone, Default, Component, Debug)]
pub struct GrimoireDraggable;

fn serialize_color<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    let hex = Srgba::from(*color).to_hex();
    <String>::serialize(&hex, serializer)
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, SceneComponent)]
#[scene(GrimoireObjectProps)]
pub struct GrimoireObject;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GrimoireObjectProps {
    pub name: String,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub color: Color,
    pub position: Vec3,
    pub shape: GrimoireShape,
}

impl Default for GrimoireObjectProps {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            color: YELLOW.into(),
            position: Vec3::new(0., 0., 0.),
            shape: GrimoireShape::Circle { radius: 25. },
        }
    }
}

impl GrimoireObject {
    pub fn scene(props: GrimoireObjectProps) -> impl Scene {
        bsn! {
            Name::new(props.name)
            Pickable
            GrimoireDraggable
            GrimoireShape::ident(props.shape)
            GrimoirePosition::new(props.position.x,props.position.y, props.position.z)
            GrimoireColor::new(props.color)
            GrimoireRedraw::new(true)
            on(grimoire_drag)
            on(|_event:On<Pointer<Over>>, mut ooo:ResMut<IsOverOrOut> | {*ooo = IsOverOrOut::Over})
            on(|_event:On<Pointer<Out>>, mut ooo:ResMut<IsOverOrOut> | {*ooo = IsOverOrOut::Out})
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Component)]
pub enum GrimoireShape {
    Rect { width: f32, height: f32 },
    Circle { radius: f32 },
    Capsule { radius: f32, half_length: f32 },
    Ellipse { half_size: Vec2 },
    Polygon { vertex_groups: Vec<Vec<Vec2>> },
}

impl Default for GrimoireShape {
    fn default() -> Self {
        GrimoireShape::Circle { radius: 25. }
    }
}

impl GrimoireShape {
    fn ident(shape: GrimoireShape) -> GrimoireShape {
        shape
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Component)]
pub struct GrimoirePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GrimoirePosition {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Default for GrimoirePosition {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct GrimoireRedraw(pub bool);

impl GrimoireRedraw {
    pub fn new(b: bool) -> Self {
        Self(b)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Component)]
pub struct GrimoireColor(pub Color);

impl GrimoireColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Default for GrimoireColor {
    fn default() -> Self {
        Self(Color::hsl(1., 1., 1.))
    }
}

pub fn grimoire_drag(
    drag: On<Pointer<Drag>>,
    world_pos: Res<grimoire::CursorWorldPos>,
    mut query: Query<(&mut GrimoirePosition, &Name), With<GrimoireDraggable>>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }
    if let Ok((mut position, name)) = query.get_mut(drag.entity) {
        if let Some(world_pos) = world_pos.0 {
            position.x = world_pos.x;
            position.y = world_pos.y;
        }
    }
}

#[derive(PartialEq, Eq, Debug, Resource)]
pub enum IsOverOrOut {
    Over,
    Out,
}
