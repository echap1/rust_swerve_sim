use bevy::app::Events;
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::Layout;
use crate::layout::LayoutSettings;

pub struct LayoutChangedEvent(pub Layout);

pub fn layout_event_update(
    resize_event: Res<Events<WindowResized>>,
    mut layout_changed_event: ResMut<Events<LayoutChangedEvent>>,
    mut layout: ResMut<Layout>,
    settings: Res<LayoutSettings>,
) {
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
