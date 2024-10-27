use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, ListState, StatefulWidget, Widget};

use crate::file_tree_state::FileTreeState;

/// File tree navigational directions
#[derive(Default, Debug)]
pub enum NavDirection {
    #[default]
    Up,
    Down,
    IntoDir,
    OutOfDir,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FileObjType {
    #[default]
    File,
    Directory,
}

#[derive(Default, Debug, Clone)]
pub struct FileObj {
    // pub sub_items: Vec<FileObj>,
    pub sub_items_size: usize,
    pub object_type: FileObjType,
    pub name: String,
    pub depth: usize,
}

impl FileObj {
    pub fn new(obj_type: FileObjType, name: String, depth: usize) -> Self {
        Self {
            // sub_items: Vec::new(),
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
    // pub root: FileObj,
    pub state: FileTreeState,
    pub linear_list: Vec<FileObj>,
}

impl FileTree {
    pub fn new() -> Self {
        // TODO: smarter way to get the starting path, env something
        let mut tree = Self {
            // root: FileObj::new(FileObjType::Directory, "root, todo".to_string(), 0),
            // root_path: Path::new("../../explorer_rust"),
            state: FileTreeState::default(),
            linear_list: Vec::new(),
        };

        // keep the list of objects for the component use
        let root_path = Path::new("../../explorer_rust");
        tree.get_files(root_path, 2);
        tree
    }

    pub fn get_files(&mut self, path: &Path, max_depth: usize) -> io::Result<()> {
        visit_dir2(path, 0, max_depth, &mut self.linear_list)?;
        Ok(())
    }

    pub fn ft_move(&mut self, direction: NavDirection) {
        match direction {
            NavDirection::Up => self.state.move_up(&self.linear_list),
            NavDirection::Down => self.state.move_down(&self.linear_list),
            NavDirection::IntoDir => self.state.move_sub_dir(&self.linear_list),
            NavDirection::OutOfDir => self.state.move_parent_dir(&self.linear_list),
        }
    }
}

fn visit_dir2(
    dir: &Path,
    depth: usize,
    max_depth: usize,
    list: &mut Vec<FileObj>,
) -> io::Result<()> {
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

        let file_type = if path.is_dir() {
            FileObjType::Directory
        } else {
            FileObjType::File
        };

        let file_obj = FileObj::new(file_type.clone(), item_name, depth);
        list.push(file_obj);
        let old_count = list.len();

        // recursively visit subdirs
        if file_type == FileObjType::Directory {
            visit_dir2(&path, depth + 1, max_depth, list);
        };

        // update the fileobj's subsize value now that it's been computed
        list[old_count - 1].sub_items_size = list.len() - old_count;
    }
    Ok(())
}
