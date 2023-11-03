use rusqlite::Connection;

pub fn init_tables(db: &Connection) -> Result<(), rusqlite::Error> {
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
          date_created TEXT -- start of recur if type is non null
        );

        CREATE TABLE IF NOT EXISTS transaction_categories(
          transaction_id INTEGER NOT NULL,
          category_id INTEGER NOT NULL,
          PRIMARY KEY (transaction_id, category_id),
          FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
          FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE
        );
        "
    )?;
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

