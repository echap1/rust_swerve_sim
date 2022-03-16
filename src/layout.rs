use bevy::app::Events;
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::field::FIELD_WH_RATIO;


pub struct DashboardLayoutPlugin;

pub struct LayoutChangedEvent(pub Layout);

pub struct DashboardLayoutSettings {
    pub margin: f32,
    pub border_size: f32
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Layout {
    pub field: LayoutRect,
    // pub console: LayoutRect,
    pub screen_size: Vec2,
}

impl Plugin for DashboardLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.world.get_resource_or_insert_with(|| DashboardLayoutSettings::default());
        app.insert_resource(Layout::default());
        app.add_system(layout_update_system);
        app.add_event::<LayoutChangedEvent>();
    }
}

impl Layout {
    fn build(settings: &DashboardLayoutSettings, width: f32, height: f32) -> Self {
        let left = -width / 2.0;
        let bottom = -height / 2.0;
        let total_usable_w = width - 2.0 * settings.margin;
        let total_usable_h = height - 2.0 * settings.margin;
        let field_w = total_usable_w.min(total_usable_h * FIELD_WH_RATIO);
        let field_h = field_w / FIELD_WH_RATIO;
        Layout {
            field: LayoutRect::new(
                left + settings.margin,
                bottom + settings.margin,
                field_w,
                field_h,
                settings.border_size
            ),
            // console: LayoutRect::new(
            //     left + settings.margin + field_w + settings.margin,
            //     bottom + settings.margin,
            //     console_w,
            //     0.5 * total_usable_h,
            //     settings.border_size
            // ),
            screen_size: Vec2::new(width, height)
        }
    }
}

fn layout_update_system(resize_event: Res<Events<WindowResized>>, mut layout_changed_event: ResMut<Events<LayoutChangedEvent>>, mut layout: ResMut<Layout>, settings: Res<DashboardLayoutSettings>) {
    match resize_event.get_reader().iter(&resize_event).next_back() {
        None => {}
        Some(e) => {
            let new_screen_size = Vec2::new(e.width, e.height);
            if new_screen_size != layout.screen_size {
                *layout = Layout::build(&settings, e.width, e.height);
                layout_changed_event.send(LayoutChangedEvent(*layout))
            }
        }
    };
}

#[derive(Default, Copy, Clone, Debug)]
pub struct LayoutRect {
    pub pos: Vec2,
    pub size: Vec2,
    pub border_size: f32
}

impl Default for DashboardLayoutSettings {
    fn default() -> Self {
        Self {
            margin: 20.0,
            border_size: 15.0
        }
    }
}

impl LayoutRect {
    fn new(x: f32, y: f32, w: f32, h: f32, border_size: f32) -> Self {
        Self {
            pos: Vec2::new(x + border_size, y + border_size),
            size: Vec2::new(w - border_size * 2.0, h - border_size * 2.0),
            border_size
        }
    }
}