use bevy::prelude::*;
use bevy_ecss::{Class, StyleSheet};

use crate::entities::state::UIState;

#[derive(Resource)]
pub struct UIRoot {
    pub root: Entity,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = commands
        .spawn(NodeBundle { ..default() })
        .insert(Name::new("root"))
        .insert(StyleSheet::new(asset_server.load("sheets/input.css")))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle { ..default() })
                .insert(Class::new("data_table"))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle::default())
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("id", TextStyle::default()));
                            parent.spawn(TextBundle::from_section("value_1", TextStyle::default()));
                            parent.spawn(TextBundle::from_section("value_2", TextStyle::default()));
                            parent.spawn(TextBundle::from_section("value_3", TextStyle::default()));
                            parent.spawn(TextBundle::from_section("date", TextStyle::default()));
                        })
                        .insert(Class::new("table_row"));

                    for col in 1..4 {
                        parent
                            .spawn(NodeBundle::default())
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("id_{}", col),
                                    TextStyle::default(),
                                ));
                                parent.spawn(TextBundle::from_section(
                                    format!("1_{}, ", col),
                                    TextStyle::default(),
                                ));
                                parent.spawn(TextBundle::from_section(
                                    format!("2_{}", col),
                                    TextStyle::default(),
                                ));
                                parent.spawn(TextBundle::from_section(
                                    format!("3_{}", col),
                                    TextStyle::default(),
                                ));
                            })
                            .insert(Class::new("table_row"));
                    }

                    parent
                        .spawn(ButtonBundle {
                            image: asset_server.load("img/hoka2_03_atmark.png").into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // TODO わかってないがボタン単体だと反応しない
                            parent.spawn(NodeBundle { ..default() });
                        });
                });
        })
        .id();
    commands.insert_resource(UIRoot { root: entity });
}

pub fn input_handle(
    mut next_state: ResMut<NextState<UIState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => next_state.set(UIState::Title),
            Interaction::Hovered => {
                print!("ok!");
            }
            Interaction::None => {
                print!("ok!");
            }
        }
    }
}

pub fn clean_up(mut commands: Commands, res: Res<UIRoot>) {
    commands.entity(res.root).despawn_recursive();
}
