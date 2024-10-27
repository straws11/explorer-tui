use crate::{
    file_tree_widget::FileTreeWidget,
    tree::{FileTree, NavDirection},
    tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::StatefulWidget;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Widget},
    Terminal,
};
use std::io;
use tui::Tui;

#[derive(Default, Debug)]
pub struct App {
    pub exit: bool,
    pub tree: FileTree,
}

impl App {
    /// runs app's setup and main loop until user quits
    pub fn run(&mut self, terminal: Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
        let mut tui = Tui::new(terminal);
        tui.enter()?;
        self.tree = FileTree::new();
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
            KeyCode::Char('j') => self.tree.ft_move(NavDirection::Down),
            KeyCode::Char('k') => self.tree.ft_move(NavDirection::Up),
            KeyCode::Char('h') => self.tree.ft_move(NavDirection::OutOfDir),
            KeyCode::Char('l') => self.tree.ft_move(NavDirection::IntoDir),
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
        // splitting the app layout into different segments
        let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // creating my custom widget and call its render method
        let filetree_widget = FileTreeWidget::new(self.tree.linear_list.clone())
            .style(Style::default().fg(Color::Green));
        filetree_widget.render(chunks[0], buf, &mut self.tree.state);

        // placeholder stuff
        let block = Block::bordered();
        let title = Paragraph::new(Text::styled(
            "I'm trying",
            Style::default().fg(Color::Green),
        ))
        .block(block);
        title.render(chunks[1], buf);
    }
}
