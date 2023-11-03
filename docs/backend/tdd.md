# Backend design with tdd

To start out with we can create tests in seperate file or in the same file as the implementation.
I have decided to make my tests at the 'api' level. 
The build system allows us to run either `npm run test:backend:watch` or `cargo watch -x test` depending on which folder we are in.

To start out with we can create a test module with a failing assertion to verify our environment works.

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn should_fail() -> Result<(), ()> {
        Err(())
    }
}
```

The initial requirement for our application is to have a few APIs that query data or insert it.
A good place to start is the categories CRUD actions.

```rust

// This function will handle the operation of adding and generating a ID for the category
// Note that this function accepts a reference to a connection
pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {

    // We want to fail for now as we don't have everything setup
    Err(rusqlite::Error::InvalidQuery)
}

#[cfg(test)]
mod tests {

    use super::*;

    // For tests we could mock the database interactions, 
    // but for the sake of simplicity we can also generate a database in memory
    fn init_db_in_memory() -> Result<Connection, rusqlite::Error> {
        let mut db = Connection::open_in_memory()?;
        Ok(db)
    }

    #[test]
    fn category_insert_should_succeed() -> Result<(), rusqlite::Error>{
        let conn = init_db_in_memory()?;
        upsert_category(&conn, "test")?;
        Ok(())
    }
}
```

Our first test assertion is ready, we need to update our implementation for the function.
But before this we need to also initialize our database with the correct table.

```rust
pub fn init_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL UNIQUE
        )"
    )?;
    Ok(())
}


fn init_db_in_memory() -> Result<Connection, rusqlite::Error> {
    let mut db = Connection::open_in_memory()?;
    init_tables(&mut db)?;
    Ok(db)
}
```

Now we have access to the table required by the test, and we can start to work on the implementation.

```rust
pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT OR IGNORE INTO categories (label) VALUES (:label)",
        named_params! {
            ":label": label
        }
    )?;
    Ok(())
}
```

After this change the test is passing, the function is short enough that it does not require refactors.
We can start to add new assertions, or different functions to be tested. 
In this case we want to add a function to query all categories, 
then we can add another test where we insert a row and check the results.

```rust
pub fn get_categories(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT label FROM categories")?;
    let mut rows = stmt.query([])?;
    let mut categories = Vec::new();
    while let Some(row) = rows.next()? {
        categories.push(row.get(0)?);
    }
    Ok(categories)
}

#[test]
fn category_query_should_return_nil_when_new() -> Result<(), rusqlite::Error>{
    let conn = init_db_in_memory()?;
    let list = get_categories(&conn)?;

    if list.len() > 0 {
        panic!("Expected empty list, got {:?}", list);
    } else {
        Ok(())
    }
}

#[test]
fn category_after_insert_should_be_readable() -> Result<(), rusqlite::Error>{
    let conn = init_db_in_memory()?;
    upsert_category(&conn, "test")?;
    let list = get_categories(&conn)?;

    if list.len() != 1 {
        panic!("Expected list with one item, got {:?}", list);
    } else {
        Ok(())
    }
}
```

We have asserted some basic expectations for the insert endpoint.
Few unsaid things about this endpoint are that the request should be idempotent, making the query multilpe times does not affect the result additively only changes behavior once.
An extra requirement is that all categories are sanitized from surrounding whitespace and stored in lowercase.

We can test idempotency by making the upsert twice.

```rust
#[test]
fn category_insert_should_be_idempotent() -> Result<(), rusqlite::Error>{
    let conn = init_db_in_memory()?;
    upsert_category(&conn, "test")?;
    upsert_category(&conn, "test")?;
    let list = get_categories(&conn)?;

    if list.len() != 1 {
        panic!("Expected list with one item, got {:?}", list);
    } else {
        Ok(())
    }
}
```

The previous test case passed, but for a fact we know for the following it should fail.

```rust
#[test]
fn category_insert_should_ignore_casing() -> Result<(), rusqlite::Error>{
    let conn = init_db_in_memory()?;
    upsert_category(&conn, "test")?;
    upsert_category(&conn, "TEST")?;
    let list = get_categories(&conn)?;

    if list.len() != 1 {
        panic!("Expected list with one item, got {:?}", list);
    } else {
        Ok(())
    }
}
```


As we expected this test fails and we now need to update the implementation to resolve the issue.

```rust
pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT OR IGNORE INTO categories (label) VALUES (:label)",
        named_params! {
            ":label": label.to_lowercase()
        }
    )?;
    Ok(())
}
```

This fixes the failing test and we can now test for whitespace.
We don't want to remove whitespace inside the label, but any extra surrounding should be removed

```rust
#[test]
fn category_insert_should_ignore_surrounding_whitespace() -> Result<(), rusqlite::Error>{
    let conn = init_db_in_memory()?;
    upsert_category(&conn, "foobar")?;
    upsert_category(&conn, " foobar")?;
    upsert_category(&conn, "foobar")?;
    upsert_category(&conn, " foobar")?;
    upsert_category(&conn, "foo bar")?;
    let list = get_categories(&conn)?;

    if list.len() != 2 {
        panic!("Expected list with two items, got {:?}", list);
    } else {
        Ok(())
    }
}
```

The test is failing as we expected and we can update the implementation once again.

```rust
pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT OR IGNORE INTO categories (label) VALUES (:label)",
        named_params! {
            ":label": label.to_lowercase().trim()
        }
    )?;
    Ok(())
}
```

All tests are now passing, this concludes the backend TDD portion.
