use std::f32::consts::PI;
use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::num_traits::Pow;
use uom::si::angle::radian;
use crate::layout::{Layout, LayoutChangedEvent};

use uom::si::f32::{Angle, Length};
use uom::si::length::{foot, inch, meter};

pub struct FieldManagementPlugin;

const FIELD_W: f32 = 16.4592;
const FIELD_H: f32 = 8.2296;

pub const FIELD_WH_RATIO: f32 = 2.0;

impl Plugin for FieldManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(field_pose_updater);
        app.add_system(field_circle_updater);
        app.add_system(field_rect_updater);
        app.add_system(field_path_updater);
        app.insert_resource(Field::default());
    }
}

pub struct Field {
    pub size: (Length, Length)
}

#[derive(Component, Debug)]
pub struct FieldPose {
    pub x: Length,
    pub y: Length,
    pub rotation: Angle
}

#[derive(Component)]
pub struct FieldCircle(Length);

#[derive(Component)]
pub struct FieldRectangle {
    pub width: Length,
    pub height: Length,
    pub origin: RectangleOrigin,
}

#[derive(Component)]
pub struct FieldPath {
    pub start: (Length, Length),
    pub points: Vec<(Length, Length)>
}

fn setup(mut commands: Commands) {
    let default_shape = shapes::Circle::default();

    // HUB
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Fill(FillMode::color(Color::MIDNIGHT_BLUE)),
        Transform::default(),
    )).insert(FieldPose {
        x: Length::new::<meter>(FIELD_W / 2.0),
        y: Length::new::<meter>(FIELD_H / 2.0),
        rotation: Angle::new::<radian>(0.0)
    }).insert(
        FieldCircle(Length::new::<foot>(2.0))
    );

    let tape_path = vec![
        (82.83, 0.0),
        (-75.07 * 67.5_f32.to_radians().cos(), 75.07 * 67.5_f32.to_radians().sin()),
        (-46.89 * (135.0_f32 - (90.0 - 67.5)).to_radians().sin(), -46.89 * (135.0_f32 - (90.0 - 67.5)).to_radians().cos()),
        (-75.07 * (360.0_f32 - (135.0 * 2.0 + 67.6)).to_radians().cos(), -75.07 * (360.0_f32 - (135.0 * 2.0 + 67.6)).to_radians().sin())
    ];
    let tape_path: Vec<(Length, Length)> = tape_path.iter().map(|(x, y)| (Length::new::<inch>(*x), Length::new::<inch>(*y))).collect();
    let k: f32 = (237.31_f32 / 2.0).pow(2) - (219.25_f32 / 2.0).pow(2);
    let k = k.sqrt();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Stroke(StrokeMode::new(Color::BLUE, 2.0)),
        Transform::default()
    )).insert(FieldPath {
        points: tape_path,
        start: (
            Length::new::<meter>(FIELD_W / 2.0) - Length::new::<inch>(k),
            Length::new::<meter>(FIELD_H / 2.0) - Length::new::<inch>(219.25 / 2.0)
        )
    });
}

// Updates the position and rotation of field-relative sprites to reflect their pose
fn field_pose_updater(field: Res<Field>, layout: Res<Layout>, mut query: Query<(&FieldPose, &mut Transform)>) {
    for i in query.iter_mut() {
        let (pose, mut transform): (&FieldPose, Mut<Transform>) = i;
        transform.translation = field.pose_to_screen_vec(&layout, pose).extend(0f32);
        transform.rotation = Quat::from_rotation_z(pose.rotation.get::<radian>())
    }
}

fn field_circle_updater(field: Res<Field>, layout_changed_event: Res<Events<LayoutChangedEvent>>, mut query: Query<(&FieldCircle, &mut Path)>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (c, mut path): (&FieldCircle, Mut<Path>) = i;
                let shape = shapes::Circle {
                    radius: layout.field.size.x * c.0.get::<meter>() / field.size.0.get::<meter>(),
                    center: Default::default()
                };
                let geometry = GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::default())),
                    Default::default()
                );
                *path = geometry.path;
            }
        }
    };
}

fn field_rect_updater(field: Res<Field>, layout_changed_event: Res<Events<LayoutChangedEvent>>, mut query: Query<(&FieldRectangle, &mut Path)>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (r, mut path): (&FieldRectangle, Mut<Path>) = i;
                let shape = shapes::Rectangle {
                    extents: Vec2::new(
                        layout.field.size.x * r.width.get::<meter>() / field.size.0.get::<meter>(),
                        layout.field.size.x * r.height.get::<meter>() / field.size.0.get::<meter>()
                    ),
                    origin: r.origin
                };
                let geometry = GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::default())),
                    Default::default()
                );
                *path = geometry.path;
            }
        }
    };
}

fn build_field_path(path: &FieldPath, field: &Field, layout: &Layout) -> Path {
    let mut builder = PathBuilder::new();

    let start_pose = FieldPose::new(path.start.0, path.start.1, Angle::default());
    builder.move_to(field.pose_to_screen_vec(layout, &start_pose));

    let mut cum_x = start_pose.x;
    let mut cum_y = start_pose.y;

    for (px, py) in &path.points {
        let pose = FieldPose::new(*px + cum_x, *py + cum_y, Angle::default());
        cum_x += *px;
        cum_y += *py;
        builder.line_to(field.pose_to_screen_vec(layout, &pose));
    }

    builder.close();
    builder.build()
}

fn field_path_updater(field: Res<Field>, layout_changed_event: Res<Events<LayoutChangedEvent>>, mut query: Query<(&FieldPath, &mut Path)>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (p, mut path): (&FieldPath, Mut<Path>) = i;
                *path = build_field_path(p, &field, layout);
            }
        }
    };
}

impl Field {
    pub fn pose_to_screen_vec(&self, layout: &Layout, pose: &FieldPose) -> Vec2 {
        Vec2::new(
            layout.field.pos.x + layout.field.size.x * (pose.x.get::<meter>() / self.size.0.get::<meter>()),
            layout.field.pos.y + layout.field.size.y * (pose.y.get::<meter>() / self.size.1.get::<meter>())
        )
    }
}

impl FieldPose {
    pub fn new(x: Length, y: Length, rotation: Angle) -> Self {
        Self { x, y, rotation }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            size: (Length::new::<meter>(FIELD_W), Length::new::<meter>(FIELD_H))
        }
    }
}
