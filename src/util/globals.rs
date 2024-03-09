use rand::{thread_rng, Rng};

static EMOTES: [&str; 6] = [":)", ";)", ":D", ":3", ":>", ":P"];

pub fn random_emote() -> &'static str {
    EMOTES[thread_rng().gen_range(0..EMOTES.len())]
}