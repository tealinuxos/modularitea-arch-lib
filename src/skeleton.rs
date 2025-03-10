use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    AUR,
    Official,
    URL,
}
