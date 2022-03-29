pub mod objects;
pub mod render;
pub mod shapes;

use bevy::prelude::*;
use uom::num_traits::Pow;

use uom::si::angle::radian;
use uom::si::f32::{Angle, Length};
use uom::si::length::meter;

use crate::layout::Layout;

use serde::{Serialize, Deserialize, Serializer};

pub struct FieldManagementPlugin;

pub struct Field {
    pub size: FieldPosition,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FieldPosition {
    pub x: Length,
    pub y: Length,
}

#[derive(Component, Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FieldPose {
    pub translation: FieldPosition,
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

    pub fn to_screen_vec(&self, layout: &Layout, pos: &FieldPosition) -> Vec2 {
        Vec2::new(
            layout.field.pos.x
                + layout.field.size.x * (pos.x.get::<meter>() / self.size.x.get::<meter>()),
            layout.field.pos.y
                + layout.field.size.y * (pos.y.get::<meter>() / self.size.y.get::<meter>()),
        )
    }

    pub fn to_screen_transform(&self, layout: &Layout, pose: &FieldPose, z: f32) -> Transform {
        Transform::from_translation(self.to_screen_vec(&layout, &pose.translation).extend(z))
            .with_rotation(Quat::from_rotation_z(pose.rotation.get::<radian>()))
    }

    pub fn to_field_position(&self, layout: &Layout, pos: Vec2) -> Option<FieldPosition> {
        return if pos.x < layout.field.pos.x
            || pos.y < layout.field.pos.y
            || pos.x > layout.field.pos.x + layout.field.size.x
            || pos.y > layout.field.pos.y + layout.field.size.y
        {
            None
        } else {
            Some(FieldPosition::new(
                self.size.x * ((pos.x - layout.field.pos.x) / layout.field.size.x),
                self.size.y * ((pos.y - layout.field.pos.y) / layout.field.size.y)
            ))
        }
    }
}

impl FieldPosition {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
    
    pub fn dist(&self, other: &FieldPosition) -> Length {
        let mx: f32 = (self.x - other.x).get::<meter>().pow(2);
        let my: f32 = (self.y - other.y).get::<meter>().pow(2);
        Length::new::<meter>((mx + my).sqrt())
    }
}

impl FieldPose {
    pub fn new(pos: FieldPosition, rotation: Angle) -> Self {
        Self { translation: pos, rotation }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            size: FieldPosition::new(Field::WIDTH(), Field::HEIGHT()),
        }
    }
}
