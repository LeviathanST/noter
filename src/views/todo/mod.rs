mod display_todo;

use bevy::{
    app::{Plugin, Startup, Update},
    prelude::{in_state, AppExtStates, IntoSystem, IntoSystemConfigs},
};
use bevy_ratatui::error::exit_on_error;
use display_todo::{poll_todos, LoadingState, TodoResources};

#[derive(Default)]
pub struct TodoPlugin;

impl Plugin for TodoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<TodoResources>()
            .insert_state(LoadingState::IsDone)
            .add_systems(Startup, display_todo::retrives_todos)
            .add_systems(Update, poll_todos.run_if(in_state(LoadingState::IsLoading)))
            .add_systems(Update, display_todo::render.pipe(exit_on_error));
    }
}
