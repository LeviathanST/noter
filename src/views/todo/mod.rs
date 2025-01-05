mod display_content;
mod display_todo;
mod setting;

use bevy::{
    app::{Plugin, Startup, Update},
    prelude::{
        in_state, AppExtStates, IntoSystem, IntoSystemConfigs, Res, ResMut, Resource, State,
    },
    utils::HashMap,
};
use bevy_ratatui::{error::exit_on_error, terminal::RatatuiContext};
use color_eyre::eyre::Result;
use display_todo::{poll_todos, LoadingState};
use ratatui::layout::{Constraint, Layout, Rect};

use crate::models::todo::Todo;

#[derive(Resource, Default)]
pub struct TodoResources {
    todos: Vec<Todo>,
    contents: HashMap<u64, String>,
    layout: Vec<Rect>,
}

#[derive(Default)]
pub struct TodoPlugin;

impl Plugin for TodoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<TodoResources>()
            .insert_state(LoadingState::IsDone)
            .add_systems(Startup, display_todo::retrives_todos)
            .add_systems(Update, poll_todos.run_if(in_state(LoadingState::IsLoading)))
            .add_systems(Update, render.pipe(exit_on_error));
    }
}

fn render(
    mut ctx: ResMut<RatatuiContext>,
    resources: Res<TodoResources>,
    loading_state: Res<State<LoadingState>>,
) -> Result<()> {
    ctx.draw(|f| {
        let layout_1 =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Min(30)]).split(f.area());
        let layout_2 = Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(layout_1[1]);

        f.render_widget(display_todo::widget(&resources, loading_state), layout_1[0]);
        f.render_widget(display_content::widget(&resources), layout_2[0]);
    })?;

    Ok(())
}
