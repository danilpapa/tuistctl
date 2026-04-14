use crate::service::file_finder::options_file;
use crate::service::option_parser::{get_options, TuistOptionsList};
use crossterm::{
    event::{self, Event},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Modifier},
};
use crate::extensions::check_box_list_ext::CheckBoxListExt;
use crate::ui::app_state::AppState;
use crate::ui::keyboard::basic_actions::{handle_keyboard, Action};
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
                .to_checkbox_items(&option_state.selected);

            let mut list_state = ListState::default();
            list_state.select(Some(option_state.cursor));

            let list = List::new(ui_items)
                .block(Block::default().title("Options").borders(Borders::ALL))
                .highlight_style(
                    Style::default().add_modifier(Modifier::REVERSED)
                );

            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if let Event::Key(key) = event::read()? {
            let string_options: Vec<String> = option_state
                .options
                .iter()
                .map(|option| option.name.clone())
                .collect();

            let result = handle_keyboard(
                key,
                app_state,
                &mut option_state.cursor,
                &string_options,
                option_state.options.len(),
                &mut option_state.selected
            );

            match result {
                Action::Submit(selected) => break selected,
                Action::Exit => {
                    app_state.prev();
                    break Vec::new()
                },
                Action::Continue => continue
            }
        }
    };

    Ok(result)
}

fn obtain_options() -> Result<TuistOptionsList, String> {
    if let Some(option_file_path) = options_file() {
        return Ok(get_options(&option_file_path))
    };
    Err("Hello".to_string())
}
