use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use ratatui_explorer::{FileExplorer, Theme};

use crate::app::App;

// TODO: move this into the tui file?
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    // let theme = Theme::default().add_default_title();
    // let mut file_explorer = FileExplorer::with_theme(theme).expect("FileExplorer failed");
    let block = Block::bordered();
    let title = Paragraph::new(Text::styled(
        "I'm trying",
        Style::default().fg(Color::Green),
    ))
    .block(block);
    frame.render_widget(title, chunks[0]);
    // frame.render_widget(&file_explorer.widget(), chunks[1]);
}
