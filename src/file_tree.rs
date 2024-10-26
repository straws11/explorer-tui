use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, List, ListState, StatefulWidget, Widget};
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
    pub ft_state: ListState,
}

impl Tree {
    pub fn new() -> Self {
        let mut tree = Self {
            root_items: Vec::new(),
            ft_state: ListState::default(),
        };
        let _ = tree.get_files(3);
        tree
    }
    pub fn get_files(&mut self, depth: u8) -> io::Result<()> {
        let path = Path::new("../../explorer_rust");
        visit_dir(path, depth, &mut self.root_items)?;
        Ok(())
    }
}

/// Helper method to recursively generate the file tree
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
                continue; // skip invalid filename entries
            }
        };

        let file_obj = if !path.is_dir() {
            FileObj::new(FileObjType::File, item_name)
        } else {
            let mut dir_obj = FileObj::new(FileObjType::Directory, item_name);
            visit_dir(&path, depth - 1, &mut dir_obj.sub_items)?;
            dir_obj
        };

        list.push(file_obj);
    }
    Ok(())
}

impl fmt::Display for FileObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Widget for &mut Tree {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.root_items.iter().map(|obj| obj.name.clone());
        let list = List::new(items).block(Block::bordered().title("FT"));
        StatefulWidget::render(list, area, buf, &mut self.ft_state);
    }
}
