use bevy::app::Events;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use crate::Layout;
use crate::layout::{LayoutChangedEvent, LayoutRect};

pub struct BorderRendererPlugin;

static FONT_SIZE: f32 = 30.0;

#[derive(Component)]
struct BorderElement {
    layout_rect: fn(&Layout) -> &LayoutRect,
    color: Color
}

impl Plugin for BorderRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(update_border);
        app.add_system(update_border_text);
    }
}

fn setup(mut commands: Commands, layout: Res<Layout>, asset_server: Res<AssetServer>) {
    generate_border(&mut commands, &layout,|l| &l.field, "Field", Color::rgb(0.7, 0.7, 1.0), &asset_server);
    // generate_border(&mut commands, &layout,|l| &l.console, "Vision Target", Color::BLUE, &asset_server);
}

#[inline]
fn get_border_geometry(location: &LayoutRect, color: Color) -> ShapeBundle {
    let shape = shapes::Rectangle {
        extents: location.size + (2.0 * location.border_size),
        origin: RectangleOrigin::BottomLeft
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Stroke(StrokeMode::new(color, location.border_size / 3.0)),
        Transform::from_translation((location.pos - location.border_size).extend(0.0))
    )
}

fn get_border_text(name: String, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect::default(),
            ..Style::default()
        },
        text: Text::with_section(
            name,
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
            TextAlignment::default(),
        ),
        ..TextBundle::default()
    }
}

fn get_border_text_position(location: &LayoutRect, layout: &Layout) -> Rect<Val> {
    Rect {
        bottom: Val::Px(location.pos.y + (layout.screen_size.y / 2.0) + location.size.y + (location.border_size / 2.0) - FONT_SIZE),
        left: Val::Px(location.pos.x + (layout.screen_size.x / 2.0) - (location.border_size / 2.0)),
        ..Rect::default()
    }
}

fn generate_border(commands: &mut Commands, layout: &Layout, location: fn(&Layout) -> &LayoutRect, name: &str, color: Color, asset_server: &AssetServer) {
    commands.spawn_bundle(
        get_border_geometry(location(layout), color.clone())
    ).insert(BorderElement {
        layout_rect: location,
        color
    });

    commands.spawn_bundle(
        get_border_text(name.to_string(), asset_server)
    ).insert(BorderElement {
        layout_rect: location,
        color
    });
}

fn update_border(mut query: Query<(&BorderElement, &mut Path, &mut Transform, &mut DrawMode)>, layout_changed_event: Res<Events<LayoutChangedEvent>>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (element, mut path, mut transform, mut mode): (&BorderElement, Mut<Path>, Mut<Transform>, Mut<DrawMode>) = i;
                let geometry = get_border_geometry((element.layout_rect)(layout), element.color.clone());
                *path = geometry.path;
                *transform = geometry.transform;
                *mode = geometry.mode;
            }
        }
    };
}


fn update_border_text(mut query: Query<(&BorderElement, &mut Style)>, layout_changed_event: Res<Events<LayoutChangedEvent>>) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (element, mut style): (&BorderElement, Mut<Style>) = i;
                let location = (element.layout_rect)(layout);
                style.position = get_border_text_position(location, layout);
            }
        }
    };
}
