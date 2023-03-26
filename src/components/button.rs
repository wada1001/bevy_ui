use bevy::{prelude::*, winit::WinitSettings};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn default_button(width: f32, height: f32) -> ButtonBundle {
    return ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(width), Val::Px(height)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
}
