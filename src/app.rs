use crate::{
    file_tree_widget::FileTreeWidget,
    preview_pane_widget::PreviewPane,
    tree::{FileTree, NavDirection},
    tui,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{Clear, ClearType},
};
use log::error;
use ratatui::prelude::StatefulWidget;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Widget},
    Terminal,
};
use std::{fs, io};
use tui::Tui;

#[derive(Default, Debug)]
pub struct App {
    pub exit: bool,
    pub tree: FileTree,
    pub preview_pane: PreviewPane,
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

    fn set_preview_contents(&mut self) {
        if self.tree.state.index_changed() {
            // generate new contents
            let current_item = self.tree.get_selected_item();
            self.preview_pane.preview_contents = match fs::read_to_string(current_item.path.clone())
            {
                Ok(text) => {
                    error!("Text generated: {}", text);
                    self.preview_pane.is_available = true;
                    text
                }
                Err(_) => {
                    // assuming it's just a dir, we will skip it
                    error!("Text not generated!");
                    self.preview_pane.is_available = false;
                    "".to_string()
                }
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // splitting the app layout into different segments
        let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // creating my custom widget and call its render method
        let filetree_widget = FileTreeWidget::new(self.tree.linear_list.clone())
            .style(Style::default().fg(Color::Green))
            .block(Block::bordered().title("File Tree"));
        filetree_widget.render(chunks[0], buf, &mut self.tree.state);

        self.set_preview_contents();
        Clear(ClearType::All);
        self.preview_pane.render(chunks[1], buf);
    }
}
