use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Badge {
    rank: String,
    username: String,
    description: String,
}

pub fn get_badge(elapsed_secs: u64, username: &str) -> Option<Badge> {
    let mut rank = String::new();
    let mut description = String::new();
    let elapsed_hours: u64 = elapsed_secs / 60;
    match elapsed_hours {
        0 => {
            rank.push_str("Ground Zero");
            description.push_str("App just been added");
        }
        1 => {
            rank.push_str("Skin deep");
            description.push_str("You've been using app for an hour");
        }
        2 => {
            rank.push_str("Hooked");
            description.push_str("You've been using app for two hours");
        }
        10 => {
            rank.push_str("Into It");
            description.push_str("You've been using app for ten hours");
        }
        50 => {
            rank.push_str("Advanced user");
            description.push_str("You've been using app for an fifty hours");
        }
        100 => {
            rank.push_str("Pro");
            description.push_str("You've been using app for one hundren hours");
        }
        _ => (),
    };

    if rank != "" {
        return Some(Badge {
            rank,
            username: username.to_owned(),
            description,
        });
    } else {
        return None;
    }
}
