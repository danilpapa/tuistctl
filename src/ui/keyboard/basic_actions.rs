use std::collections::HashSet;
use crossterm::event::{KeyCode, KeyEvent};
use crate::ui::app_state::AppState;

pub enum Action {
    Continue,
    Submit(Vec<String>),
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
            match *app_state {
                AppState::Targets => {
                    app_state.none();
                    return Action::Exit;
                },
                AppState::Options => {
                    app_state.prev();
                    return Action::Exit;
                },
                _ => return Action::Continue
            }
        },
        KeyCode::Down => {
            if *cursor < total_items - 1 {
                *cursor += 1;
            } else {
                *cursor = 0;
            }
        }
        KeyCode::Up => {
            if *cursor > 0 {
                *cursor -= 1;
            } else {
                *cursor = total_items - 1;
            }
        }
        KeyCode::Char(' ') => {
            if selected.contains(cursor) {
                selected.remove(cursor);
            } else {
                selected.insert(*cursor);
            }
        }
        KeyCode::Enter => {
            let selected: Vec<String> = selected
                .iter()
                .map(|&i| items[i].clone())
                .collect();

            match app_state {
                AppState::Targets => {
                    if selected.is_empty() {
                        app_state.none();
                        // TODO: UI warning
                        eprintln!("Cannot handle action with out selection of targets");
                        return Action::Exit;
                    }
                }
                _ => {},
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