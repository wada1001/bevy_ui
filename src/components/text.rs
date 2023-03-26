use bevy::prelude::*;

pub fn default_text(font: Handle<Font>, text: &str) -> TextBundle {
    return TextBundle::from_section(
        text,
        TextStyle {
            font: font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
}
