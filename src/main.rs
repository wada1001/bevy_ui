use bevy::{prelude::*, window::WindowResolution};
use bevy_ecss::prelude::EcssPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use entities::state::UIState;
use input::InputStatePlugin;
use title::TitleStatePlugin;

mod entities;
mod input;
mod title;

fn main() {
    App::new()
        .add_state::<UIState>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy UI Test"),
                        resizable: true,
                        resolution: WindowResolution::new(500., 400.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(EcssPlugin::with_hot_reload())
        .add_plugin(TitleStatePlugin)
        .add_plugin(InputStatePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<UIState>>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    next_state.set(UIState::Title);
}
