use std::collections::HashSet;
use ratatui::widgets::ListItem;

pub trait CheckBoxListExt {
    fn to_checkbox_items(
        self,
        selected: &HashSet<usize>,
    ) -> Vec<ListItem<'static>>;
}

impl<I, T> CheckBoxListExt for I
where
    I: Iterator<Item = (usize, T)>,
    T: ToString
{
    fn to_checkbox_items(self, selected: &HashSet<usize>) -> Vec<ListItem<'static>> {
        self.map(|(idx, item)| {
            let prefix = if selected.contains(&idx) {
                "[x] "
            } else {
                "[ ] "
            };
            ListItem::new(format!("{}{}", prefix, item.to_string()))
        })
        .collect()
    }
}