use std::path::Path;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};
use sysinfo::{Disk, Disks, System};

#[derive(Debug, Default)]
pub struct StatusBar {
    disk_usage: f64,
}

impl StatusBar {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        let disk = disks
            .into_iter()
            .filter(|disk| disk.mount_point().eq(Path::new("/")))
            .collect::<Vec<&Disk>>()[0];
        let total_filled = disk.total_space() - disk.available_space();
        StatusBar {
            disk_usage: (total_filled as f64 / disk.total_space() as f64) * 100.0,
        }
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let status_text = format!(
            "Usage: {:.2}% | Sys Name: {:?}",
            self.disk_usage,
            System::name().unwrap_or("Unknown".to_string())
        );
        let paragraph = Paragraph::new(Text::from(status_text)).block(Block::bordered());
        paragraph.render(area, buf);
    }
}
