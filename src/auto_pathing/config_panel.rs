use bevy::app::Events;
use bevy::prelude::*;
use uom::ConstZero;
use uom::si::angle::Angle;
use uom::si::f32::Length;
use uom::si::length::meter;
use crate::auto_pathing::waypoints::{FieldWaypointList, spawn_waypoint, Waypoint};
use crate::field::{FieldPose, FieldPosition};
use crate::Layout;
use crate::layout::event::LayoutChangedEvent;
use crate::layout::render::FONT_SIZE;

#[derive(Component)]
pub struct ConfigRoot;

pub enum ConfigButtonAction {
    AddWaypoint(usize),
    RemoveWaypoint(usize)
}

#[derive(Component)]
pub struct ConfigButton {
    action: ConfigButtonAction
}

const NORMAL_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);
const PRESSED_BUTTON: Color = Color::rgb(0.55, 0.75, 0.55);
const TEXT_COLOR: Color = Color::BLACK;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(NodeBundle {
        color: UiColor(Color::NONE),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            ..Default::default()
        }).with_children(|parent_2| {
            parent_2.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Waypoint Number",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::GRAY,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
            generate_button(parent_2, "+".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::AddWaypoint(0)
            });
            generate_button(parent_2, "-".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::RemoveWaypoint(0)
            });
        });
        // generate_button(parent, "helo".to_string(), &asset_server, ConfigButton {
        //     action: ConfigButtonAction::AddWaypoint
        // });
    }).insert(ConfigRoot {

    });
}

fn generate_button(parent: &mut ChildBuilder, text: String, asset_server: &AssetServer, component: ConfigButton) {
    parent.spawn_bundle(ButtonBundle {
        color: UiColor(NORMAL_BUTTON),
        style: Style {
            margin: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|button_parent| {
        button_parent.spawn_bundle(TextBundle {
            style: Style {
                margin: Rect::all(Val::Px(7.0)),
                ..Default::default()
            },
            text: Text::with_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        });
    }).insert(component);
}

pub fn root_updater(
    mut query: Query<(&ConfigRoot, &mut Style)>,
    layout_changed_event: Res<Events<LayoutChangedEvent>>
) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (_, mut style): (&ConfigRoot, Mut<Style>) = i;
                *style = Style {
                    size: Size::new(Val::Px(layout.auto_cfg.size.x), Val::Px(layout.auto_cfg.size.y - FONT_SIZE)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::ColumnReverse,
                    position: Rect {
                        left: Val::Px(layout.auto_cfg.pos.x + (layout.screen_size.x / 2.0)),
                        bottom: Val::Px(layout.auto_cfg.pos.y + (layout.screen_size.y / 2.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                };
            }
        }
    };
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &ConfigButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut waypoint_list: ResMut<FieldWaypointList>,
    mut commands: Commands
) {
    for i in interaction_query.iter_mut() {
        let (interaction, mut color, children, button): (&Interaction, Mut<UiColor>, &Children, &ConfigButton) = i;
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                match button.action {
                    ConfigButtonAction::AddWaypoint(path_id) => {
                        let l = waypoint_list.0[path_id].last_mut().unwrap();
                        *l = match l {
                            None => { None }
                            Some(w) => {
                                match w {
                                    Waypoint::Translation(t) => { Some(Waypoint::Translation(*t)) }
                                    Waypoint::Pose(p) => { Some(Waypoint::Translation(p.translation)) }
                                }
                            }
                        };

                        spawn_waypoint(
                            Waypoint::Pose(FieldPose::new(FieldPosition::new(Length::new::<meter>(1.0), Length::new::<meter>(1.0)), Angle::ZERO)),
                            &mut waypoint_list,
                            &mut commands,
                            path_id
                        )
                    }
                    ConfigButtonAction::RemoveWaypoint(path_id) => {
                        waypoint_list.0[path_id].pop();

                        let l = waypoint_list.0[path_id].last_mut().unwrap();
                        *l = match l {
                            None => { None }
                            Some(w) => {
                                match w {
                                    Waypoint::Translation(t) => {
                                        Some(Waypoint::Pose(
                                            FieldPose::new(*t, Angle::ZERO)
                                        ))
                                    }
                                    Waypoint::Pose(p) => {
                                        Some(Waypoint::Pose(*p))
                                    }
                                }
                            }
                        };
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}