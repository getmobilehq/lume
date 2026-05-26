use crate::error::Result;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Once;

static VEC_INIT: Once = Once::new();

/// Register sqlite-vec as an auto-extension so every connection opened
/// afterwards has the `vec0` virtual table available. `tauri-plugin-sql` can't
/// load extensions, which is why we use rusqlite directly (see SKILLS.md).
fn register_vec_extension() {
    type InitFn = unsafe extern "C" fn(
        *mut rusqlite::ffi::sqlite3,
        *mut *mut std::os::raw::c_char,
        *const rusqlite::ffi::sqlite3_api_routines,
    ) -> std::os::raw::c_int;

    VEC_INIT.call_once(|| unsafe {
        rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute::<*const (), InitFn>(
            sqlite_vec::sqlite3_vec_init as *const (),
        )));
    });
}

/// Open the database at `db_path`, apply the schema, and verify sqlite-vec.
pub fn init(db_path: &Path) -> Result<Connection> {
    register_vec_extension();

    let conn = Connection::open(db_path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(include_str!("../migrations/0001_init.sql"))?;

    smoke_test_vec(&conn)?;
    Ok(conn)
}

/// Insert and read back a single vector in a throwaway temp table, proving the
/// extension is loaded and `vec0` works without touching real data.
fn smoke_test_vec(conn: &Connection) -> Result<()> {
    let version: String = conn.query_row("SELECT vec_version()", [], |row| row.get(0))?;

    conn.execute_batch(
        "CREATE VIRTUAL TABLE temp.vec_smoke USING vec0(embedding FLOAT[4]);
         INSERT INTO temp.vec_smoke(rowid, embedding) VALUES (1, '[0.1,0.2,0.3,0.4]');",
    )?;
    let stored: String = conn.query_row(
        "SELECT vec_to_json(embedding) FROM temp.vec_smoke WHERE rowid = 1",
        [],
        |row| row.get(0),
    )?;
    conn.execute_batch("DROP TABLE temp.vec_smoke;")?;

    log::info!("sqlite-vec {version} ok; smoke-test read back {stored}");
    Ok(())
}
