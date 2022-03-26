mod config_panel;
mod waypoints;
mod trajectory;

use bevy::prelude::*;

pub struct AutoPathingPlugin;


impl Plugin for AutoPathingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(config_panel::setup);
        app.add_startup_system(waypoints::setup);
        app.add_system(config_panel::root_updater);
        app.add_system(waypoints::waypoint_updater);
        app.add_system(waypoints::rotation_anchor_updater);
        app.add_system(trajectory::trajectory_updater);
        app.add_system(trajectory::trajectory_path_updater);
        app.add_system(waypoints::waypoint_grab_system);
    }
}

