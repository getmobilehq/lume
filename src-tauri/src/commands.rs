use crate::error::Result;
use keyring::Entry;
use uuid::Uuid;

const KEYCHAIN_SERVICE: &str = "com.getmobilehq.lume";
const KEYCHAIN_ACCOUNT: &str = "stronghold-vault";

/// Returns the Stronghold vault password, generating and storing a random one
/// in the macOS Keychain on first run. The password never touches disk in
/// plaintext — only the Keychain holds it; the vault file is encrypted with it.
#[tauri::command]
pub fn vault_password() -> Result<String> {
    let entry = Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)?;
    match entry.get_password() {
        Ok(password) => Ok(password),
        Err(keyring::Error::NoEntry) => {
            let password = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
            entry.set_password(&password)?;
            Ok(password)
        }
        Err(err) => Err(err.into()),
    }
}
