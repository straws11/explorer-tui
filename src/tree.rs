use log::error;

use crate::file_tree_state::FileTreeState;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

/// An action to take after a tree operation
#[derive(Default, Debug)]
pub enum TreeAction {
    #[default]
    None,
    GenerateParent,
    GenerateChild,
}
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
    pub root_path: PathBuf,
}

impl FileTree {
    pub fn new() -> Self {
        let mut tree = Self {
            state: FileTreeState::default(),
            linear_list: Vec::new(),
            root_path: PathBuf::new(),
        };

        // keep the list of objects for the component use
        // let root_path = Path::new("../../explorer_rust");
        let root_path = env::current_dir();
        match root_path {
            Ok(path) => {
                let _ = tree.get_files(&path, 3);
            }
            Err(e) => println!("Current Dir error: {}", e),
        }
        tree
    }

    /// Return reference to the FileObj at the currently selected index
    pub fn get_selected_item(&mut self) -> &FileObj {
        let idx = self.state.list_state.selected().expect("No file selected");
        &self.linear_list[idx]
    }

    pub fn get_files(&mut self, path: &Path, max_depth: usize) -> io::Result<()> {
        visit_dir(path, 0, max_depth, &mut self.linear_list)?;
        self.root_path = path.to_path_buf();
        Ok(())
    }

    pub fn ft_move(&mut self, direction: NavDirection) {
        match direction {
            NavDirection::Up => {
                let action = self.state.move_up(&self.linear_list);
                self.handle_action(action);
            }
            NavDirection::Down => {
                let action = self.state.move_down(&self.linear_list);
                self.handle_action(action);
            }
            NavDirection::IntoDir => {
                let action = self.state.move_sub_dir(&self.linear_list);
                self.handle_action(action);
            }
            NavDirection::OutOfDir => {
                let action = self.state.move_parent_dir(&self.linear_list);
                self.handle_action(action);
            }
        }
    }

    fn handle_action(&mut self, action: TreeAction) {
        match action {
            TreeAction::GenerateParent => self.regen_tree(NavDirection::OutOfDir),
            TreeAction::GenerateChild => self.regen_tree(NavDirection::IntoDir),
            TreeAction::None => {}
        }
    }

    /// Regenerates the list used for the filetree
    fn regen_tree(&mut self, direction: NavDirection) {
        let old_root = self.root_path.clone();
        // path of selected file before regen
        let cur_selected_path = self.get_selected_item().path.clone();
        let cur_selected_parent = match cur_selected_path.parent() {
            Some(path) => path,
            None => return,
        };

        let new_root_path: &Path = match direction {
            NavDirection::IntoDir => {
                // &cur_selected_path
                let next_root_idx = self.state.parent_indices[0];
                &self.linear_list[next_root_idx].path.clone()
            }
            NavDirection::OutOfDir => match old_root.parent() {
                Some(valid_path) => {
                    if valid_path.to_str() != Some("") {
                        valid_path
                    } else {
                        return;
                    }
                }
                None => return,
            },
            // this really should not ever happen lol
            _ => return,
        };

        // expensive, sad
        let mut old_parents_path: Vec<PathBuf> = self
            .state
            .parent_indices
            .iter()
            .skip(1)
            .map(|idx| self.linear_list[*idx].path.clone())
            .collect();

        // this will be the newest parent, forcing it to be added
        old_parents_path.push(cur_selected_path.clone());
        self.linear_list = Vec::new();
        self.state = FileTreeState::default();
        let _ = self.get_files(new_root_path, 3);

        let mut new_parents: Vec<usize> = Vec::new();
        if let NavDirection::IntoDir = direction {
            for (i, item) in self.linear_list.iter().enumerate() {
                if item.path == old_parents_path[0] {
                    new_parents.push(i);
                    old_parents_path.remove(0);
                    // have found all parents
                    if old_parents_path.is_empty() {
                        break;
                    }
                }
            }
        }
        self.state.parent_indices = new_parents;

        // set pointer state appropriately
        for (i, item) in self.linear_list.iter().enumerate() {
            match direction {
                NavDirection::IntoDir => {
                    if item.path == cur_selected_path {
                        // don't go into empty dirs
                        let idx = if item.sub_items_size > 0 { i + 1 } else { i };
                        self.state.list_state.select(Some(idx));
                        return;
                    }
                }
                NavDirection::OutOfDir => {
                    if item.path == cur_selected_parent {
                        self.state.list_state.select(Some(i));
                        return;
                    }
                }
                _ => {}
            }
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

        // check if this is the subdir we're coming out of -> set the struct val
        // nice short circuiting below (i believe) dont have to eval whole string if depth is wrong
        // if depth == 0 && path == parent_to_find {
        //     self.s
        // }

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
        // note this be incorrect (0) if max depth is hit on the above call
        list[old_count - 1].sub_items_size = list.len() - old_count;
    }
    Ok(())
}
