mod app;
mod file_tree_state;
mod file_tree_widget;
mod tree;
mod tui;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut app = App::default();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    app.run(terminal)
}
