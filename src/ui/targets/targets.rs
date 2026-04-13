use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Modifier},
};
use crate::obtain_targets;
use crate::ui::target_state::TargetsState;

pub fn run_targets_stage() -> anyhow::Result<Vec<String>> {
    let targets = obtain_targets();
    match targets {
        Ok(targets) => {
            let mut state = TargetsState::new(targets);
            return process_ui(&mut state);
        }
        Err(e) => {
            eprintln!("No targets found: {}", e);
            std::process::exit(1);
        }
    }
}

fn process_ui(state: &mut TargetsState) -> anyhow::Result<Vec<String>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = loop {
        terminal.draw(|f| {
            let size = f.area();

            let ui_items: Vec<ListItem> = state
                .targets
                .iter()
                .enumerate()
                .map(|(idx, target)| {
                    let prefix = if state.selected.contains(&idx) {
                        "[x] "
                    } else {
                        "[ ] "
                    };
                    return ListItem::new(format!("{}{}", prefix, target));
                })
                .collect();

            let mut list_state = ListState::default();
            list_state.select(Some(state.cursor));

            let list = List::new(ui_items)
                .block(Block::default().title("Targets").borders(Borders::ALL))
                .highlight_style(
                    Style::default().add_modifier(Modifier::REVERSED)
                );

            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break None,
                KeyCode::Down => {
                    if state.cursor + 1 < state.targets.len() {
                        state.cursor += 1;
                    } else {
                        state.cursor = 0;
                    }
                }
                KeyCode::Up => {
                    if state.cursor > 0 {
                        state.cursor -= 1;
                    } else {
                        state.cursor = state.targets.len() - 1;
                    }
                }
                KeyCode::Char(' ') => {
                    if state.selected.contains(&state.cursor) {
                        state.selected.remove(&state.cursor);
                    } else {
                        state.selected.insert(state.cursor);
                    }
                }
                KeyCode::Enter => {
                    let selected = state.selected
                        .iter()
                        .map(|&i| state.targets[i].clone())
                        .collect();
                    break Some(selected);
                }
                _ => {}
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(result.unwrap_or_default())
}


