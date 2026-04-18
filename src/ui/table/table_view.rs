use std::collections::HashSet;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use crate::extensions::check_box_list_ext::CheckBoxListExt;
use crate::TerminalCFG;

pub fn render_table_view<I, T>(
    terminal: &mut TerminalCFG,
    items: I,
    selected: &mut HashSet<usize>,
    cursor: usize,
) where
    I: Iterator<Item = T>,
    T: ToString,
{
    _ = terminal.draw(|f| {
        let size = f.area();

        let ui_items: Vec<ListItem> = items
            .enumerate()
            .to_checkbox_items(selected);

        let mut list_state = ListState::default();
        list_state.select(Some(cursor));

        let list = List::new(ui_items)
            .block(Block::default().title("Targets").borders(Borders::ALL))
            .highlight_style(
                Style::default().add_modifier(Modifier::REVERSED)
            );

        f.render_stateful_widget(list, size, &mut list_state);
    });
}