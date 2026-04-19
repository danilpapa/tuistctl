use std::collections::HashSet;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use crate::extensions::check_box_list_ext::CheckBoxListExt;
use crate::TerminalCFG;

pub fn render_table_view<I, T>(
    terminal: &mut TerminalCFG,
    items: I,
    selected: &mut HashSet<usize>,
    cursor: usize,
    warning: Option<&str>,
) where
    I: Iterator<Item = T>,
    T: ToString,
{
    _ = terminal.draw(|f| {
        let area = f.area();

        // Split vertically: list on top, optional warning bar at bottom
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(if warning.is_some() {
                vec![Constraint::Min(1), Constraint::Length(3)]
            } else {
                vec![Constraint::Min(1), Constraint::Length(0)]
            })
            .split(area);

        // --- List ---
        let ui_items: Vec<ListItem> = items
            .enumerate()
            .to_checkbox_items(selected);

        let mut list_state = ListState::default();
        list_state.select(Some(cursor));

        let list = List::new(ui_items)
            .block(Block::default().title("Targets").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(list, chunks[0], &mut list_state);

        // --- Warning popup ---
        if let Some(msg) = warning {
            let warn_area = chunks[1];

            // Clear background so the block renders cleanly
            f.render_widget(Clear, warn_area);

            let warning_text = Line::from(vec![
                Span::styled(" ! ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {} ", msg), Style::default().fg(Color::Yellow)),
            ]);

            let warning_widget = Paragraph::new(warning_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow))
                        .title(Span::styled(
                            " Warning ",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        )),
                )
                .alignment(Alignment::Left);

            f.render_widget(warning_widget, warn_area);
        }
    });
}