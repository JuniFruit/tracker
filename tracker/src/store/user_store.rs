use std::sync::{Mutex, MutexGuard};

use crate::win_funcs::user::get_username;

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
            username: "".to_string(),
            is_logged: false,
        }
    }
}

impl UserState {
    fn change_username(&mut self, new_name: &str) {
        if self.username == *new_name {
            return;
        };

        self.username = new_name.to_owned();
    }

    fn init_username(&mut self) {
        if self.is_logged {
            todo!()
        } else {
            match get_username() {
                Ok(username) => {
                    println!("Username: {}", username);
                    self.username = username;
                }
                Err(e) => println!("Couldn't get logon username.{}", e),
            }
        }
    }
}

fn reducer(state: &mut UserState, msg: UserActions) {
    match msg {
        UserActions::ChangeUsername(new_username) => {
            state.change_username(&new_username);
        }
        UserActions::InitConfig => {
            state.is_logged = false;
            state.init_username();
        }
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
    InitConfig,
}

impl ReducerMsg for UserActions {
    type Value = UserActions;
}
