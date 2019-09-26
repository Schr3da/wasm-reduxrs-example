use super::Actions;
use super::settings::Settings;
use super::game::Game;

use std::rc::Rc;

#[derive(Default, Clone, Debug)]
pub struct AppState{
    pub settings: Settings,
    pub game: Game,
}

#[derive(Clone)]
pub struct OnChangeCallback {
    pub cb: Rc<dyn Fn(&State)>
}

impl OnChangeCallback {
    pub fn new(cb: Rc<dyn Fn(&State)>) -> Self {
        OnChangeCallback { cb }
    }
}
impl Default for OnChangeCallback {
    fn default() -> OnChangeCallback {
        let cb = Rc::new(|_: &State| {
            println!("OnChangeCallback deault called");
        });
        OnChangeCallback::new(cb)
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub callback: OnChangeCallback,
    pub validate_test: bool,
    pub prev: AppState,
    pub next: AppState,
}

pub fn next(state: &State) -> State {
    State {
        callback: state.callback.clone(),
        prev: state.next.clone(),
        next: state.next.clone(),
        ..*state
    }
}

pub fn default(state: &State) -> State {
    State {
        callback: state.callback.clone(),
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

fn app_set_on_change_callback(state: &State, callback: &OnChangeCallback) -> State {
    let mut next_state = next(state);
    next_state.callback = callback.clone(); 
    next_state
}


pub fn state_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::AppValidateTest(v) => app_validate_test(state, v),
        Actions::AppSetOnChangeCallback(cb) => app_set_on_change_callback(state, cb),
        _ => default(state),
    }
}

