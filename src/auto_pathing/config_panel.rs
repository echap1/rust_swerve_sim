use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::ConstZero;
use uom::si::angle::Angle;
use uom::si::f32::Length;
use uom::si::length::meter;
use crate::auto_pathing::waypoints::{FieldWaypointList, spawn_waypoint, Waypoint};
use crate::field::{FieldPose, FieldPosition};
use crate::{Layout, Trajectory};
use crate::auto_pathing::trajectory::TrajectoryID;
use crate::field::render::FieldZ;
use crate::layout::event::LayoutChangedEvent;
use crate::layout::render::FONT_SIZE;

#[derive(Component)]
pub struct ConfigRoot;

pub enum ConfigButtonAction {
    AddWaypoint,
    RemoveWaypoint,
    IncrementPathIdx,
    DecrementPathIdx,
    AddPath
}

#[derive(Component)]
pub struct ConfigButton {
    action: ConfigButtonAction
}

#[derive(Component)]
pub enum ConfigText {
    RoutineNumber
}

const NORMAL_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);
const PRESSED_BUTTON: Color = Color::rgb(0.55, 0.75, 0.55);
const TEXT_COLOR: Color = Color::BLACK;

fn text(text: &str, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: FONT_SIZE,
                color: Color::GRAY,
            },
            TextAlignment::default(),
        ),
        ..TextBundle::default()
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(NodeBundle {
        color: UiColor(Color::NONE),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            ..Default::default()
        }).with_children(|parent_2| {
            parent_2.spawn_bundle(text("Waypoint Number", &asset_server));
            generate_button(parent_2, "+".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::AddWaypoint
            });
            generate_button(parent_2, "-".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::RemoveWaypoint
            });
        });
        parent.spawn_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            ..Default::default()
        }).with_children(|parent_2| {
            parent_2.spawn_bundle(text("Routine: ", &asset_server)).insert(ConfigText::RoutineNumber);
            generate_button(parent_2, "+".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::IncrementPathIdx
            });
            generate_button(parent_2, "-".to_string(), &asset_server, ConfigButton {
                action: ConfigButtonAction::DecrementPathIdx
            });
        });
        generate_button(parent, "Add Path".to_string(), &asset_server, ConfigButton {
            action: ConfigButtonAction::AddPath
        });
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

pub fn config_text_updater(mut query: Query<(&mut Text, &ConfigText)>, list: Res<FieldWaypointList>) {
    for i in query.iter_mut() {
        let (mut text, t): (Mut<Text>, &ConfigText) = i;
        match t {
            ConfigText::RoutineNumber => {
                text.sections[0].value = "Routine: ".to_string() + &*list.1.to_string();
            }
        }
    }
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
                    ConfigButtonAction::AddWaypoint => {
                        let path_idx = waypoint_list.1;
                        let l = waypoint_list.0[path_idx].last_mut().unwrap();
                        *l = match l {
                            Waypoint::Translation(t) => { Waypoint::Translation(*t) }
                            Waypoint::Pose(p) => { Waypoint::Translation(p.translation) }
                        };

                        spawn_waypoint(
                            Waypoint::Pose(FieldPose::new(FieldPosition::new(Length::new::<meter>(1.0), Length::new::<meter>(1.0)), Angle::ZERO)),
                            &mut waypoint_list,
                            &mut commands,
                            path_idx
                        )
                    }
                    ConfigButtonAction::RemoveWaypoint => {
                        let path_idx = waypoint_list.1;

                        if waypoint_list.0[path_idx].len() <= 2 {
                            return;
                        }

                        waypoint_list.0[path_idx].pop();

                        let l = waypoint_list.0[path_idx].last_mut().unwrap();
                        *l = match l {
                            Waypoint::Translation(t) => {
                                Waypoint::Pose(
                                    FieldPose::new(*t, Angle::ZERO)
                                )
                            }
                            Waypoint::Pose(p) => {
                                Waypoint::Pose(*p)
                            }
                        };
                    }
                    ConfigButtonAction::IncrementPathIdx => {
                        if waypoint_list.1 >= waypoint_list.0.len() - 1 {
                            waypoint_list.1 = 0;
                        } else {
                            waypoint_list.1 += 1;
                        }
                    }
                    ConfigButtonAction::DecrementPathIdx => {
                        if waypoint_list.1 == 0 {
                            waypoint_list.1 = waypoint_list.0.len() - 1;
                        } else {
                            waypoint_list.1 -= 1;
                        }
                    }
                    ConfigButtonAction::AddPath => {
                        let path_idx = waypoint_list.0.len();
                        let start_pos = match waypoint_list.0.last().unwrap().last().unwrap() {
                            Waypoint::Translation(t) => { *t }
                            Waypoint::Pose(p) => { p.translation }
                        };
                        waypoint_list.0.push(vec![]);
                        spawn_waypoint(
                            Waypoint::Pose(FieldPose::new(start_pos, Angle::ZERO)),
                            &mut waypoint_list,
                            &mut commands,
                            path_idx
                        );
                        spawn_waypoint(
                            Waypoint::Pose(FieldPose::new(FieldPosition::new(Length::new::<meter>(2.0), Length::new::<meter>(1.0)), Angle::ZERO)),
                            &mut waypoint_list,
                            &mut commands,
                            path_idx
                        );
                        let default_shape = shapes::Circle::default();
                        commands.spawn_bundle(GeometryBuilder::build_as(
                            &default_shape,
                            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                            Transform::from_xyz(0.0, 0.0, FieldZ::AUTO_PATH.0)
                        )).insert(Trajectory::default()).insert(TrajectoryID(path_idx));
                        waypoint_list.1 = path_idx;
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