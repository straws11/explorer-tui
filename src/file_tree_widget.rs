use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

use crate::{
    file_tree_state::FileTreeState,
    tree::{FileObj, FileObjType, FileTree},
};

#[derive(Debug)]
pub struct FileTreeWidget<'a> {
    /// filetree object with sublists
    tree: FileTree,
    /// Ratatui list component for rendering the output list
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

    /// Helper method to generate the List (of ListItems) for Tree
    fn generate_list_items(&self) -> Vec<ListItem> {
        let mut flat_list: Vec<FileObj> = Vec::new();
        let mut item_list: Vec<ListItem> = Vec::new();
        // create flat list
        build_list(&self.tree.root, 0, &mut flat_list);

        for item in &flat_list {
            let disp_str = " ".repeat(item.depth * 3);
            let disp_str = match item.object_type {
                FileObjType::File => format!("{}{}", disp_str, item.name.clone()),
                FileObjType::Directory => format!("{}{}/", disp_str, item.name.clone()),
            };
            item_list.push(ListItem::new(disp_str).style(Style::default().fg(Color::White)));
        }
        item_list
    }
}

/// Create a flat list (1D vec) of FileObj
fn build_list(obj: &FileObj, depth: u8, list: &mut Vec<FileObj>) {
    for item in &obj.sub_items {
        // add item to list
        // let rep = " ".repeat((depth * 3).into());
        // let name = match item.object_type {
        //     FileObjType::File => format!("{}{}", rep, item.name.clone()),
        //     FileObjType::Directory => format!("{}{}/", rep, item.name.clone()),
        // };
        // let li = ListItem::new(name).style(Style::default().fg(Color::White));
        // list.push(name);
        // recursive call for dirs
        list.push(item.clone());
        match item.object_type {
            FileObjType::Directory => build_list(item, depth + 1, list),
            FileObjType::File => {}
        }
    }
}
// TODO: is this & or &mut or as is??
impl StatefulWidget for FileTreeWidget<'_> {
    type State = FileTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // generate ListItems
        let list_items_formatted = self.generate_list_items();

        let list = List::new(list_items_formatted).highlight_style(self.highlight_style);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
