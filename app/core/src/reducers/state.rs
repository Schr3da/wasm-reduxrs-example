use super::Actions;
use super::settings::Settings;
use super::game::Game;

#[derive(Default, Clone)]
pub struct AppState{
    pub settings: Settings,
    pub game: Game,
}

#[derive(Default, Clone)]
pub struct State {
    pub validate_test: bool,
    pub prev: AppState,
    pub next: AppState,
}

pub fn next(state: &State) -> State {
    State {
        prev: state.next.clone(),
        next: state.next.clone(),
        ..*state
    }
}

pub fn default(state: &State) -> State {
    State {
        prev: state.prev.clone(),
        next: state.next.clone(),
        ..*state
    }
}

fn app_validate_test(state: &State, value: &bool) -> State {
    let mut next_state = next(state);
    next_state.validate_test = *value; 
    next_state
}

pub fn state_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::AppValidateTest(v) => app_validate_test(state, v),
        _ => default(state),
    }
}

