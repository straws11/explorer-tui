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
                let _ = tree.get_files(&path, 2, None);
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

    pub fn get_files(
        &mut self,
        path: &Path,
        max_depth: usize,
        curr_file: Option<&Path>,
    ) -> io::Result<()> {
        let mut parent_indices: Vec<usize> = Vec::new();
        visit_dir(path, 0, max_depth, &mut parent_indices, curr_file, self)?;
        self.root_path = path.to_path_buf();
        self.state.parent_indices = parent_indices;
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
        let path_buf = self.root_path.clone().to_path_buf();
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
            NavDirection::OutOfDir => match path_buf.parent() {
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

        // fix / generate parent indices list
        let new_parents: Vec<usize> = match direction {
            NavDirection::IntoDir => {
                let offset = self.state.parent_indices[0];
                self.state
                    .parent_indices
                    .iter()
                    .skip(1)
                    .map(|idx| idx - offset - 1)
                    .collect::<Vec<usize>>()
            }
            _ => Vec::new(),
        };

        self.linear_list = Vec::new();
        self.state = FileTreeState::default();
        let _ = self.get_files(new_root_path, 2, Some(cur_selected_parent));

        if let NavDirection::IntoDir = direction {
            self.state.parent_indices = new_parents;
        }

        // find old selected one
        for (i, item) in self.linear_list.iter().enumerate() {
            if item.path == cur_selected_parent {
                self.state.list_state.select(Some(i));
            }
        }
    }
}

fn visit_dir(
    dir: &Path,
    depth: usize,
    max_depth: usize,
    idx_tracker: &mut Vec<usize>,
    path_to_search: Option<&Path>,
    tree: &mut FileTree,
) -> io::Result<()> {
    // depth reached, base case
    if depth == max_depth || !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // assign parent indices vec if reached, should only happen once
        error!("{:?}, {:?}", path_to_search, path);

        if let Some(searched) = path_to_search {
            if searched == path {
                error!("Found, assigning: {:?}", idx_tracker);
                // tree.state.parent_indices = idx_tracker.to_vec();
            }
        }

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
        tree.linear_list.push(file_obj);
        let old_count = tree.linear_list.len();

        // recursively visit subdirs
        if file_type == FileObjType::Directory {
            // put the parent idx on the tracker stack
            // idx_tracker.push(old_count - 1);
            // error!("calling with stack: {:?}", idx_tracker);
            let _ = visit_dir(
                &path,
                depth + 1,
                max_depth,
                idx_tracker,
                path_to_search,
                tree,
            );
            // idx_tracker.pop();
            // error!("stack after pop: {:?}", idx_tracker);
        };

        // update the fileobj's subsize value now that it's been computed
        // note this be incorrect (0) if max depth is hit on the above call
        tree.linear_list[old_count - 1].sub_items_size = tree.linear_list.len() - old_count;
    }
    Ok(())
}
