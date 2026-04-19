use crate::service::file_finder::options_file;
use crate::service::option_parser::{get_options, TuistOption, TuistOptionsList};
use crossterm::{
    event::{self, Event},
};
use crate::TerminalCFG;
use crate::ui::app_state::AppState;
use crate::ui::keyboard::basic_actions::{handle_keyboard, Action};
use crate::ui::option_state::OptionState;
use crate::ui::table::table_view::render_table_view;

pub fn run_options_stage(
    app_state: &mut AppState,
    terminal: &mut TerminalCFG,
) -> anyhow::Result<(Vec<String>, Vec<TuistOption>)> {
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
    terminal: &mut TerminalCFG,
) -> anyhow::Result<(Vec<String>, Vec<TuistOption>)> {
    let result: (Vec<String>, Vec<TuistOption>) = loop {
        render_table_view(
            terminal,
            option_state.options.iter(),
            &mut option_state.selected,
            option_state.cursor,
            option_state.warning.as_deref(),
        );

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
                Action::Submit(selected) => {
                    break (selected, option_state.options.clone())
                },
                Action::Warning(msg) => {
                    option_state.warning = Some(msg);
                    continue;
                },
                Action::ClearWarning => {
                    option_state.warning = None;
                    continue;
                },
                Action::Exit => {
                    break (Vec::new(), Vec::new())
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
