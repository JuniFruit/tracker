use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Badge {
    pub rank: BadgeRank,
    pub username: String,
    pub description: String,
}

pub fn get_badge(elapsed_secs: u64, username: &str) -> Option<Badge> {
    let mut rank: Option<BadgeRank> = None;
    let mut description = String::new();
    let elapsed_hours: u64 = elapsed_secs / 3600;
    match elapsed_hours {
        0 => {
            rank = Some(BadgeRank::Initial);
            description.push_str("App's just been added.");
        }
        1 => {
            rank = Some(BadgeRank::Common);
            description.push_str("You've been using app for an hour. Keep it up.");
        }
        2 => {
            rank = Some(BadgeRank::Rare);
            description.push_str("You've been using app for two hours. Not bad.");
        }
        10 => {
            rank = Some(BadgeRank::Experienced);
            description
                .push_str("You've been using app for ten hours. I think you're already into it.");
        }
        50 => {
            rank = Some(BadgeRank::Advanced);
            description.push_str("You've been using app for an fifty hours. Point of no return.");
        }
        100 => {
            rank = Some(BadgeRank::Pro);
            description
                .push_str("You've been using app for one hundred hours. You're already hooked.");
        }
        500 => {
            rank = Some(BadgeRank::Insane);
            description.push_str(
                "You've been using app for five hundred hours. You really like this, don't you?",
            );
        }
        1000 => {
            rank = Some(BadgeRank::Lunatic);
            description.push_str(
                "You've been using app for one thousand hours. You know everything about this app.",
            );
        }
        3000 => {
            rank = Some(BadgeRank::TouchGrass);
            description.push_str(
                "You've been using app for three thousand hours. Can't believe I just said that.",
            );
        }
        10000 => {
            rank = Some(BadgeRank::Master);
            description
                .push_str("You've been using app for ten thousand hours. You've mastered it all");
        }

        _ => (),
    };

    if rank.is_some() {
        return Some(Badge {
            rank: rank.unwrap(),
            username: username.to_owned(),
            description,
        });
    } else {
        return None;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BadgeRank {
    Initial,
    Common,
    Rare,
    Experienced,
    Advanced,
    Pro,
    Insane,
    Lunatic,
    TouchGrass,
    Master,
}
