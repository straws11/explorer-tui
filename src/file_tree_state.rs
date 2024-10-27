use ratatui::widgets::ListState;

#[derive(Default, Debug)]
pub struct FileTreeState {
    /// Internally managed underlying Ratatui List state
    list_state: ListState,
    /// Depth into sub dir
    depth: usize,
}

impl FileTreeState {
    pub fn move_down(&mut self) {
        // self.state.list_state.select(num)
    }

    pub fn move_up(&mut self) {}

    /// Move into a subdir, pushing the parent idx onto stack
    pub fn move_sub_dir(&mut self) {
        let parent_idx = self.state.list_state.selected().unwrap();
        self.state.parent_index.push(parent_idx);
        self.state.list_state.select_next();
    }

    /// Move up to the parent dir's index, popped from a stack
    pub fn move_parent_dir(&mut self) {
        let parent_idx = self.state.parent_index.pop();
        if parent_idx.is_some() {
            self.state.list_state.select(parent_idx);
        }
    }
}
