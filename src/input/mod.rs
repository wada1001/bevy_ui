use crate::entities::state::UIState;
use bevy::prelude::*;

use self::systems::clean_up;
use self::systems::input_handle;
use self::systems::setup;

pub mod systems;

pub struct InputStatePlugin;

impl Plugin for InputStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UIState::Input)))
            .add_system(input_handle.in_set(OnUpdate(UIState::Input)))
            .add_system(clean_up.in_schedule(OnExit(UIState::Input)));
    }
}
