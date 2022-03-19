pub mod border;
pub mod event;
pub mod render;

use bevy::prelude::*;

use crate::field::Field;
use crate::layout::event::LayoutChangedEvent;

pub struct LayoutPlugin;

pub struct LayoutSettings {
    pub margin: f32,
    pub border_size: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Layout {
    pub field: LayoutRect,
    pub auto_cfg: LayoutRect,
    pub console: LayoutRect,
    pub screen_size: Vec2,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct LayoutRect {
    pub pos: Vec2,
    pub size: Vec2,
    pub border_size: f32,
}

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.world
            .get_resource_or_insert_with(|| LayoutSettings::default());
        app.insert_resource(Layout::default());

        app.add_event::<LayoutChangedEvent>();

        app.add_startup_system(border::add_borders);
        app.add_system(render::update_border);
        app.add_system(render::update_border_text);
        app.add_system(event::layout_event_update);
    }
}

impl Layout {
    fn build(settings: &LayoutSettings, width: f32, height: f32) -> Self {
        let left = -width / 2.0;
        let bottom = -height / 2.0;
        let total_usable_w = width - 2.0 * settings.margin;
        let total_usable_h = height - 2.0 * settings.margin;
        let field_w = total_usable_w.min(
            (total_usable_h * 0.7) * Field::WH_RATIO
        ).min(
            0.7 * total_usable_w
        );
        let field_h = field_w / Field::WH_RATIO;
        Layout {
            field: LayoutRect::new(
                left + settings.margin,
                bottom + settings.margin + (total_usable_h - field_h),
                field_w,
                field_h,
                settings.border_size,
            ),
            auto_cfg: LayoutRect::new(
                left + settings.margin + field_w + settings.margin,
                bottom + settings.margin + (total_usable_h - field_h),
                total_usable_w - field_w - settings.margin,
                field_h,
                settings.border_size
            ),
            console: LayoutRect::new(
                left + settings.margin,
                bottom + settings.margin,
                total_usable_w,
                total_usable_h - field_h - settings.margin,
                settings.border_size
            ),
            screen_size: Vec2::new(width, height),
        }
    }
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            margin: 20.0,
            border_size: 15.0,
        }
    }
}

impl LayoutRect {
    fn new(x: f32, y: f32, w: f32, h: f32, border_size: f32) -> Self {
        Self {
            pos: Vec2::new(x + border_size, y + border_size),
            size: Vec2::new(w - border_size * 2.0, h - border_size * 2.0),
            border_size,
        }
    }
}
