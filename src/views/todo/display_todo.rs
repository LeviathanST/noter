use bevy::{
    app::AppExit,
    prelude::{
        Commands, EventReader, EventWriter, NextState, Res, ResMut, Resource, State, States,
    },
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
    utils::HashMap,
};
use bevy_ratatui::event::KeyEvent;
use ratatui::{
    layout::Constraint,
    prelude::Stylize,
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table, Widget},
};

use crate::{
    global_resource::GlobalResource, models::todo::Todo, utils::into_row, views::NoterKeyEvent,
};

use super::TodoResources;

#[derive(States, Clone, std::cmp::Eq, PartialEq, Hash, Debug)]
pub enum LoadingState {
    IsLoading,
    IsDone,
}

#[derive(Resource)]
pub struct DisplayTodoTask(Task<Vec<Todo>>);

pub fn widget(
    resources: &Res<TodoResources>,
    loading_state: Res<State<LoadingState>>,
) -> impl Widget {
    let constraint = [
        Constraint::Percentage(5),
        Constraint::Percentage(65),
        Constraint::Percentage(30),
    ];
    let rows = match loading_state.get() {
        LoadingState::IsLoading => vec![Row::new(["Loading", "Loading", "Loading"])],
        LoadingState::IsDone => resources
            .todos
            .iter()
            .map(|todo| todo.into_row())
            .collect::<Vec<Row>>(),
    };
    let header = into_row(
        ["ID", "Name", "Passed Time"],
        Style::default().fg(Color::Yellow),
    );
    let block = Block::default()
        .title("Todo")
        .fg(if resources.selected_block_idx == 0 {
            Color::Green
        } else {
            Color::White
        })
        .borders(Borders::ALL);

    Table::new(rows, constraint).header(header).block(block)
}

pub fn key_binding(
    mut events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    global_resources: Res<GlobalResource>,
    resources: Res<TodoResources>,
) {
    if resources.selected_block_idx == 0 {
        let action_map = &global_resources.action;
        for event in events.read() {
            let noter_event = NoterKeyEvent::from(event.clone());
            if let Some(action) = action_map.0.get(&noter_event) {
                match action.as_str() {
                    "quit_program" => {
                        exit.send_default();
                    }
                    _ => {}
                };
            }
        }
    }
}

pub fn retrives_todos(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LoadingState>>,
    global_resource: Res<GlobalResource>,
) {
    let pool = global_resource.pool.clone();

    let task = AsyncComputeTaskPool::get().spawn(async move {
        match Todo::get_all(&pool).await {
            Ok(todos) => todos,
            Err(_) => vec![],
        }
    });

    commands.insert_resource(DisplayTodoTask(task));
    next_state.set(LoadingState::IsLoading);
}

pub fn poll_todos(
    mut commands: Commands,
    mut resources: ResMut<TodoResources>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut todo_task: ResMut<DisplayTodoTask>,
) {
    if let Some(todos) = block_on(future::poll_once(&mut todo_task.0)) {
        resources.contents = todos
            .iter()
            .map(|todo| (todo.id as u64, todo.description.to_string()))
            .collect::<HashMap<u64, String>>();

        resources.todos = todos;
        next_state.set(LoadingState::IsDone);
        commands.remove_resource::<DisplayTodoTask>();
    }
}
