use bevy::prelude::*;

use crate::Layout;
use crate::layout::{LayoutRect, render};

#[derive(Component)]
pub struct BorderElement {
    pub layout_rect: fn(&Layout) -> &LayoutRect,
    pub color: Color
}

pub fn add_borders(mut commands: Commands, layout: Res<Layout>, asset_server: Res<AssetServer>) {
    generate_border(&mut commands, &layout,|l| &l.field, "Field", Color::rgb(0.7, 0.7, 1.0), &asset_server);
    generate_border(&mut commands, &layout, |l| &l.auto_cfg, "Autonomous Config", Color::BLUE, &asset_server);
    generate_border(&mut commands, &layout, |l| &l.console, "Console", Color::GRAY, &asset_server);
}

pub fn generate_border(commands: &mut Commands, layout: &Layout, location: fn(&Layout) -> &LayoutRect, name: &str, color: Color, asset_server: &AssetServer) {
    commands.spawn_bundle(
        render::get_border_geometry(location(layout), color.clone())
    ).insert(BorderElement {
        layout_rect: location,
        color
    });

    commands.spawn_bundle(
        render::get_border_text(name.to_string(), asset_server)
    ).insert(BorderElement {
        layout_rect: location,
        color
    });
}