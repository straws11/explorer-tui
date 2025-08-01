use crate::{
    file_tree_widget::FileTreeWidget,
    preview_pane_widget::PreviewPane,
    status_bar_widget::StatusBar,
    tree::{FileObj, FileObjType, FileTree, NavDirection},
    tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use log::error;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Widget},
    Terminal,
};
use ratatui::{prelude::StatefulWidget, text::Text};
use std::{fs, io};
use tui::Tui;

#[derive(Default, Debug)]
pub struct App {
    pub exit: bool,
    pub tree: FileTree,
    pub preview_pane: PreviewPane,
    pub app_action: AppAction,
    pub notify: String,
}

#[derive(Debug, Default)]
pub enum AppAction {
    Copying(FileObj),
    Moving(FileObj),
    #[default]
    None,
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
            KeyCode::Char('L') => self.tree.ft_move(NavDirection::ZoomIn),
            KeyCode::Char(' ') => self
                .tree
                .try_toggle_collapse()
                .expect("File Error TODO move lol"),
            KeyCode::Char('y') => {
                self.app_action = AppAction::Copying(self.tree.get_selected_item().clone())
            }
            KeyCode::Char('P') => self.paste_file(true),
            KeyCode::Char('p') => self.paste_file(false),
            KeyCode::Char('x') => {
                self.app_action = AppAction::Moving(self.tree.get_selected_item().clone())
            }
            _ => {}
        }
    }
    // todo some test stuff from the example page, might be cool to look at
    // https://ratatui.rs/tutorials/counter-app/basic-app/

    fn paste_file(&mut self, inside: bool) {
        if let AppAction::None = self.app_action {
            return;
        }

        // setup destination
        let file = match &self.app_action {
            AppAction::Copying(file) | AppAction::Moving(file) => file,
            AppAction::None => {
                // impossible to reach anyway
                return;
            }
        };

        let item = self.tree.get_selected_item();
        let mut dest = if inside && matches!(item.object_type, FileObjType::Directory(_)) {
            // try to paste inside the directory currently selected
            item.path.to_path_buf()
        } else {
            // else: either not a dir, or we don't want to paste inside, paste here
            item.path.parent().unwrap().to_path_buf()
        };
        dest.push(&file.name);

        // final action
        match &self.app_action {
            AppAction::Copying(file) => {
                let copy_result = fs::copy(file.path.clone(), &dest);
                match copy_result {
                    Ok(_) => {}
                    Err(e) => {
                        error!("{}", e)
                    }
                }
                self.notify = format!("Pasted {:?} to {:?}", file.path, dest);
            }
            AppAction::Moving(file) => {
                let move_result = fs::rename(file.path.clone(), &dest);
                match move_result {
                    Ok(_) => {}
                    Err(e) => {
                        error!("{}", e)
                    }
                }
                self.notify = format!("Moved {:?} to {:?}", file.path, dest);
            }
            AppAction::None => {}
        }
        self.app_action = AppAction::None;
    }

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
                    // error!("Text generated: {}", text);
                    self.preview_pane.is_available = true;
                    str::replace(&text, "\t", "    ")
                }
                Err(_) => {
                    // assuming it's just a dir, we will skip it
                    // error!("Text not generated!");
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
        let main_chunks =
            // Layout::vertical([Constraint::Min(2), Constraint::Percentage(100)]).split(area);
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1), Constraint::Length(1)]).split(area);

        let status = StatusBar::new();
        status.render(main_chunks[0], buf);
        self.notify = match &self.app_action {
            AppAction::Copying(file) => format!("Copying {:?}", file.path),
            AppAction::Moving(file) => format!("Moving {:?}", file.path),
            AppAction::None => "".to_string(),
        };
        let notify_text = Text::from(self.notify.clone());
        notify_text.render(main_chunks[2], buf);

        let content_chunks =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);
        // creating my custom widget and call its render method
        let filetree_widget = FileTreeWidget::new(self.tree.linear_list.clone())
            .style(Style::default().fg(Color::Green))
            .block(Block::bordered().title(format!("{}", self.tree.root_path.clone().display(),)));
        filetree_widget.render(content_chunks[0], buf, &mut self.tree.state);

        self.set_preview_contents();
        self.preview_pane.render(content_chunks[1], buf);
    }
}
