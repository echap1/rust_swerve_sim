use std::env;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use pyo3::Python;
use pyo3::types::{PyFunction, PyList};
use uom::ConstZero;
use uom::si::angle::radian;
use uom::si::f32::*;
use uom::si::length::meter;

use serde::Serialize;

use crate::auto_pathing::waypoints::{FieldWaypointList, Waypoint};
use crate::field::{Field, FieldPose, FieldPosition};
use crate::{Layout, RobotClient};

#[derive(Component, Default, Serialize)]
pub struct Trajectory {
    pub start: FieldPose,
    pub points: Vec<FieldPosition>,
    pub end: FieldPose
}

pub fn build_trajectory_path(trajectory: &Trajectory, field: &Field, layout: &Layout, client: &mut RobotClient) -> Path {
    let points = client.gen_trajectory(trajectory);

    let mut builder = PathBuilder::new();

    if points.len() == 0 {
        builder.move_to(Vec2::new(0.0, 0.0));
        return builder.build();
    }

    builder.move_to(field.to_screen_vec(layout, &points[0]));

    // TODO Dont draw line to first point
    for p in &points {
        builder.line_to(field.to_screen_vec(layout, p));
    }

    builder.build()
}

pub fn generate_trajectory(waypoints: &FieldWaypointList) -> Trajectory {
    let mut points: Vec<FieldPosition> = Vec::with_capacity(waypoints.0.len());

    let internal_waypoints = &waypoints.0[1..waypoints.0.len()-1];

    for w in internal_waypoints {
        if let Some(w) = w {
            points.push(match w {
                Waypoint::Translation(t) => { *t }
                Waypoint::Pose(pose) => { pose.translation }
            })
        }
    }

    Trajectory {
        start: match waypoints.0.first().unwrap().unwrap() {
            Waypoint::Translation(t) => { FieldPose::new(t, Angle::ZERO) }
            Waypoint::Pose(p) => { p }
        },
        points,
        end: match waypoints.0.last().unwrap().unwrap() {
            Waypoint::Translation(t) => { FieldPose::new(t, Angle::ZERO) }
            Waypoint::Pose(p) => { p }
        }
    }
}

pub fn trajectory_updater(mut query: Query<&mut Trajectory>, waypoints: Res<FieldWaypointList>) {
    let mut trajectory: Mut<Trajectory> = query.single_mut();
    *trajectory = generate_trajectory(&waypoints);
}

pub fn trajectory_path_updater(mut query: Query<(&Trajectory, &mut Path)>, field: Res<Field>, layout: Res<Layout>, mut client: ResMut<RobotClient>) {
    for i in query.iter_mut() {
        let (trajectory, mut path): (&Trajectory, Mut<Path>) = i;
        *path = build_trajectory_path(trajectory, &field, &layout, &mut client);
    }
}