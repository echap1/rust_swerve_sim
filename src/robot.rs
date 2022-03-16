use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::si::angle::radian;
use uom::si::f32::{Angle, Length};
use crate::field::{FieldPose, FieldRectangle};

use uom::si::length::{inch, meter};

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(update);
    }
}

#[derive(Component)]
pub struct Robot {
    state: RobotState
}

pub enum RobotState {
    DISABLED,
    TELEOP,
    AUTONOMOUS(u32)  // Routine number?
}

fn setup(mut commands: Commands) {
    let robot_shape = shapes::Rectangle::default();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &robot_shape,
        DrawMode::Fill(FillMode::color(Color::GRAY)),
        Transform::default(),
    )).insert(FieldPose {
        x: Length::new::<meter>(10.0),
        y: Length::new::<meter>(5.0),
        rotation: Angle::default()
    }).insert(FieldRectangle {
        width: Length::new::<inch>(29.0),
        height: Length::new::<inch>(29.0),
        origin: RectangleOrigin::Center
    }).insert(Robot {
        state: RobotState::DISABLED
    });
}

fn update(mut query: Query<(&Robot, &mut FieldPose)>, time: Res<Time>, keyboard_input: Res<Input<KeyCode>>) {
    let (_, mut pose): (&Robot, Mut<FieldPose>) = query.single_mut();
    let v = 5.0 * Length::new::<meter>(time.delta_seconds());
    let vr = 3.0 * Angle::new::<radian>(time.delta_seconds());
    if keyboard_input.pressed(KeyCode::A) { pose.x -= v; }
    if keyboard_input.pressed(KeyCode::D) { pose.x += v; }
    if keyboard_input.pressed(KeyCode::S) { pose.y -= v; }
    if keyboard_input.pressed(KeyCode::W) { pose.y += v; }
    if keyboard_input.pressed(KeyCode::Q) { pose.rotation += vr; }
    if keyboard_input.pressed(KeyCode::E) { pose.rotation -= vr; }
}