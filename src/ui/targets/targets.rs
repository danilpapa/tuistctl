use crossterm::{
    event::{self, Event}
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Modifier},
};
use crate::extensions::check_box_list_ext::CheckBoxListExt;
use crate::service::file_finder::{find_workspace};
use crate::service::scheme_parser::get_targets;
use crate::ui::app_state::AppState;
use crate::ui::keyboard::basic_actions::{handle_keyboard, Action};
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
                .to_checkbox_items(&state.selected);

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


