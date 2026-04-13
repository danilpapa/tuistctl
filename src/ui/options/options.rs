use crate::service::file_finder::options_file;
use crate::service::option_parser::{get_options, TuistOptionsList};
use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Modifier},
};
use crate::ui::app_state::AppState;
use crate::ui::option_state::OptionState;

pub fn run_options_stage(
    app_state: &mut AppState,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<Vec<String>> {
    let options_result = obtain_options();
    match options_result {
        Ok(options) => {
            let mut state = OptionState::new(options.items);
            return process_ui(
                app_state,
                &mut state,
                terminal,
            );
        },
        Err(e) => {
            eprintln!("No options to handle found: {}", e);
            std::process::exit(1);
        }
    }
}
fn process_ui(
    app_state: &mut AppState,
    option_state: &mut OptionState,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<Vec<String>> {
    let result = loop {
        terminal.draw(|f| {
            let size = f.area();

            let ui_items: Vec<ListItem> = option_state
                .options
                .iter()
                .map(|option| {
                    option.name.clone()
                })
                .enumerate()
                // TODO: Extension to collection
                .map(|(idx, option)| {
                    let prefix = if option_state.selected.contains(&idx) {
                        "[x] "
                    } else {
                        "[ ] "
                    };
                    return ListItem::new(format!("{}{}", prefix, option));
                })
                .collect();

            let mut list_state = ListState::default();
            list_state.select(Some(option_state.cursor));

            let list = List::new(ui_items)
                .block(Block::default().title("Options").borders(Borders::ALL))
                .highlight_style(
                    Style::default().add_modifier(Modifier::REVERSED)
                );

            f.render_stateful_widget(list, size, &mut list_state);
        })?;
        // TODO: Reusable
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app_state.prev();
                    break None
                },
                KeyCode::Down => {
                    if option_state.cursor + 1 < option_state.options.len() {
                        option_state.cursor += 1;
                    } else {
                        option_state.cursor = 0;
                    }
                }
                KeyCode::Up => {
                    if option_state.cursor > 0 {
                        option_state.cursor -= 1;
                    } else {
                        option_state.cursor = option_state.options.len() - 1;
                    }
                }
                KeyCode::Char(' ') => {
                    if option_state.selected.contains(&option_state.cursor) {
                        option_state.selected.remove(&option_state.cursor);
                    } else {
                        option_state.selected.insert(option_state.cursor);
                    }
                }
                KeyCode::Enter => {
                    let selected = option_state.selected
                        .iter()
                        .map(|&i| option_state.options[i].name.clone())
                        .collect();
                    app_state.next();
                    break Some(selected);
                }
                _ => {}
            }
        }
    };

    Ok(result.unwrap_or_default())
}

fn obtain_options() -> Result<TuistOptionsList, String> {
    if let Some(option_file_path) = options_file() {
        return Ok(get_options(&option_file_path))
    };
    Err("Hello".to_string())
}
