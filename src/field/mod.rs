pub mod objects;
pub mod render;
pub mod shapes;

use bevy::prelude::*;

use uom::si::angle::radian;
use uom::si::f32::{Angle, Length};
use uom::si::length::meter;

use crate::layout::Layout;

pub struct FieldManagementPlugin;

pub struct Field {
    pub size: FieldVec,
}

#[derive(Debug)]
pub struct FieldVec {
    pub x: Length,
    pub y: Length,
}

#[derive(Component, Debug)]
pub struct FieldPose {
    pub pos: FieldVec,
    pub rotation: Angle,
}

impl Plugin for FieldManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(objects::spawn_objects);
        app.add_system(render::field_pose_updater);
        app.add_system(render::field_circle_updater);
        app.add_system(render::field_rect_updater);
        app.add_system(render::field_path_updater);
        app.insert_resource(Field::default());
    }
}

impl Field {
    const WIDTH_M: f32 = 16.4592;
    const HEIGHT_M: f32 = 8.2296;

    // Probably not the best way to do this
    const WIDTH: fn() -> Length = || Length::new::<meter>(Field::WIDTH_M);
    const HEIGHT: fn() -> Length = || Length::new::<meter>(Field::HEIGHT_M);

    pub const WH_RATIO: f32 = 2.0;

    pub fn to_screen_vec(&self, layout: &Layout, pos: &FieldVec) -> Vec2 {
        Vec2::new(
            layout.field.pos.x
                + layout.field.size.x * (pos.x.get::<meter>() / self.size.x.get::<meter>()),
            layout.field.pos.y
                + layout.field.size.y * (pos.y.get::<meter>() / self.size.y.get::<meter>()),
        )
    }

    pub fn to_screen_transform(&self, layout: &Layout, pose: &FieldPose, z: f32) -> Transform {
        Transform::from_translation(self.to_screen_vec(&layout, &pose.pos).extend(z))
            .with_rotation(Quat::from_rotation_z(pose.rotation.get::<radian>()))
    }
}

impl FieldVec {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
}

impl FieldPose {
    pub fn new(pos: FieldVec, rotation: Angle) -> Self {
        Self { pos, rotation }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            size: FieldVec::new(Field::WIDTH(), Field::HEIGHT()),
        }
    }
}
