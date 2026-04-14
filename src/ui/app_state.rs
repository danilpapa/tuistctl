#[derive(Clone)]
pub enum AppState {
    Targets,
    Options,
    Generation,
    None
}

impl AppState {
    pub fn next(&mut self) {
        *self = match self {
            AppState::Targets => AppState::Options,
            AppState::Options => AppState::Generation,
            AppState::Generation => AppState::Generation,
            AppState::None => AppState::None
        }
    }

    pub fn prev(&mut self) {
        *self = match self {
            AppState::Targets => AppState::Targets,
            AppState::Options => AppState::Targets,
            AppState::Generation => AppState::Options,
            AppState::None => AppState::None
        }
    }

    pub fn skip_options(&mut self) {
        *self = AppState::Generation
    }

    pub fn none(&mut self) {
        *self = AppState::None
    }
}
