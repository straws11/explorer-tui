use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Styled, Stylize},
    widgets::{Block, List, ListItem, StatefulWidget},
};

use crate::{
    file_tree_state::FileTreeState,
    tree::{DirectoryStatus, FileObj, FileObjType},
};

#[derive(Debug, Clone)]
pub struct FileTreeWidget<'a> {
    /// filetree object with sublists
    file_list: Vec<FileObj>,
    /// Ratatui list component for rendering the output list
    // list: List<'a>,
    style: Style,
    highlight_style: Style,
    block: Block<'a>,
}

impl<'a> FileTreeWidget<'a> {
    pub fn new(obj_list: Vec<FileObj>) -> Self {
        Self {
            file_list: obj_list,
            // list: List::default(),
            style: Style::default(),
            highlight_style: Style::default(),
            block: Block::default(),
        }
    }

    /// Mimic ratatui component styling
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Mimic ratatui component styling for highlight
    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }

    /// Mimic ratatui component styling with block
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }

    /// Helper method to generate the List (of ListItems) for Tree
    fn generate_list_items(&self, selected_idx: Option<usize>) -> Vec<ListItem> {
        let mut item_list: Vec<ListItem> = Vec::new();

        // map each FileObj to a ListItem
        for (pos, item) in self.file_list.iter().enumerate() {
            // TODO: fix the way the disp_str is calculated
            let disp_str = match selected_idx {
                Some(idx) => {
                    if idx == pos {
                        format!("{}> ", " ".repeat(item.depth * 3))
                    } else {
                        format!("{}  ", " ".repeat(item.depth * 3))
                    }
                }
                None => format!("{}  ", " ".repeat(item.depth * 3)),
            };
            let disp_str = match item.object_type {
                FileObjType::File => format!("{} {}", disp_str, item.name.clone()),
                FileObjType::Directory(DirectoryStatus::Collapsed) => {
                    format!("{} {}/", disp_str, item.name.clone())
                }
                FileObjType::Directory(DirectoryStatus::Open) => {
                    format!("{} {}/", disp_str, item.name.clone())
                }
            };
            item_list.push(ListItem::new(disp_str).style(Style::default().fg(Color::White)));
        }
        item_list
    }
}

// TODO: is this & or &mut or as is??
impl StatefulWidget for FileTreeWidget<'_> {
    type State = FileTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // generate ListItems
        // TODO: fix this clone nonsense, i think generate_list_items needs changing
        let clon = self.clone();
        let list_items_formatted = clon.generate_list_items(state.list_state.selected());

        let list = List::new(list_items_formatted)
            .style(self.style)
            .highlight_style(Style::default().fg(Color::Red))
            .block(self.block)
            .scroll_padding(3);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
