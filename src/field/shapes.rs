use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::field::FieldVec;
use uom::si::f32::{Angle, Length};

#[derive(Component)]
pub struct FieldCircle(pub Length);

#[derive(Component)]
pub struct FieldRectangle {
    pub width: Length,
    pub height: Length,
    pub origin: RectangleOrigin,
}

#[derive(Component)]
pub struct FieldPath {
    pub origin: FieldVec,
    pub points: Vec<FieldVec>,
    pub rotation: Angle,
}
