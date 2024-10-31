mod app;
mod file_tree_state;
mod file_tree_widget;
mod preview_pane_widget;
mod tree;
mod tui;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let target = Box::new(File::create("./log.txt").expect("Can't create file"));
    env_logger::builder()
        .target(env_logger::Target::Pipe(target))
        .init();
    let mut app = App::default();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    app.run(terminal)
}
