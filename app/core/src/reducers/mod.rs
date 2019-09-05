use redux_rs::{Store};
use crate::state::{State};
use crate::{actions};

pub mod settings;


type Actions = actions::settings::Actions;

pub fn combine_reducer() -> Store<State, Actions>{
    Store::new(settings::reducer, State::default())
}
