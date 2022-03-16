mod field;
mod layout;
mod border_renderer;
mod robot;

extern crate uom;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::border_renderer::BorderRendererPlugin;
use crate::field::FieldManagementPlugin;
use crate::layout::{DashboardLayoutPlugin, Layout};
use crate::robot::RobotPlugin;

fn main() {
    App::new()
        // Default Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        // MSAA and BG color
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))

        .add_plugin(DashboardLayoutPlugin)
        .add_plugin(BorderRendererPlugin)
        .add_plugin(FieldManagementPlugin)
        .add_plugin(RobotPlugin)
        .add_startup_system(setup)

        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
