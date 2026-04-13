use crossterm::{
    event::{self, Event, KeyCode}
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Modifier},
};
use crate::service::file_finder::{find_workspace, options_file};
use crate::service::scheme_parser::get_targets;
use crate::ui::app_state::AppState;
use crate::ui::target_state::TargetsState;

pub fn run_targets_stage(
    app_state: &mut AppState,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<Vec<String>> {
    let targets = obtain_targets();
    match targets {
        Ok(targets) => {
            let mut state = TargetsState::new(targets);
            return process_ui(
                app_state,
                terminal,
                &mut state,
            );
        },
        Err(e) => {
            eprintln!("No options to handle found: {}", e);
            std::process::exit(1);
        }
    }
}

fn obtain_targets() -> Result<Vec<String>, String> {
    let workspace_path = find_workspace()
        .expect("It is impossible to find \"workspace\" file in current file system");
    get_targets(&workspace_path)
}

fn process_ui(
    app_state: &mut AppState,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut TargetsState
) -> anyhow::Result<Vec<String>> {

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
                    ListItem::new(format!("{}{}", prefix, target))
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
                KeyCode::Char('q') => {
                    app_state.none();
                    break None
                },
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
                    let selected: Vec<String> = state.selected
                        .iter()
                        .map(|&i| state.targets[i].clone())
                        .collect();

                    if selected.is_empty() {
                        app_state.none();
                        break None;
                    }

                    if options_file().is_none() {
                        app_state.skip_options();
                    } else {
                        app_state.next();
                    }
                    break Some(selected);
                }
                _ => {}
            }
        }
    };

    Ok(result.unwrap_or_default())
}


