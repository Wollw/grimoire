use bevy::prelude::*;

pub fn drag_plugin(app: &mut App) {
    app.add_observer(drag);
}

#[derive(Component, Debug, Default, Clone)]
#[require(Pickable)]
pub struct Draggable;

fn drag(
    drag: On<Pointer<Drag>>,
    mut tr: Query<&mut Transform, With<Draggable>>,
    cam: Single<(&Camera, &GlobalTransform)>,
) {
    if let Ok(mut transform) = tr.get_mut(drag.entity) {
        // drag.propagate(false);
        let Ok(mut viewport) = cam.0.world_to_viewport(cam.1, transform.translation) else {
            return;
        };
        viewport += drag.delta;
        transform.translation = match cam.0.viewport_to_world_2d(cam.1, viewport) {
            Ok(it) => it,
            Err(_) => return,
        }
        .extend(transform.translation.z);
    }
}
