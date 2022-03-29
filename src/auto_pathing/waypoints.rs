use bevy::input::ElementState;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::ConstZero;
use uom::si::angle::radian;
use uom::si::f32::*;
use uom::si::length::meter;
use crate::auto_pathing::trajectory::{generate_trajectory, Trajectory};

use crate::field::{Field, FieldPose, FieldPosition};
use crate::field::render::FieldZ;
use crate::field::shapes::FieldPath;
use crate::Layout;

const WAYPOINT_RADIUS: f32 = 15.0;
const ROTATION_ANCHOR_POINT_RADIUS: f32 = 10.0;
const ROTATION_ANCHOR_REVOLUTION_RADIUS: f32 = 25.0;

#[derive(Copy, Clone)]
pub enum Waypoint {
    Translation(FieldPosition),
    Pose(FieldPose),
}

#[derive(Default)]
pub struct FieldWaypointList(pub Vec<Option<Waypoint>>);

#[derive(Component)]
pub struct FieldWaypoint(usize);

#[derive(Component)]
pub struct FieldRotationAnchor(usize);

#[derive(Component)]
pub struct DrawnTrajectory;

pub fn setup(mut commands: Commands) {
    let mut list = FieldWaypointList::default();

    let wp1 = Waypoint::Pose(
        FieldPose::new(
            FieldPosition::new(
                Length::new::<meter>(1.0),
                Length::new::<meter>(1.0)
            ),
            Angle::ZERO
        )
    );

    let wp2 = Waypoint::Translation(
        FieldPosition::new(
            Length::new::<meter>(3.0),
            Length::new::<meter>(2.0)
        )
    );

    let wp3 = Waypoint::Translation(
        FieldPosition::new(
            Length::new::<meter>(4.0),
            Length::new::<meter>(1.0)
        )
    );

    let wp4 = Waypoint::Pose(
        FieldPose::new(
            FieldPosition::new(
                Length::new::<meter>(6.0),
                Length::new::<meter>(1.0)
            ),
            Angle::ZERO
        )
    );

    spawn_waypoint(wp1, &mut list, &mut commands);
    spawn_waypoint(wp2, &mut list, &mut commands);
    spawn_waypoint(wp3, &mut list, &mut commands);
    spawn_waypoint(wp4, &mut list, &mut commands);

    let default_shape = shapes::Circle::default();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
        Transform::from_xyz(0.0, 0.0, FieldZ::AUTO_PATH.0)
    )).insert(Trajectory::default());

    commands.insert_resource(list);
    commands.insert_resource(CursorState::default());
}

fn spawn_waypoint(waypoint: Waypoint, list: &mut FieldWaypointList, commands: &mut Commands) {
    let waypoint_shape = shapes::Circle {
        radius: WAYPOINT_RADIUS - 4.0,
        center: Default::default()
    };

    let fill_color: Color;
    let stroke_color: Color;

    match waypoint {
        Waypoint::Translation(_) => {
            fill_color = Color::GREEN;
            stroke_color = Color::LIME_GREEN;
        }
        Waypoint::Pose(_) => {
            fill_color = Color::ORANGE_RED;
            stroke_color = Color::RED;
        }
    }

    commands.spawn_bundle(GeometryBuilder::build_as(
        &waypoint_shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(fill_color),
            outline_mode: StrokeMode::new(stroke_color, 4.0)
        },
        Transform::default()
    )).insert(FieldWaypoint(list.0.len()));

    let rotation_anchor_shape = shapes::Circle {
        radius: ROTATION_ANCHOR_POINT_RADIUS - 4.0,
        center: Default::default()
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &rotation_anchor_shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::PINK, 4.0)
        },
        Transform::default()
    )).insert(FieldRotationAnchor(list.0.len()));

    list.0.push(Some(waypoint))
}

pub fn waypoint_updater(
    field: Res<Field>,
    layout: Res<Layout>,
    mut query: Query<(&FieldWaypoint, &mut Transform, &mut Visibility)>,
    waypoints: Res<FieldWaypointList>
) {
    for i in query.iter_mut() {
        let (field_waypoint, mut transform, mut visibility): (&FieldWaypoint, Mut<Transform>, Mut<Visibility>) = i;
        match waypoints.0[field_waypoint.0] {
            None => {
                *visibility = Visibility {
                    is_visible: false
                }
            }
            Some(w) => {
                let pose = match w {
                    Waypoint::Translation(t) => { FieldPose::new(t, Angle::ZERO) }
                    Waypoint::Pose(p) => { p }
                };
                *transform = field.to_screen_transform(
                    &layout,
                    &pose,
                    FieldZ::AUTO_WAYPOINTS.0,
                );
                *visibility = Visibility {
                    is_visible: true
                }
            }
        }
    }
}

pub fn rotation_anchor_updater(
    field: Res<Field>,
    layout: Res<Layout>,
    mut query: Query<(&FieldRotationAnchor, &mut Transform, &mut Visibility)>,
    waypoints: Res<FieldWaypointList>
) {
    for i in query.iter_mut() {
        let (rotation_anchor, mut transform, mut visibility): (&FieldRotationAnchor, Mut<Transform>, Mut<Visibility>) = i;

        if let Some(Waypoint::Pose(pose)) = waypoints.0[rotation_anchor.0] {
            let center_transform = field.to_screen_transform(
                &layout,
                &pose,
                FieldZ::AUTO_WAYPOINTS.0,
            );

            let theta = pose.rotation.get::<radian>();
            let transform_offset = Vec3::new(
                theta.cos() * ROTATION_ANCHOR_REVOLUTION_RADIUS,
                theta.sin() * ROTATION_ANCHOR_REVOLUTION_RADIUS,
                0.0
            );

            let final_transform = Transform::from_translation(center_transform.translation + transform_offset);

            *transform = final_transform;

            *visibility = Visibility {
                is_visible: true
            }
        } else {
            *visibility = Visibility {
                is_visible: false
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct CursorState {
    pos: Option<FieldPosition>,
    grabbed: CursorGrabOption
}

#[derive(Debug)]
pub enum CursorGrabOption {
    Position(usize),
    Rotation(usize),
    None
}

impl Default for CursorGrabOption {
    fn default() -> Self { CursorGrabOption::None }
}

pub fn waypoint_grab_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_state: ResMut<CursorState>,
    layout: Res<Layout>,
    field: Res<Field>,
    mut waypoints: ResMut<FieldWaypointList>
) {
    for event in cursor_moved_events.iter() {
        let event: &CursorMoved = event;

        cursor_state.pos = field.to_field_position(
            &layout,
            event.position - Vec2::new(
                layout.screen_size.x / 2.0,
                layout.screen_size.y / 2.0
            )
        );

        if let Some(new_cursor_pos) = cursor_state.pos {
            match cursor_state.grabbed {
                CursorGrabOption::Position(id) => {
                    if let Some(t) = waypoints.0[id] {
                        match t {
                            Waypoint::Translation(_) => { waypoints.0[id] = Some(Waypoint::Translation(new_cursor_pos)) }
                            Waypoint::Pose(pose) => { waypoints.0[id] = Some(Waypoint::Pose(FieldPose::new(new_cursor_pos, pose.rotation))) }
                        };
                    }
                }
                CursorGrabOption::Rotation(id) => {
                    if let Some(Waypoint::Pose(pose)) = waypoints.0[id] {
                        if let Some(cursor_pos) = cursor_state.pos {
                            waypoints.0[id] = Some(Waypoint::Pose(FieldPose::new(
                                pose.translation,
                                Angle::new::<radian>(
                                    (cursor_pos.y - pose.translation.y).get::<meter>().atan2(
                                        (cursor_pos.x - pose.translation.x).get::<meter>()
                                    )
                                )
                            )));
                        }
                    }
                }
                CursorGrabOption::None => {}
            }
        }
    }

    for event in mouse_button_input_events.iter() {
        let event: &MouseButtonInput = event;

        if event.button == MouseButton::Left {
            match event.state {
                ElementState::Pressed => {
                    if let CursorGrabOption::None = cursor_state.grabbed {
                        if let Some(mouse_pos) = cursor_state.pos {
                            'outer: for (id, w) in waypoints.0.iter().enumerate() {
                                if let Some(w) = w {
                                    let field_position: &FieldPosition;

                                    match w {
                                        Waypoint::Pose(pose) => {
                                            field_position = &pose.translation;

                                            let theta = pose.rotation.get::<radian>();
                                            let anchor_pos = FieldPosition::new(
                                                field_position.x + (
                                                    theta.cos() * ROTATION_ANCHOR_REVOLUTION_RADIUS *
                                                        (field.size.x / layout.field.size.x)
                                                ),
                                                field_position.y + (
                                                    theta.sin() * ROTATION_ANCHOR_REVOLUTION_RADIUS *
                                                        (field.size.y / layout.field.size.y)
                                                )
                                            );

                                            let d = mouse_pos.dist(&anchor_pos);
                                            let d = layout.field.size.x * d.get::<meter>() / field.size.x.get::<meter>();

                                            if d <= ROTATION_ANCHOR_POINT_RADIUS {
                                                cursor_state.grabbed = CursorGrabOption::Rotation(id);
                                                break 'outer;
                                            }
                                        }
                                        Waypoint::Translation(t) => { field_position = t }
                                    }

                                    let d = mouse_pos.dist(field_position);
                                    let d = layout.field.size.x * d.get::<meter>() / field.size.x.get::<meter>();

                                    if d <= WAYPOINT_RADIUS {
                                        cursor_state.grabbed = CursorGrabOption::Position(id);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                ElementState::Released => {
                    cursor_state.grabbed = CursorGrabOption::None;
                }
            }
        }
    }
}