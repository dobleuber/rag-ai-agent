use serde::Deserialize;

use crate::agents::MyAgent;

#[derive(Deserialize)]
pub struct Prompt {
    pub prompt: String,
}

pub struct AppState {
    pub agent: MyAgent,
}
