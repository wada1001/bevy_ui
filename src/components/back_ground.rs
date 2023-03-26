use bevy::prelude::*;

// TODO 別に背景じゃない
pub fn default_back_ground() -> NodeBundle {
    return NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };
}
