use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UIState {
    #[default]
    Title,
    Input,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TitleState {
    #[default]
    Invalid,
    Active,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InputState {
    #[default]
    Invalid,
    Active,
}
