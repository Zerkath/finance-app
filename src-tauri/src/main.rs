// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, named_params};

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT OR IGNORE INTO categories (label) VALUES (:label)",
        named_params! {
            ":label": label.to_lowercase().trim()
        }
    )?;
    Ok(())
}

pub fn get_categories(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT label FROM categories")?;
    let mut rows = stmt.query([])?;
    let mut categories = Vec::new();
    while let Some(row) = rows.next()? {
        categories.push(row.get(0)?);
    }
    Ok(categories)
}

pub fn init_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL UNIQUE
        )"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // For tests we could mock the database interactions, 
    // but for the sake of simplicity we can also generate a database in memory
    fn init_db_in_memory() -> Result<Connection, rusqlite::Error> {
        let mut db = Connection::open_in_memory()?;
        init_tables(&mut db)?;
        Ok(db)
    }

    #[test]
    fn category_insert_should_succeed() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "test")?;
        Ok(())
    }

    #[test]
    fn category_query_should_return_nil_when_new() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        let list = get_categories(&conn)?;

        assert!(list.len() == 0, "Expected empty list, got {:?}", list);
        Ok(())
    }

    #[test]
    fn category_after_insert_should_be_readable() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "test")?;
        let list = get_categories(&conn)?;

        assert!(list.len() == 1, "Expected list with one item, got {:?}", list);
        Ok(())
    }

    #[test]
    fn category_insert_should_be_idempotent() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "test")?;
        upsert_category(&conn, "test")?;
        let list = get_categories(&conn)?;

        assert!(list.len() == 1, "Expected list with one item, got {:?}", list);
        Ok(())
    }

    #[test]
    fn category_insert_should_ignore_casing() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "test")?;
        upsert_category(&conn, "TEST")?;
        let list = get_categories(&conn)?;

        assert!(list.len() == 1, "Expected list with one item, got {:?}", list);
        Ok(())
    }

    #[test]
    fn category_insert_should_ignore_surrounding_whitespace() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "foobar")?;
        upsert_category(&conn, " foobar")?;
        upsert_category(&conn, "foobar")?;
        upsert_category(&conn, " foobar")?;
        upsert_category(&conn, "    foobar  ")?;
        upsert_category(&conn, "foo bar")?;
        let list = get_categories(&conn)?;

        assert!(list.len() == 2, "Expected list with two items, got {:?}", list);
        Ok(())
    }
}
