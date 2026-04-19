use std::collections::HashSet;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph};
use crate::extensions::check_box_list_ext::CheckBoxListExt;
use crate::TerminalCFG;

pub fn render_table_view<I, T>(
    terminal: &mut TerminalCFG,
    items: I,
    selected: &mut HashSet<usize>,
    cursor: usize,
    warning: Option<&str>,
    title: &str,
) where
    I: Iterator<Item = T>,
    T: ToString,
{
    let title = title.to_string();
    _ = terminal.draw(move |f| {
        let area = f.area();

        let warning_height = if warning.is_some() { 3 } else { 0 };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(warning_height),
                Constraint::Length(1),
            ])
            .split(area);

        // --- Header ---
        let header = Paragraph::new(Line::from(vec![
            Span::styled("tuistctl", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("  //  ", Style::default().fg(Color::DarkGray)),
            Span::styled(&*title, Style::default().fg(Color::White)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Left);
        f.render_widget(header, chunks[0]);

        // --- List ---
        let ui_items: Vec<ListItem> = items
            .enumerate()
            .to_checkbox_items(selected);

        let mut list_state = ListState::default();
        list_state.select(Some(cursor));

        let list = List::new(ui_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(list, chunks[1], &mut list_state);

        // --- Warning ---
        if let Some(msg) = warning {
            let warn_area = chunks[2];
            f.render_widget(Clear, warn_area);

            let warning_text = Line::from(vec![
                Span::styled(" ! ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {}", msg), Style::default().fg(Color::Yellow)),
            ]);

            let warning_widget = Paragraph::new(warning_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Yellow))
                        .title(Span::styled(
                            " Warning ",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        )),
                )
                .alignment(Alignment::Left);

            f.render_widget(warning_widget, warn_area);
        }

        // --- Footer hints ---
        let footer = Paragraph::new(Line::from(vec![
            Span::styled("  ↑↓", Style::default().fg(Color::Cyan)),
            Span::styled(" navigate  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Space", Style::default().fg(Color::Cyan)),
            Span::styled(" select  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::styled(" confirm  ", Style::default().fg(Color::DarkGray)),
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::styled(" quit  ", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Left);
        f.render_widget(footer, chunks[3]);
    });
}
