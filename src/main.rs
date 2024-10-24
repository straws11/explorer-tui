mod app;
use crate::app::App;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

#[derive(Default, Debug)]
pub struct Tree {
    pub files: Vec<String>,
}

impl Tree {
    pub fn get_files(&mut self) -> io::Result<()> {
        let path = Path::new("../../explorer_rust");
        Tree::visit_dir(path, 2)?;
        Ok(())
    }
    fn visit_dir(dir: &Path, depth: u8) -> io::Result<()> {
        // depth reached
        if depth == 0 {
            return Ok(());
        }
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Self::visit_dir(&path, depth - 1)?;
                } else {
                    println!("{:?}", &entry);
                }
            }
        }
        Ok(())
    }
}
fn main() -> io::Result<()> {
    let mut tree = Tree::default();
    tree.get_files()?;
    println!("{:?}", tree);
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
