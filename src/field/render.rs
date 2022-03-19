use crate::layout::Layout;
use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use uom::si::angle::radian;
use uom::si::length::meter;

use crate::field::shapes::{FieldCircle, FieldPath, FieldRectangle};
use crate::field::{Field, FieldPose, FieldPosition};
use crate::layout::event::LayoutChangedEvent;

#[derive(Component)]
pub struct FieldZ(pub f32);

impl FieldZ {
    pub const FIELD_OBJECTS: FieldZ = FieldZ(0.0);
    pub const AUTO_PATH: FieldZ = FieldZ(1.0);
    pub const AUTO_WAYPOINTS: FieldZ = FieldZ(2.0);
    pub const ROBOT: FieldZ = FieldZ(3.0);
}

// Updates the position and rotation of field-relative sprites to reflect their pose
pub fn field_pose_updater(
    field: Res<Field>,
    layout: Res<Layout>,
    mut query: Query<(&FieldPose, &mut Transform, Option<&FieldZ>)>,
) {
    for i in query.iter_mut() {
        let (pose, mut transform, z): (&FieldPose, Mut<Transform>, Option<&FieldZ>) = i;
        *transform = field.to_screen_transform(
            &layout,
            &pose,
            match z {
                None => 0_f32,
                Some(field_z) => field_z.0,
            },
        );
    }
}

pub fn field_circle_updater(
    field: Res<Field>,
    layout_changed_event: Res<Events<LayoutChangedEvent>>,
    mut query: Query<(&FieldCircle, &mut Path)>,
) {
    match layout_changed_event
        .get_reader()
        .iter(&layout_changed_event)
        .next_back()
    {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (c, mut path): (&FieldCircle, Mut<Path>) = i;
                let shape = shapes::Circle {
                    radius: layout.field.size.x * c.0.get::<meter>() / field.size.x.get::<meter>(),
                    center: Default::default(),
                };
                let geometry = GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::default())),
                    Default::default(),
                );
                *path = geometry.path;
            }
        }
    };
}

pub fn field_rect_updater(
    field: Res<Field>,
    layout_changed_event: Res<Events<LayoutChangedEvent>>,
    mut query: Query<(&FieldRectangle, &mut Path)>,
) {
    match layout_changed_event
        .get_reader()
        .iter(&layout_changed_event)
        .next_back()
    {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (r, mut path): (&FieldRectangle, Mut<Path>) = i;
                let shape = shapes::Rectangle {
                    extents: Vec2::new(
                        layout.field.size.x * r.width.get::<meter>() / field.size.x.get::<meter>(),
                        layout.field.size.x * r.height.get::<meter>() / field.size.x.get::<meter>(),
                    ),
                    origin: r.origin,
                };
                let geometry = GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::default())),
                    Default::default(),
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
        let pose = FieldPosition::new(p.x + cum_x, p.y + cum_y);
        cum_x += p.x;
        cum_y += p.y;
        builder.line_to(field.to_screen_vec(layout, &pose) - start_screen_vec);
    }

    builder.close();
    builder.build()
}

pub fn field_path_updater(
    field: Res<Field>,
    layout_changed_event: Res<Events<LayoutChangedEvent>>,
    mut query: Query<(&FieldPath, &mut Path, &mut Transform, Option<&FieldZ>)>,
) {
    match layout_changed_event
        .get_reader()
        .iter(&layout_changed_event)
        .next_back()
    {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (p, mut path, mut transform, z): (
                    &FieldPath,
                    Mut<Path>,
                    Mut<Transform>,
                    Option<&FieldZ>,
                ) = i;

                let z = match z {
                    None => 0_f32,
                    Some(field_z) => field_z.0,
                };

                *path = build_field_path(p, &field, layout);
                *transform = Transform {
                    translation: field.to_screen_vec(&layout, &p.origin).extend(z),
                    rotation: Quat::from_rotation_z(p.rotation.get::<radian>()),
                    ..*transform
                }
            }
        }
    };
}
