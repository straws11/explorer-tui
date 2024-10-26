use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

// TODO: move this into the tui file?
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let block = Block::bordered();
    let title = Paragraph::new(Text::styled(
        "I'm trying",
        Style::default().fg(Color::Green),
    ))
    .block(block);
    frame.render_widget(title, chunks[0]);
}
