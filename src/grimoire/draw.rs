use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy_procedural_meshes::*;

use crate::grimoire::components::*;

#[derive(Resource)]
pub struct MeshHandleRes(pub Option<Handle<Mesh>>);

// Main draw system for the canvas
pub fn grimoire_draw(
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
