use crossterm::{
    event::{self, Event}
};
use crate::service::file_finder::{find_workspace};
use crate::service::scheme_parser::get_targets;
use crate::TerminalCFG;
use crate::ui::app_state::AppState;
use crate::ui::keyboard::basic_actions::{handle_keyboard, Action};
use crate::ui::table::table_view::render_table_view;
use crate::ui::target_state::TargetsState;

pub fn run_targets_stage(
    app_state: &mut AppState,
    terminal: &mut TerminalCFG,
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
    terminal: &mut TerminalCFG,
    state: &mut TargetsState
) -> anyhow::Result<Vec<String>> {
    let result = loop {
        render_table_view(
            terminal,
            state.targets.iter(),
            &mut state.selected,
            state.cursor,
            state.warning.as_deref(),
            None,
            "Step 1/2 — Select Targets",
        );

        if let Event::Key(key) = event::read()? {
            let result = handle_keyboard(
                key,
                app_state,
                &mut state.cursor,
                &state.targets,
                state.targets.len(),
                &mut state.selected
            );

            match result {
                Action::Submit(selected) => break selected,
                Action::Warning(msg) => {
                    state.warning = Some(msg);
                    continue;
                },
                Action::ClearWarning => {
                    state.warning = None;
                    continue;
                },
                Action::Exit => {
                    app_state.none();
                    break Vec::new()
                },
                Action::Continue => continue
            }
        }
    };
    Ok(result)
}


