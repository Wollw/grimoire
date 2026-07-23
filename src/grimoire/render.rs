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
    mut query: Query<(
        &GrimoireShape,
        &GrimoireColor,
        &GrimoirePosition,
        Entity,
        &Name,
        &mut GrimoireRedraw,
    )>,
    mut commands: Commands,
) {
    for (shape, GrimoireColor(color), position, entity, name, mut redraw) in &mut query {
        let rd = redraw.0;
        if !rd {
            commands.entity(entity).insert(Transform::from_xyz(
                (*position).x,
                (*position).y,
                (*position).z,
            ));
        } else {
            info!("(Re)drawing {}", name);
            if let Some(mesh) = build_mesh(shape) {
                let mesh = meshes.add(mesh);
                commands.entity(entity).insert(Mesh2d(mesh));
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(materials.add(*color)));
                commands.entity(entity).insert(Transform::from_xyz(
                    (*position).x,
                    (*position).y,
                    (*position).z,
                ));
            }
            *redraw = GrimoireRedraw::new(false);
        }
    }
}

fn build_mesh(shape: &GrimoireShape) -> Option<Mesh> {
    match shape {
        GrimoireShape::Circle { radius } => Some(Mesh::from(Circle::new(*radius))),
        GrimoireShape::Ellipse { half_size } => {
            Some(Mesh::from(Ellipse::new(half_size.x, half_size.y)))
        }
        GrimoireShape::Rect { width, height } => Some(Mesh::from(Rectangle::new(*width, *height))),
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
            Some(Mesh::from(mesh.to_bevy(RenderAssetUsages::all())))
        }
        _ => None,
    }
}
