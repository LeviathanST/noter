use widgetui::{
    ratatui::{
        style::{Color, Stylize},
        widgets::{Block, Borders},
    },
    Chunks, Res, ResMut, WidgetFrame, WidgetResult,
};

use crate::app_state::{AppState, Page};

pub struct ContentDisplayChunk;

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    app_state: Res<AppState>,
    page_state: Res<super::PageState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
    if app_state.page != Page::Todo {
        return Ok(());
    }
    let chunk = chunks.get_chunk::<ContentDisplayChunk>()?;
    let block = Block::new().title("Content").borders(Borders::ALL).fg(
        if page_state.selected_block_idx == 1 {
            Color::Green
        } else {
            Color::White
        },
    );

    frame.render_widget(block, chunk);
    Ok(())
}
