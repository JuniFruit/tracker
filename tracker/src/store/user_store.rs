use std::sync::{Mutex, MutexGuard};

use super::{ReducerMsg, Store};

lazy_static! {
    static ref USER_STORE: Mutex<Store<UserState, UserActions>> =
        Mutex::new(Store::new(Box::new(reducer)));
}

pub struct UserState {
    pub username: String,
    pub is_logged: bool,
}

impl Default for UserState {
    fn default() -> Self {
        UserState {
            username: "fruit".to_string(),
            is_logged: false,
        }
    }
}

fn reducer(state: &mut UserState, msg: UserActions) {
    match msg {
        UserActions::ChangeUsername(new_username) => println!("Change username"),
        _ => (),
    }
}

pub fn use_user_store() -> MutexGuard<'static, Store<UserState, UserActions>> {
    USER_STORE.lock().unwrap()
}

#[derive(Clone)]
pub enum UserActions {
    None,
    ChangeUsername(String),
}

impl ReducerMsg for UserActions {
    type Value = UserActions;
}
