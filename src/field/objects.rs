use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::ConstZero;
use uom::num_traits::Pow;
use uom::si::angle::{degree, radian};

use uom::si::f32::{Angle, Length};
use uom::si::length::{foot, inch};

use crate::field::{Field, FieldPose, FieldVec};
use crate::field::render::FieldZ;
use crate::field::shapes::{FieldCircle, FieldPath};

pub fn spawn_objects(mut commands: Commands) {
    let default_shape = shapes::Circle::default();

    // HUB
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Fill(FillMode::color(Color::rgb(0.8, 0.8, 0.8))),
        Transform::default(),
    )).insert(FieldPose {
        pos: FieldVec::new(Field::WIDTH() / 2.0, Field::HEIGHT() / 2.0),
        rotation: Angle::new::<radian>(0.0)
    }).insert(
        FieldCircle(Length::new::<foot>(2.0))
    ).insert(FieldZ::FIELD_OBJECTS);

    let small_offset: f32 = (237.31_f32 / 2.0).pow(2) - (219.25_f32 / 2.0).pow(2);
    let small_offset = small_offset.sqrt();

    spawn_tarmac(&mut commands, FieldVec::new(
        Field::WIDTH() / 2.0 - Length::new::<inch>(small_offset),
        Field::HEIGHT() / 2.0 - Length::new::<inch>(219.25 / 2.0)
    ), Angle::ZERO, Color::BLUE);

    spawn_tarmac(&mut commands, FieldVec::new(
        Field::WIDTH() / 2.0 + Length::new::<inch>(small_offset),
        Field::HEIGHT() / 2.0 + Length::new::<inch>(219.25 / 2.0)
    ), Angle::new::<degree>(180.0), Color::RED);

    spawn_tarmac(&mut commands, FieldVec::new(
        Field::WIDTH() / 2.0 + Length::new::<inch>(219.25 / 2.0),
        Field::HEIGHT() / 2.0 - Length::new::<inch>(small_offset)
    ), Angle::new::<degree>(90.0), Color::RED);

    spawn_tarmac(&mut commands, FieldVec::new(
        Field::WIDTH() / 2.0 - Length::new::<inch>(219.25 / 2.0),
        Field::HEIGHT() / 2.0 + Length::new::<inch>(small_offset)
    ), Angle::new::<degree>(-90.0), Color::BLUE);
}

fn spawn_tarmac(commands: &mut Commands, origin: FieldVec, rotation: Angle, color: Color) {
    let tape_path = vec![
        (82.83, 0.0),
        (-75.07 * 67.5_f32.to_radians().cos(), 75.07 * 67.5_f32.to_radians().sin()),
        (-46.89 * (135.0_f32 - (90.0 - 67.5)).to_radians().sin(), -46.89 * (135.0_f32 - (90.0 - 67.5)).to_radians().cos()),
        (-75.07 * (360.0_f32 - (135.0 * 2.0 + 67.6)).to_radians().cos(), -75.07 * (360.0_f32 - (135.0 * 2.0 + 67.6)).to_radians().sin())
    ];
    let tape_path: Vec<FieldVec> = tape_path.iter().map(|(x, y)| FieldVec::new(Length::new::<inch>(*x), Length::new::<inch>(*y))).collect();
    let default_shape = shapes::Circle::default();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Stroke(StrokeMode::new(color, 2.0)),
        Transform::default()
    )).insert(FieldPath {
        points: tape_path,
        origin,
        rotation
    }).insert(FieldZ::FIELD_OBJECTS);
}
