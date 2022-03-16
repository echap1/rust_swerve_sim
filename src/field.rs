use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::ConstZero;
use uom::num_traits::Pow;
use uom::si::angle::{degree, radian};
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
    pub size: FieldVec
}

#[derive(Debug)]
pub struct FieldVec {
    pub x: Length,
    pub y: Length
}

#[derive(Component, Debug)]
pub struct FieldPose {
    pub pos: FieldVec,
    pub rotation: Angle,
    pub z: f32
}

#[derive(Component)]
pub struct FieldCircle(Length);

#[derive(Component)]
pub struct FieldRectangle {
    pub width: Length,
    pub height: Length,
    pub origin: RectangleOrigin
}

#[derive(Component)]
pub struct FieldPath {
    pub origin: FieldVec,
    pub points: Vec<FieldVec>,
    pub rotation: Angle,
    pub z: f32
}

fn setup(mut commands: Commands) {
    let default_shape = shapes::Circle::default();

    // HUB
    commands.spawn_bundle(GeometryBuilder::build_as(
        &default_shape,
        DrawMode::Fill(FillMode::color(Color::rgb(0.8, 0.8, 0.8))),
        Transform::default(),
    )).insert(FieldPose {
        pos: FieldVec::new(Length::new::<meter>(FIELD_W / 2.0), Length::new::<meter>(FIELD_H / 2.0)),
        rotation: Angle::new::<radian>(0.0),
        z: Field::FIELD_OBJECTS_Z
    }).insert(
        FieldCircle(Length::new::<foot>(2.0))
    );
    let k: f32 = (237.31_f32 / 2.0).pow(2) - (219.25_f32 / 2.0).pow(2);
    let k = k.sqrt();

    spawn_tarmac(&mut commands, FieldVec::new(
        Length::new::<meter>(FIELD_W / 2.0) - Length::new::<inch>(k),
        Length::new::<meter>(FIELD_H / 2.0) - Length::new::<inch>(219.25 / 2.0)
    ), Angle::ZERO, Color::BLUE);

    spawn_tarmac(&mut commands, FieldVec::new(
        Length::new::<meter>(FIELD_W / 2.0) + Length::new::<inch>(k),
        Length::new::<meter>(FIELD_H / 2.0) + Length::new::<inch>(219.25 / 2.0)
    ), Angle::new::<degree>(180.0), Color::RED);

    spawn_tarmac(&mut commands, FieldVec::new(
        Length::new::<meter>(FIELD_W / 2.0) + Length::new::<inch>(219.25 / 2.0),
        Length::new::<meter>(FIELD_H / 2.0) - Length::new::<inch>(k)
    ), Angle::new::<degree>(90.0), Color::RED);

    spawn_tarmac(&mut commands, FieldVec::new(
        Length::new::<meter>(FIELD_W / 2.0) - Length::new::<inch>(219.25 / 2.0),
        Length::new::<meter>(FIELD_H / 2.0) + Length::new::<inch>(k)
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
        rotation,
        z: Field::FIELD_OBJECTS_Z
    });
}

// Updates the position and rotation of field-relative sprites to reflect their pose
fn field_pose_updater(field: Res<Field>, layout: Res<Layout>, mut query: Query<(&FieldPose, &mut Transform)>) {
    for i in query.iter_mut() {
        let (pose, mut transform): (&FieldPose, Mut<Transform>) = i;
        *transform = field.to_screen_transform(&layout, &pose);
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
                    radius: layout.field.size.x * c.0.get::<meter>() / field.size.x.get::<meter>(),
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
                        layout.field.size.x * r.width.get::<meter>() / field.size.x.get::<meter>(),
                        layout.field.size.x * r.height.get::<meter>() / field.size.x.get::<meter>()
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

    builder.move_to(Vec2::ZERO);

    let mut cum_x = path.origin.x;
    let mut cum_y = path.origin.y;

    let start_screen_vec = field.to_screen_vec(layout, &path.origin);

    for p in &path.points {
        let pose = FieldVec::new(p.x + cum_x, p.y + cum_y);
        cum_x += p.x;
        cum_y += p.y;
        builder.line_to(field.to_screen_vec(layout, &pose) - start_screen_vec);
    }

    builder.close();
    builder.build()
}

fn field_path_updater(field: Res<Field>, layout_changed_event: Res<Events<LayoutChangedEvent>>, mut query: Query<(&FieldPath, &mut Path, &mut Transform)>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (p, mut path, mut transform): (&FieldPath, Mut<Path>, Mut<Transform>) = i;
                *path = build_field_path(p, &field, layout);
                *transform = Transform {
                    translation: field.to_screen_vec(&layout, &p.origin).extend(p.z),
                    rotation: Quat::from_rotation_z(p.rotation.get::<radian>()),
                    ..*transform
                }
            }
        }
    };
}

impl Field {
    const FIELD_OBJECTS_Z: f32 = 0.0;
    const ROBOT_Z: f32 = 1.0;

    pub fn to_screen_vec(&self, layout: &Layout, pos: &FieldVec) -> Vec2 {
        Vec2::new(
            layout.field.pos.x + layout.field.size.x * (pos.x.get::<meter>() / self.size.x.get::<meter>()),
            layout.field.pos.y + layout.field.size.y * (pos.y.get::<meter>() / self.size.y.get::<meter>())
        )
    }

    pub fn to_screen_transform(&self, layout: &Layout, pose: &FieldPose) -> Transform {
        Transform::from_translation(self.to_screen_vec(&layout, &pose.pos).extend(pose.z)).with_rotation(Quat::from_rotation_z(pose.rotation.get::<radian>()))
    }
}

impl FieldVec {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
}

impl FieldPose {
    pub fn new(pos: FieldVec, rotation: Angle, z: f32) -> Self {
        Self { pos, rotation, z }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            size: FieldVec::new(Length::new::<meter>(FIELD_W), Length::new::<meter>(FIELD_H))
        }
    }
}
