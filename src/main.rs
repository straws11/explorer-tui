mod app;
mod tui;
mod ui;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;
use crate::tui::Tui;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

#[derive(Default, Debug)]
pub enum FileObjType {
    #[default]
    File,
    Directory,
}

#[derive(Default, Debug)]
pub struct FileObj {
    pub sub_items: Vec<FileObj>,
    pub object_type: FileObjType,
    pub name: String,
}

impl FileObj {
    pub fn new(obj_type: FileObjType, name: String) -> Self {
        Self {
            sub_items: Vec::new(),
            object_type: obj_type,
            name,
        }
    }
}

#[derive(Default, Debug)]
pub struct Tree {
    pub root_items: Vec<FileObj>,
}

impl Tree {
    pub fn get_files(&mut self, depth: u8) -> io::Result<()> {
        let path = Path::new("../../explorer_rust");
        Tree::visit_dir(path, depth, &mut self.root_items)?;
        Ok(())
    }

    /// Recursively generate the file tree
    fn visit_dir(dir: &Path, depth: u8, list: &mut Vec<FileObj>) -> io::Result<()> {
        // depth reached, base case
        if depth == 0 || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let item_name = entry.file_name().into_string();
            let item_name = match item_name {
                Ok(name) => name,
                Err(e) => {
                    println!("Error converting filename: {:?}", e);
                    // lol this can't be right vv
                    return Ok(());
                }
            };

            let mut file_obj;
            if !path.is_dir() {
                file_obj = FileObj::new(FileObjType::File, item_name);
            } else {
                file_obj = FileObj::new(FileObjType::Directory, item_name);
                Self::visit_dir(&path, depth - 1, &mut file_obj.sub_items)?;
            }

            list.push(file_obj);
        }
        Ok(())
    }
}
fn main() -> io::Result<()> {
    let mut app = App::default();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    app.run(terminal)
}
