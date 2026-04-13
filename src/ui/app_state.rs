#[derive(Clone)]
pub enum AppState {
    Targets,
    Options,
    Generation,
}

impl AppState {
    pub fn next(&mut self) {
        *self = match self {
            AppState::Targets => AppState::Options,
            AppState::Options => AppState::Generation,
            AppState::Generation => AppState::Generation
        }
    }

    pub fn prev(&mut self) {
        *self = match self {
            AppState::Targets => AppState::Targets,
            AppState::Options => AppState::Targets,
            AppState::Generation => AppState::Options
        }
    }
    
    pub fn skip_options(&mut self) {
        *self = AppState::Generation
    }
}