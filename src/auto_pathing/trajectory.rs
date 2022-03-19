use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use uom::ConstZero;
use uom::si::f32::*;

use crate::auto_pathing::waypoints::{FieldWaypointList, Waypoint};
use crate::field::{Field, FieldPosition};
use crate::Layout;

#[derive(Default, Component)]
pub struct Trajectory {
    points: Vec<FieldPosition>
}

pub fn build_trajectory_path(trajectory: &Trajectory, field: &Field, layout: &Layout) -> Path {
    let mut builder = PathBuilder::new();

    if trajectory.points.len() == 0 {
        builder.move_to(Vec2::new(0.0, 0.0));
        return builder.build();
    }

    builder.move_to(field.to_screen_vec(layout, &trajectory.points[0]));

    // TODO Dont draw line to first point
    for p in &trajectory.points {
        builder.line_to(field.to_screen_vec(layout, p));
    }

    builder.build()
}

pub fn generate_trajectory(waypoints: &FieldWaypointList) -> Trajectory {
    let mut points: Vec<FieldPosition> = Vec::with_capacity(waypoints.0.len());

    for w in &waypoints.0 {
        if let Some(w) = w {
            points.push(match w {
                Waypoint::Translation(t) => { *t }
                Waypoint::Pose(pose) => { pose.translation }
            })
        }
    }

    Trajectory { points }
}

pub fn trajectory_updater(mut query: Query<&mut Trajectory>, waypoints: Res<FieldWaypointList>) {
    let mut trajectory: Mut<Trajectory> = query.single_mut();
    *trajectory = generate_trajectory(&waypoints);
}

pub fn trajectory_path_updater(mut query: Query<(&Trajectory, &mut Path)>, field: Res<Field>, layout: Res<Layout>) {
    for i in query.iter_mut() {
        let (trajectory, mut path): (&Trajectory, Mut<Path>) = i;
        *path = build_trajectory_path(trajectory, &field, &layout);
    }
}