use super::Actions;
use super::settings::Settings;
use super::game::Game;

#[derive(Default, Clone)]
pub struct State {
    pub validate_test: bool,
    pub settings: Settings,
    pub game: Game,
}

pub fn default(state: &State) -> State {
    State {
        game: state.game.clone(),
        ..*state
    }
}

fn app_validate_test(state: &State, value: &bool) -> State {
    State {
        validate_test: *value,
        game: state.game.clone(),
        ..*state
    }
}

pub fn state_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::AppValidateTest(v) => app_validate_test(state, v),
        _ => default(state),
    }

}


