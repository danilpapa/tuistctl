use crate::ui::app_state::AppState;
use crate::ui::options::options::run_options_stage;
use crate::ui::targets::targets::run_targets_stage;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use crate::service::option_parser::TuistOption;
use crate::ui::generation::generate_cmd;

mod service;
pub mod ui;
mod extensions;

pub type TerminalCFG = Terminal<CrosstermBackend<io::Stdout>>;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut screen_state = AppState::Targets;

    let mut targets: Vec<String> = Vec::new();

    let mut options: Vec<String> = Vec::new();
    let mut source_options: Vec<TuistOption> = Vec::new();

    loop {
        match &screen_state {
            AppState::Targets => {
                targets = run_targets_stage(&mut screen_state, &mut terminal)?;
            }
            AppState::Options => {
                (options, source_options) = run_options_stage(&mut screen_state, &mut terminal)?;
            }
            AppState::Generation => {
                // TODO: не переходит сюда
                let cmd: String = generate_cmd(&targets, &options, &source_options);
                println!("{}", cmd);
                screen_state.none();
            },
            AppState::None => {
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen
                )?;
                break
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}