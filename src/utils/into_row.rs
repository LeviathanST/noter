use ratatui::{style::Style, text::Text, widgets::Row};

pub fn into_row<'a, const N: usize>(data: [&str; N], style: Style) -> Row<'a> {
    Row::new(data.map(|content| Text::from(content.to_string()).style(style)))
}
