use std::collections::HashSet;
use crossterm::event::{KeyCode, KeyEvent};
use crate::ui::app_state::AppState;

pub enum Action {
    Continue,
    ClearWarning,
    Submit(Vec<String>),
    Warning(String),
    Exit,
}

pub fn handle_keyboard(
    key: KeyEvent,
    app_state: &mut AppState,
    cursor: &mut usize,
    items: &[String],
    total_items: usize,
    selected: &mut HashSet<usize>,
) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            return match *app_state {
                AppState::Targets => {
                    app_state.none();
                    Action::Exit
                },
                AppState::Options => {
                    app_state.prev();
                    Action::Exit
                },
                _ => Action::Continue
            }
        },
        KeyCode::Down => {
            if *cursor < total_items - 1 {
                *cursor += 1;
            } else {
                *cursor = 0;
            }
            return Action::ClearWarning;
        }
        KeyCode::Up => {
            if *cursor > 0 {
                *cursor -= 1;
            } else {
                *cursor = total_items - 1;
            }
            return Action::ClearWarning;
        }
        KeyCode::Char(' ') => {
            if selected.contains(cursor) {
                selected.remove(cursor);
            } else {
                selected.insert(*cursor);
            }
            return Action::ClearWarning;
        }
        KeyCode::Enter => {
            let selected: Vec<String> = selected
                .iter()
                .map(|&i| items[i].clone())
                .collect();

            if selected.is_empty() {
                return Action::Warning(
                    "Select targets/options to generate your project".to_string()
                );
            }

            app_state.next();
            return Action::Submit(selected);
        }
        _ => {
            return Action::Continue;
        }
    }
    return Action::Continue;
}