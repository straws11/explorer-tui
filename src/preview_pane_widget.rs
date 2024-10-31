use log::error;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
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
            let a = Text::from(self.preview_contents.clone());
            error!("Text item for contents: {}", a.to_string());
            a
        } else {
            Text::from(Line::from("Preview Unavailable").style(Style::default().italic()))
        };
        let para = Paragraph::new(text).block(Block::bordered().title("File Preview"));
        error!("Paragraph text: {:?}", para);
        para.render(area, buf);
    }
}
