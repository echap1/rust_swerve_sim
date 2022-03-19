use bevy::app::Events;
use bevy::prelude::*;
use crate::Layout;
use crate::layout::event::LayoutChangedEvent;
use crate::layout::render::FONT_SIZE;

#[derive(Component)]
pub struct ConfigRoot;

pub enum ConfigButtonAction {
    None
}

#[derive(Component)]
pub struct ConfigButton {
    // action: ConfigButtonAction
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(NodeBundle {
        color: UiColor(Color::NONE),
        ..NodeBundle::default()
    }).with_children(|parent| {
        generate_button(parent, Color::GRAY, Color::BLACK, "helo".to_string(), &asset_server)
    }).insert(ConfigRoot {

    });
}

fn generate_button(parent: &mut ChildBuilder, color: Color, text_color: Color, text: String, asset_server: &AssetServer) {
    parent.spawn_bundle(ButtonBundle {
        color: UiColor(color),
        style: Style {
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|button_parent| {
        button_parent.spawn_bundle(TextBundle {
            style: Style {
                margin: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            text: Text::with_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: FONT_SIZE,
                    color: text_color,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        });
    }).insert(ConfigButton {});
}

pub fn root_updater(
    mut query: Query<(&ConfigRoot, &mut Style)>,
    layout_changed_event: Res<Events<LayoutChangedEvent>>
) {
    match layout_changed_event.get_reader().iter(&layout_changed_event).next_back() {
        None => {}
        Some(e) => {
            let layout: &Layout = &e.0;

            for i in query.iter_mut() {
                let (_, mut style): (&ConfigRoot, Mut<Style>) = i;
                *style = Style {
                    size: Size::new(Val::Px(layout.auto_cfg.size.x), Val::Px(layout.auto_cfg.size.y - FONT_SIZE)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(layout.auto_cfg.pos.x + (layout.screen_size.x / 2.0)),
                        bottom: Val::Px(layout.auto_cfg.pos.y + (layout.screen_size.y / 2.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                };
            }
        }
    };
}