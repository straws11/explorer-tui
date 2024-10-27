use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::style::{Color, Style};
use ratatui::widgets::ListItem;
use ratatui::widgets::{Block, List, ListState, StatefulWidget, Widget};

use crate::file_tree_widget::FileTreeState;

#[derive(Default, Debug, Clone)]
pub enum FileObjType {
    #[default]
    File,
    Directory,
}

#[derive(Default, Debug, Clone)]
pub struct FileObj {
    pub sub_items: Vec<FileObj>,
    pub sub_items_size: usize,
    pub object_type: FileObjType,
    pub name: String,
    pub depth: usize,
}

impl FileObj {
    pub fn new(obj_type: FileObjType, name: String, depth: usize) -> Self {
        Self {
            sub_items: Vec::new(),
            sub_items_size: 0,
            object_type: obj_type,
            name,
            depth,
        }
    }
}

/// Struct resembling a directory structure, with user state
#[derive(Default, Debug)]
pub struct FileTree {
    pub root: FileObj,
    pub state: FileTreeState,
}

impl FileTree {
    pub fn new() -> Self {
        // TODO: smarter way to get the starting path, env something
        let mut tree = Self {
            root: FileObj::new(FileObjType::Directory, "root, todo".to_string(), 0),
            // root_path: Path::new("../../explorer_rust"),
            state: FileTreeState::default(),
        };

        // keep the list of objects for the component use
        let root_path = Path::new("../../explorer_rust");
        tree.get_files(root_path, 2);
        tree
    }

    pub fn get_files(&mut self, path: &Path, max_depth: usize) -> io::Result<()> {
        visit_dir(path, 0, max_depth, &mut self.root)?;
        Ok(())
    }
}

/// Helper method to recursively generate the file tree
fn visit_dir(dir: &Path, depth: usize, max_depth: usize, node: &mut FileObj) -> io::Result<()> {
    // depth reached, base case
    if depth == max_depth || !dir.is_dir() {
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
            FileObj::new(FileObjType::File, item_name, depth)
        } else {
            let mut dir_obj = FileObj::new(FileObjType::Directory, item_name, depth);
            visit_dir(&path, depth + 1, max_depth, &mut dir_obj)?;
            dir_obj
        };

        node.sub_items.push(file_obj);
    }
    node.sub_items_size = node.sub_items.len();
    Ok(())
}

/// Helper method to generate the List (of ListItems) for Tree
pub fn build_list(obj: &FileObj, depth: u8, list: &mut Vec<ListItem>) {
    for item in &obj.sub_items {
        // add item to list
        let rep = " ".repeat((depth * 3).into());
        let name = match item.object_type {
            FileObjType::File => format!("{}{}", rep, item.name.clone()),
            FileObjType::Directory => format!("{}{}/", rep, item.name.clone()),
        };
        let li = ListItem::new(name).style(Style::default().fg(Color::White));
        list.push(li);

        // recursive call for dirs
        match item.object_type {
            FileObjType::Directory => build_list(item, depth + 1, list),
            FileObjType::File => {}
        }
    }
}

impl fmt::Display for FileObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Widget for &mut FileTree {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // building the List component data
        let mut list_items: Vec<ListItem> = Vec::new();
        build_list(&self.root, 0, &mut list_items);
    }
}
