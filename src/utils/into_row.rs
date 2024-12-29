use widgetui::ratatui::{style::Style, text::Text, widgets::Row};

pub fn into_row<'a, const N: usize>(data: [String; N], style: Style) -> Row<'a> {
    Row::new(data.map(|content| Text::from(content).style(style)))
}
