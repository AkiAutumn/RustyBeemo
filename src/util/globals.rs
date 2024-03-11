use rand::{thread_rng, Rng};
use serenity::all::UserId;

static POSITIVE_EMOTES: [&str; 7] = [":)", ";)", ":D", ":3", ":>", ":P", "(^-^)"];
static NEGATIVE_EMOTES: [&str; 6] = [":(", ";(", ":<", "D:", "T-T", "('^')"];
static NEUTRAL_EMOTES: [&str; 2] = ["(>-<)", "('-')"];
pub enum Mood {
    Positive,
    Neutral,
    Negative
}

static PRIVILEGED_USERS: [u64; 2] = [309307881205923840, 197424794063470592];
//                                   ^Annika             ^Daniel
pub fn is_privileged_user(user_id: UserId) -> bool {
    PRIVILEGED_USERS.contains(&u64::from(user_id))
}

pub fn random_emote(mood: Mood) -> &'static str {
    match mood  {
        Mood::Positive => POSITIVE_EMOTES[thread_rng().gen_range(0..POSITIVE_EMOTES.len())],
        Mood::Neutral => NEUTRAL_EMOTES[thread_rng().gen_range(0..NEUTRAL_EMOTES.len())],
        Mood::Negative => NEGATIVE_EMOTES[thread_rng().gen_range(0..NEGATIVE_EMOTES.len())]
    }
}