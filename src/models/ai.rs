use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserAiRequest {
    pub input: String,
}
