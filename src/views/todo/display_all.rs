use sqlx::types::chrono::{self, Local, NaiveDate, NaiveDateTime, TimeZone};
use widgetui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    ratatui::{
        layout::Constraint,
        style::{Color, Style, Stylize},
        widgets::{Block, Borders, Row, Table},
    },
    Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult,
};

use crate::{app_state::AppState, models::todo::Todo, utils, views::key_match};

pub struct TodoDisplayChunk;

#[derive(Default, State)]
pub struct CustomState {
    todos: Vec<Todo>,
}

pub fn event_handler(
    app_state: Res<AppState>,
    page_state: Res<super::PageState>,
    mut events: ResMut<Events>,
) -> WidgetResult {
    let Some(event) = &events.event else {
        return Ok(());
    };

    let binding = &app_state.key_config.key_binding;

    if let Event::Key(key) = event {
        if key_match(&key, binding.get("quit_program").unwrap()) {
            events.register_exit();
        }
    }

    return Ok(());
}
pub fn todo_updater(app_state: Res<AppState>, mut state: ResMut<CustomState>) -> WidgetResult {
    let pool = app_state.pool.clone();
    let todos = utils::run_async_fn(async move { Todo::get_all(&pool).await })?;
    state.todos = todos;
    return Ok(());
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    state: Res<CustomState>,
    page_state: Res<super::PageState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
    let chunk = chunks.get_chunk::<TodoDisplayChunk>()?;
    let header = utils::into_row(
        [
            "ID".to_string(),
            "Name".to_string(),
            "Created At".to_string(),
        ],
        Style::default().fg(Color::Yellow),
    );
    let todos = &state.todos;

    let rows = todos
        .iter()
        .map(|todo| todo.into_row())
        .collect::<Vec<Row>>();
    let tbl_widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ];

    let block = Block::new()
        .borders(Borders::ALL)
        .fg(if page_state.selected_block_idx == 0 {
            Color::Green
        } else {
            Color::White
        });
    let table = Table::new(rows, tbl_widths).block(block).header(header);

    frame.render_widget(table, chunk);

    Ok(())
}
