use log::error;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct PreviewPane {
    pub preview_contents: String,
    pub is_available: bool,
}

impl PreviewPane {}

impl Widget for &PreviewPane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.is_available {
            // TODO: expensive clone rip
            Text::from(self.preview_contents.clone())
        } else {
            Text::from(Line::from("Preview Unavailable").style(Style::default().italic()))
        };
        let chunks =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

        let para = Paragraph::new(text).block(Block::bordered().title("File Preview"));
        // error!("Paragraph text: {:?}", para);
        para.render(chunks[0], buf);
    }
}
