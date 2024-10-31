use ratatui::widgets::ListState;
use std::cmp::Ordering;

use crate::tree::{FileObj, FileObjType};

#[derive(Default, Debug)]
pub struct FileTreeState {
    /// Internally managed underlying Ratatui List state
    pub list_state: ListState,
    /// Depth into sub dir
    pub parent_indices: Vec<usize>,
    /// Prev file index
    pub prev_idx: usize,
}

impl FileTreeState {
    pub fn default() -> Self {
        let mut fts = Self {
            list_state: ListState::default(),
            parent_indices: Vec::new(),
            prev_idx: 0,
        };
        fts.list_state.select_first();
        fts
    }

    pub fn move_down(&mut self, list: &Vec<FileObj>) {
        let idx = self.list_state.selected().unwrap();

        let mut count = idx + 1;
        while count < list.len() {
            match list[idx].depth.cmp(&list[count].depth) {
                Ordering::Equal => {
                    // next file is in the same dir
                    self.list_state.select(Some(count));
                    break;
                }
                // bottom of this sub list
                Ordering::Greater => break,

                // skipping over a subdir's entries
                Ordering::Less => count += 1,
            }
        }
    }

    pub fn move_up(&mut self, list: &Vec<FileObj>) {
        let idx = self.list_state.selected().unwrap();
        if idx == 0 {
            return;
        }
        let mut count = idx;
        while count > 0 {
            count -= 1;
            match list[idx].depth.cmp(&list[count].depth) {
                Ordering::Equal => {
                    self.list_state.select(Some(count));
                    return;
                }
                // top of this subdir
                // NOTE: make this select the parent? (by popping)
                Ordering::Greater => break,
                Ordering::Less => {}
            }
        }
    }

    pub fn move_sub_dir(&mut self, list: &Vec<FileObj>) {
        let idx = self.list_state.selected().unwrap();
        // file or empty dir => skip
        if list[idx].object_type == FileObjType::File || list[idx].sub_items_size == 0 {
            return;
        }
        self.list_state.select_next();
        self.parent_indices.push(idx);
    }

    pub fn move_parent_dir(&mut self, list: &Vec<FileObj>) {
        let idx = self.list_state.selected().unwrap();
        let parent_idx = self.parent_indices.pop();

        if let Some(idx) = parent_idx {
            self.list_state.select(Some(idx));
        }
    }

    /// Returns whether the file tree selected item has changed since the last call to this
    /// function
    pub fn index_changed(&mut self) -> bool {
        let idx = self.list_state.selected().expect("File not selected");

        if idx != self.prev_idx {
            self.prev_idx = idx;
            return true;
        } else {
            return false;
        }
    }
}
