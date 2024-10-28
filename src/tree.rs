use crate::file_tree_state::FileTreeState;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

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
    pub sub_items_size: usize,
    pub object_type: FileObjType,
    pub name: String,
    pub depth: usize,
    pub path: PathBuf,
}

impl FileObj {
    pub fn new(obj_type: FileObjType, name: String, depth: usize, path: PathBuf) -> Self {
        Self {
            sub_items_size: 0,
            object_type: obj_type,
            name,
            depth,
            path,
        }
    }
}

/// Struct resembling a directory structure, with user state
#[derive(Default, Debug)]
pub struct FileTree {
    pub state: FileTreeState,
    pub linear_list: Vec<FileObj>,
}

impl FileTree {
    pub fn new() -> Self {
        let mut tree = Self {
            state: FileTreeState::default(),
            linear_list: Vec::new(),
        };

        // keep the list of objects for the component use
        // let root_path = Path::new("../../explorer_rust");
        let root_path = env::current_dir();
        match root_path {
            Ok(path) => {
                let _ = tree.get_files(&path, 2);
            }
            Err(e) => println!("Current Dir error: {}", e),
        }
        tree
    }

    pub fn get_selected_item(&mut self) -> Option<FileObj> {
        let idx = self.state.list_state.selected();
        // TODO: i don't think this method should be cloning the object, I only need to view it..
        match idx {
            Some(idx) => Some(self.linear_list[idx].clone()),
            None => None,
        }
    }

    pub fn get_files(&mut self, path: &Path, max_depth: usize) -> io::Result<()> {
        visit_dir(path, 0, max_depth, &mut self.linear_list)?;
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

fn visit_dir(
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

        let file_obj = FileObj::new(file_type.clone(), item_name, depth, entry.path());
        list.push(file_obj);
        let old_count = list.len();

        // recursively visit subdirs
        if file_type == FileObjType::Directory {
            let _ = visit_dir(&path, depth + 1, max_depth, list);
        };

        // update the fileobj's subsize value now that it's been computed
        list[old_count - 1].sub_items_size = list.len() - old_count;
    }
    Ok(())
}
