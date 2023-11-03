use crate::models;
use rusqlite::{named_params, Connection};

pub fn insert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT OR IGNORE INTO categories (label) VALUES (:label);",
        named_params! {
            ":label": label.to_lowercase().trim()
        },
    )?;
    Ok(())
}

pub fn delete_category(db: &Connection, id: i32) -> Result<(), rusqlite::Error> {
    db.execute(
        "DELETE FROM categories WHERE id = (:id);",
        named_params! {
            ":id": id,
        },
    )?;
    Ok(())
}

pub fn get_categories(db: &Connection) -> Result<Vec<models::Category>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT id, label FROM categories;")?;
    let mut rows = stmt.query([])?;
    let mut categories = Vec::new();
    while let Some(row) = rows.next()? {
        categories.push(models::Category {
            id: row.get(0)?,
            label: row.get(1)?,
        });
    }
    Ok(categories)
}

#[cfg(test)]
use crate::migration_service::init_db_in_memory;

#[test]
fn insert_should_succeed() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "test")?;
    Ok(())
}

#[test]
fn query_should_return_nil_when_new() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    let list = get_categories(&conn)?;

    assert!(list.len() == 0, "Expected empty list, got {:?}", list);
    Ok(())
}

#[test]
fn after_insert_should_be_readable() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "test")?;
    let list = get_categories(&conn)?;

    assert!(
        list.len() == 1,
        "Expected list with one item, got {:?}",
        list
    );
    Ok(())
}

#[test]
fn insert_should_be_idempotent() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "test")?;
    insert_category(&conn, "test")?;
    let list = get_categories(&conn)?;

    assert!(
        list.len() == 1,
        "Expected list with one item, got {:?}",
        list
    );
    Ok(())
}

#[test]
fn insert_should_ignore_casing() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "test")?;
    insert_category(&conn, "TEST")?;
    let list = get_categories(&conn)?;

    assert!(
        list.len() == 1,
        "Expected list with one item, got {:?}",
        list
    );
    Ok(())
}

#[test]
fn insert_should_ignore_surrounding_whitespace() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "foobar")?;
    insert_category(&conn, " foobar")?;
    insert_category(&conn, "foobar")?;
    insert_category(&conn, " foobar")?;
    insert_category(&conn, "    foobar  ")?;
    insert_category(&conn, "foo bar")?;
    let list = get_categories(&conn)?;

    assert!(
        list.len() == 2,
        "Expected list with two items, got {:?}",
        list
    );
    Ok(())
}

#[test]
fn delete_should_remove_entry() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_category(&conn, "test")?;
    let mut list = get_categories(&conn)?;
    assert!(
        list.len() == 1,
        "Expected list with one item, got {:?}",
        list
    );

    let entry = list.pop().unwrap();

    delete_category(&conn, entry.id)?;

    list = get_categories(&conn)?;

    assert!(
        list.len() == 0,
        "Expected list with no items, got {:?}",
        list
    );

    Ok(())
}
