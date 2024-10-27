use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{List, ListState, StatefulWidget},
};

use crate::tree::FileTree;

#[derive(Debug)]
pub struct FileTreeWidget<'a> {
    /// filetree object with sublists
    tree: FileTree,
    list: List<'a>,
    style: Style,
    highlight_style: Style,
}

impl FileTreeWidget<'_> {
    pub fn new(tree: FileTree) -> Self {
        Self {
            tree,
            list: List::default(),
            style: Style::default(),
            highlight_style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }
}

// TODO: is this & or &mut or as is??
impl StatefulWidget for FileTreeWidget<'_> {
    type State = FileTreeState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
