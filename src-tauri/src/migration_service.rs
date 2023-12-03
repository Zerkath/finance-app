use rusqlite::Connection;

pub fn init_tables(db: &Connection) -> Result<(), rusqlite::Error> {

    // load optional modules
    rusqlite::vtab::array::load_module(db)?;
    
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            label TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS transactions(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          value REAL NOT NULL, -- monetary value, no currency based on users locale
          name TEXT NOT NULL, -- mandatory name of transactions
          description TEXT, -- optional text
          date_created TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS transaction_categories(
          transaction_id INTEGER NOT NULL,
          category_id INTEGER NOT NULL,
          PRIMARY KEY (transaction_id, category_id),
          FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
          FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE
        );
        ",
    )?;
    Ok(())
}

fn drop_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch(
        "
        DROP TABLE IF EXISTS transaction_categories;
        DROP TABLE IF EXISTS transactions;
        DROP TABLE IF EXISTS categories;
        "
    )?;
    Ok(())
}

pub fn reset_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    drop_tables(db)?;
    init_tables(db)?;
    Ok(())
}

// For tests we could mock the database interactions,
// but for the sake of simplicity we can also generate a database in memory
#[allow(dead_code)]
pub fn init_db_in_memory() -> Result<Connection, rusqlite::Error> {
    let mut db = Connection::open_in_memory()?;
    init_tables(&mut db)?;
    Ok(db)
}
