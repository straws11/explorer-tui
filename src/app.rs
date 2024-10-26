use crate::{
    file_tree::Tree,
    tui::{self, CrosstermTerminal},
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    style::Style,
    widgets::{ListState, StatefulWidget},
    Terminal,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use tui::Tui;

#[derive(Default, Debug)]
pub struct App {
    pub exit: bool,
    pub file_tree: Tree,
}

impl App {
    /// runs app's setup and main loop until user quits
    pub fn run(&mut self, terminal: Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
        let mut tui = Tui::new(terminal);
        tui.enter()?;
        self.file_tree = Tree::new();
        // main loop
        while !self.exit {
            // receives ref to app for its state data
            tui.draw(self)?;
            self.handle_events()?;
        }
        tui.exit()?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // checking key **press** because crossterm emits key release and repeat too (on W*ndows)
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // KeyCode::Left => self.dec_counter(),
            // KeyCode::Right => self.inc_counter(),
            _ => {}
        }
    }
    // todo some test stuff from the example page, might be cool to look at
    // https://ratatui.rs/tutorials/counter-app/basic-app/

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let block = Block::bordered();
        let title = Paragraph::new(Text::styled(
            "I'm trying",
            Style::default().fg(Color::Green),
        ))
        .block(block);
        self.file_tree.render(chunks[0], buf);
        // frame.render_widget(title, chunks[0]);
        // frame.render_widget(&app.file_tree, chunks[1]);
    }
}
