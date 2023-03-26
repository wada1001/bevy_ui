use crate::entities::state::UIState;
use bevy::prelude::*;

use self::systems::{clean_up, input_handle, setup};

mod systems;

pub struct TitleStatePlugin;

impl Plugin for TitleStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UIState::Title)))
            .add_system(input_handle.in_set(OnUpdate(UIState::Title)))
            .add_system(clean_up.in_schedule(OnExit(UIState::Title)));
    }
}
