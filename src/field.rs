use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::si::angle::radian;
use crate::layout::{Layout, LayoutChangedEvent};

use uom::si::f32::{Angle, Length};
use uom::si::length::{foot, meter};

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

fn setup(mut commands: Commands) {
    let hub_shape = shapes::Circle::default();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &hub_shape,
        DrawMode::Fill(FillMode::color(Color::MIDNIGHT_BLUE)),
        Transform::default(),
    )).insert(FieldPose {
        x: Length::new::<meter>(FIELD_W / 2.0),
        y: Length::new::<meter>(FIELD_H / 2.0),
        rotation: Angle::new::<radian>(0.0)
    }).insert(
        FieldCircle(Length::new::<foot>(2.0))
    );
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

impl Field {
    pub fn pose_to_screen_vec(&self, layout: &Layout, pose: &FieldPose) -> Vec2 {
        Vec2::new(
            layout.field.pos.x + layout.field.size.x * (pose.x.get::<meter>() / self.size.0.get::<meter>()),
            layout.field.pos.y + layout.field.size.y * (pose.y.get::<meter>() / self.size.1.get::<meter>())
        )
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            size: (Length::new::<meter>(FIELD_W), Length::new::<meter>(FIELD_H))
        }
    }
}
