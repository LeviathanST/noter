use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::PluginGroup,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_ratatui::RatatuiPlugins;
use crossterm::terminal::enable_raw_mode;
use global_resource::GlobalResource;
use views::todo::TodoPlugin;

mod global_resource;
mod models;
mod utils;
mod views;

fn main() {
    enable_raw_mode().unwrap();
    let wait_duration = std::time::Duration::from_secs_f64(1. / 60.);

    App::new()
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(StatesPlugin::default())
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(wait_duration)))
        .init_resource::<GlobalResource>()
        .add_plugins(TodoPlugin::default())
        .run();
}
