use crate::ui::app_state::AppState;
use crate::ui::options::options::run_options_stage;
use crate::ui::targets::targets::run_targets_stage;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

mod service;
pub mod ui;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut screen_state = AppState::Targets;

    loop {
        match screen_state {
            AppState::Targets => {
                run_targets_stage(&mut screen_state, &mut terminal)?;
            }
            AppState::Options => {
                run_options_stage(&mut screen_state, &mut terminal)?;
            }
            AppState::Generation => break
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}