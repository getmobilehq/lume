use serde::{Serialize, Serializer};

/// All errors that cross a Tauri command boundary. Serialises to its message
/// string so the frontend receives actionable text (AGENT.md: no silent failures).
#[derive(Debug, thiserror::Error)]
pub enum LumeError {
    #[error("keychain error: {0}")]
    Keychain(#[from] keyring::Error),

    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
}

impl Serialize for LumeError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LumeError>;
