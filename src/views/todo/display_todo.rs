use bevy::prelude::{Commands, NextState, Res, ResMut, Resource, States};
use bevy_ratatui::terminal::RatatuiContext;
use color_eyre::eyre::Result;
use crossbeam_channel::{bounded, Receiver};
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Row, Table},
};
use tokio::runtime::Runtime;

use crate::{global_resource::GlobalResource, models::todo::Todo, utils::into_row};

#[derive(States, Clone, std::cmp::Eq, PartialEq, Hash, Debug)]
pub enum LoadingState {
    IsLoading,
    IsDone,
}

#[derive(Resource)]
pub struct TodoReceiver(Receiver<Vec<Todo>>);

#[derive(Resource, Default)]
pub struct TodoResources {
    todos: Vec<Todo>,
}

pub fn render(mut context: ResMut<RatatuiContext>, resources: Res<TodoResources>) -> Result<()> {
    context.draw(|frame| {
        let constraint = [
            Constraint::Percentage(5),
            Constraint::Percentage(65),
            Constraint::Percentage(30),
        ];
        let rows = resources
            .todos
            .iter()
            .map(|todo| todo.into_row())
            .collect::<Vec<Row>>();
        let header = into_row(
            ["ID", "Name", "Passed Time"],
            Style::default().fg(Color::Yellow),
        );

        let list = Table::new(rows, constraint).header(header);
        frame.render_widget(list, frame.area())
    })?;
    Ok(())
}

// TODO: Need to make data can be sent before tx is dropped
pub fn retrives_todos(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LoadingState>>,
    global_resource: Res<GlobalResource>,
) {
    let (tx, rx) = bounded(1);
    let pool = global_resource.pool.clone();
    let rt = Runtime::new().unwrap();

    let _ = rt.spawn(async move {
        match Todo::get_all(&pool).await {
            Ok(todos) => {
                if tx.send(todos).is_err() {
                    eprintln!("Failed to send todos: Receiver dropped");
                } else {
                    println!("Send!");
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch todos: {}", err);
            }
        }
    });

    commands.insert_resource(TodoReceiver(rx));
    next_state.set(LoadingState::IsLoading);
}

pub fn poll_todos(
    mut commands: Commands,
    mut resources: ResMut<TodoResources>,
    mut next_state: ResMut<NextState<LoadingState>>,
    receiver: Res<TodoReceiver>,
) {
    match receiver.0.try_recv() {
        Ok(todos) => {
            resources.todos = todos;
            next_state.set(LoadingState::IsDone);
            commands.remove_resource::<TodoReceiver>();
        }
        Err(crossbeam_channel::TryRecvError::Disconnected) => {
            eprintln!("Receiver disconnected: Task failed or channel closed.");
            next_state.set(LoadingState::IsDone);
            commands.remove_resource::<TodoReceiver>();
        }
        Err(crossbeam_channel::TryRecvError::Empty) => {
            // Do nothing, waiting for data
        }
    }
}
