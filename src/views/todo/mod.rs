mod content_display;
mod display_all;
mod menu;

use std::usize;

use content_display::ContentDisplayChunk;
use display_all::TodoDisplayChunk;
use menu::MenuChunk;
use widgetui::{
    constraint,
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout,
    ratatui::layout::{Constraint, Direction, Layout},
    set, App, Chunks, Events, Res, ResMut, Set, State, WidgetFrame, WidgetResult,
};

use crate::app_state::{self, AppState};

use super::{key_match, NoterKeyEvent};

#[derive(State, Default)]
pub struct PageState {
    pub selected_block_idx: usize,
}

impl PageState {
    pub fn next_block(&mut self) {
        if (self.selected_block_idx + 1) > 2 {
            self.selected_block_idx = 0;
        } else {
            self.selected_block_idx += 1;
        };
    }
    pub fn prev_block(&mut self) {
        if (self.selected_block_idx - 1) < 0 {
            self.selected_block_idx = 2;
        } else {
            self.selected_block_idx -= 1;
        };
    }
}

fn chunk_generator(frame: Res<WidgetFrame>, mut chunks: ResMut<Chunks>) -> WidgetResult {
    let chunk = layout! {
        frame.size(),
        (#1),
        (%100) => {#1, %70, %30, #1},
        (#1)
    };
    let sub_chunk = layout! {
        chunk[1][2],
        (%70),
        (%30)
    };

    chunks.register_chunk::<TodoDisplayChunk>(chunk[1][1]);
    chunks.register_chunk::<ContentDisplayChunk>(sub_chunk[0][0]);
    chunks.register_chunk::<MenuChunk>(sub_chunk[1][0]);
    Ok(())
}
fn event_handler(
    mut page_state: ResMut<PageState>,
    app_state: Res<AppState>,
    events: Res<Events>,
) -> WidgetResult {
    let Some(event) = &events.event else {
        return Ok(());
    };
    let binding = &app_state.key_config.key_binding;

    if let Event::Key(key) = event {}

    Ok(())
}

#[set]
pub fn TodoSet(app: App) -> App {
    app.states((PageState::default(), display_all::CustomState::default()))
        .widgets((
            chunk_generator,
            display_all::todo_updater,
            display_all::event_handler,
            display_all::render,
            content_display::render,
            menu::render,
        ))
}
