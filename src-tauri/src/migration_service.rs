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

// TODO this can be removed once testing is over
#[cfg(not(tarpaulin_include))]
pub fn init_db_with_data(db: &Connection) -> Result<(), rusqlite::Error> {
    let data = vec![
        (-10.0, "test1", Some("test1"), "2023-11-01", vec![1, 2, 3]),
        (20.0, "test2", Some("test2"), "2023-11-02", vec![1]),
        (30.0, "test3", Some("test3"), "2023-11-03", vec![2, 3]),
        (-5.0, "test4", Some("test4"), "2023-11-04", vec![1, 2]),
        (10.0, "test5", Some("test5"), "2023-11-05", vec![1, 3]),
        (20.0, "test6", Some("test6"), "2023-11-06", vec![1, 2]),
        (30.0, "test7", Some("test7"), "2023-11-07", vec![2, 3]),
        (-5.0, "test8", Some("test8"), "2023-11-08", vec![2, 3]),
        (10.0, "test9", Some("test9"), "2023-11-09", vec![1, 2, 3]),
        (-5.0, "test12", Some("test12"), "2023-11-12", vec![1, 2]),
        (-10.0, "test13", Some("test13"), "2023-11-13", vec![2, 3]),
        (-20.0, "test14", Some("test14"), "2023-11-14", vec![1, 2, 3]),
        (-30.0, "test15", Some("test15"), "2023-11-15", vec![1]),
        (-5.0, "test16", Some("test16"), "2023-11-16", vec![2, 3]),
        (10.0, "test17", Some("test17"), "2023-11-17", vec![1]),
        (20.0, "test18", Some("test18"), "2023-11-18", vec![2]),
        (30.0, "test19", Some("test19"), "2023-11-19", vec![3]),
        (-5.0, "test20", Some("test20"), "2023-11-20", vec![2]),
        (-10.0, "test21", Some("test21"), "2023-11-21", vec![1, 2, 3]),
        (-20.0, "test22", Some("test22"), "2023-11-22", vec![3]),
        (27.0, "test23", Some("test23"), "2023-11-28", vec![]),
    ];

    let mut iter = data.iter();

    while let Some((value, name, description, date_created, category_ids)) = iter.next() {
        crate::transaction_service::insert_transaction(
            db,
            *value,
            name,
            *description,
            date_created,
            category_ids.to_vec(),
        )?;
    }
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
