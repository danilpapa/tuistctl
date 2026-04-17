use std::collections::HashSet;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{Modifier, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use crate::extensions::check_box_list_ext::CheckBoxListExt;

pub fn render_table_view(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    items: &Vec<String>,
    selected: &mut HashSet<usize>,
    cursor: usize,
) {
    _ = terminal.draw(|f| {
        let size = f.area();

        let ui_items: Vec<ListItem> = items
            .iter()
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