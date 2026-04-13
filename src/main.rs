use crate::service::option_parser::{get_options, TuistOption, TuistOptionsList};
use crate::service::file_finder::{find_workspace, options_file};
use crate::service::scheme_parser::get_targets;
use crate::ui::app_state::AppState;
use crate::ui::targets::targets::run_targets_stage;

mod service;
pub mod ui;

fn main() {
    let mut screen_state = AppState::Targets;

    loop {
        match screen_state.clone() {
            AppState::Targets => {
                let selected_targets = run_targets_stage()
                    .expect("Failed to get targets from screen");

                println!("Selected targets: {:?}", selected_targets);
                break;
            },
            AppState::Options => {

            },
            AppState::Generation => {

            }
        }
    }
}

fn obtain_targets() -> Result<Vec<String>, String> {
    let workspace_path = find_workspace()
        .expect("It is impossible to find \"workspace\" file in current file system");
    get_targets(&workspace_path)
}

fn obtain_options() -> Result<TuistOptionsList, String> {
    if let Some(option_file_path) = options_file() {
        return Ok(get_options(&option_file_path))
    };
    Err("Hello".to_string())
}
