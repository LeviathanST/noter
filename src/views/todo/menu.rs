use widgetui::{
    ratatui::{
        style::{Color, Stylize},
        widgets::{Block, Borders},
    },
    Chunks, Res, ResMut, WidgetFrame, WidgetResult,
};

pub struct MenuChunk;

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    page_state: Res<super::PageState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
    let chunk = chunks.get_chunk::<MenuChunk>()?;
    let block = Block::new().title("Menu").borders(Borders::ALL).fg(
        if page_state.selected_block_idx == 2 {
            Color::Green
        } else {
            Color::White
        },
    );

    frame.render_widget(block, chunk);
    Ok(())
}
