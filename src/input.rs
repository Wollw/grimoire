use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_enhanced_input::prelude::*;
pub struct GrimoireInputPlugin;

#[derive(Resource)]
pub struct IsWindowFocused(bool);

impl Plugin for GrimoireInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_observer(fullscreen);

        // Remove for release
        app.add_observer(exit);

        app // comment out for release
            .add_input_context::<User>()
            .add_systems(Startup, setup_actions);
    }
}

#[derive(Component)]
pub struct User;

#[derive(InputAction)]
#[action_output(bool)]
struct Fullscreen;

#[derive(InputAction)]
#[action_output(bool)]
struct Exit;

fn setup_actions(mut commands: Commands) {
    commands.spawn((
        User,
        actions!(
            User[
                (Action::<Fullscreen>::new(), bindings![KeyCode::F12]),
                (Action::<Exit>::new(), bindings![KeyCode::Escape]
            )]
        ),
    ));
}

fn fullscreen(
    _fullscreen: On<Start<Fullscreen>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    window.mode = match window.mode {
        WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
        _ => WindowMode::Windowed,
    };
}

fn exit(_exit: On<Start<Exit>>, mut commands: Commands) {
    commands.write_message(AppExit::default());
}
