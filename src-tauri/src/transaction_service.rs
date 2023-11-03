use rusqlite::{named_params, Connection};

use crate::models::{Transaction, Page};

pub fn query_page(
    db: &Connection,
    page_size: i32,
    current_page: i32,
) -> Result<Page, rusqlite::Error> {
    let count: i32 = db.query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))?;

    let total_pages = if count == 0 {
        1
    } else {
        (count as f64 / page_size as f64).ceil() as i32
    };

    let mut transactions: Vec<Transaction> = Vec::new();

    let mut transaction_rows_statement = db.prepare(
        "
           SELECT
           id,
           value,
           name,
           description,
           date_created
           FROM transactions 
           ORDER BY date_created ASC 
           LIMIT :page_size 
           OFFSET :offset
           ",
    )?;

    let mut transaction_rows = transaction_rows_statement.query(named_params! {
        ":page_size": page_size,
        ":offset": (current_page - 1) * page_size,
    })?;

    let mut transaction_category_labels = crate::shared_service::query_transaction_category_rows(db)?;

    while let Some(row) = transaction_rows.next()? {
        let id = row.get(0)?;

        let categories = transaction_category_labels.remove(&id).unwrap_or(Vec::new());

        let transaction = Transaction {
            id,
            value: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            date_created: row.get(4)?,
            categories,
        };
        transactions.push(transaction);
    }

    Ok(Page {
        total_pages,
        transactions,
    })
}

pub fn insert_transaction(
    db: &Connection,
    value: f64,
    name: &str,
    description: Option<&str>,
    date_created: Option<&str>,
    transaction_categories: Vec<i32>,
) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        INSERT INTO transactions(
            value,
            name,
            description,
            date_created
        )
        VALUES(
            :value,
            :name,
            :description,
            :date_created
        );
        ",
        named_params! {
            ":value": value,
            ":name": name,
            ":description": description,
            ":date_created": date_created
        },
    )?;

    let transaction_id = db.last_insert_rowid();

    for category in transaction_categories {
        let category_exists: i32 = db.query_row(
            "
            SELECT COUNT(*) FROM categories WHERE id = :id;
            ",
            named_params! {
                ":id": category,
            },
            |row| row.get(0),
        )?;

        // If the category does not exist, skip it
        if category_exists == 0 {
            continue;
        }

        db.execute(
            "
            INSERT INTO transaction_categories(
                transaction_id, 
                category_id
            ) 
            VALUES(
                :transaction_id, 
                :category_id
            );",
            named_params! {
                ":transaction_id": transaction_id,
                ":category_id": category,
            },
        )?;
    }

    Ok(())
}

pub fn delete_transaction(db: &Connection, id: i32) -> Result<(), rusqlite::Error> {
    db.execute(
        "DELETE FROM transactions WHERE id = :id",
        named_params! {
            ":id": id,
        },
    )?;

    Ok(())
}


#[cfg(test)]
use crate::migration_service::init_db_in_memory;

#[test]
fn insert_should_succeed() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_transaction(&conn, 1.0, "test", None, None, vec!())?;
    Ok(())
}

#[test]
fn query_should_return_nil_when_new() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    let page = query_page(&conn, 10, 1)?;

    assert!(page.transactions.len() == 0, "Expected empty list, got {:?}", page.transactions);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn query_should_return_entry_after_insert() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_transaction(&conn, 1.0, "test", None, None, vec!())?;
    let page = query_page(&conn, 10, 1)?;

    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn query_should_return_empty_page_if_added_entry_is_removed() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_transaction(&conn, 1.0, "test", None, None, vec!())?;
    delete_transaction(&conn, 1)?;
    let page = query_page(&conn, 10, 1)?;

    assert!(page.transactions.len() == 0, "Expected 0 entries, got {:?}", page.transactions);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn inserting_missing_categories_should_not_result_in_failure() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_transaction(&conn, 1.0, "test", None, None, vec!(1, 2))?;
    let page = query_page(&conn, 10, 1)?;

    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn querying_inserted_entry_with_non_existing_categories_should_return_empty_category_list() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    insert_transaction(&conn, 1.0, "test", None, None, vec!(1, 2))?;
    let page = query_page(&conn, 10, 1)?;

    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    assert!(page.transactions[0].categories.len() == 0, "Expected 0 categories, got {:?}", page.transactions[0].categories);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn querying_transaction_when_category_exists_should_return_correct_list() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;

    crate::category_service::insert_category(&conn, "test")?;
    let category = &crate::category_service::get_categories(&conn)?[0];

    insert_transaction(&conn, 1.0, "test", None, None, vec!(category.id))?;

    let page = query_page(&conn, 10, 1)?;
    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    let transaction = &page.transactions[0];
    assert!(transaction.categories.len() == 1, "Expected 1 category, got {:?}", transaction.categories);
    assert!(transaction.categories[0].id == category.id, "Expected category id {:?}, got {:?}", category.id, transaction.categories[0].id);
    assert!(transaction.categories[0].label == category.label, "Expected category label {:?}, got {:?}", category.label, transaction.categories[0].label);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    Ok(())
}

#[test]
fn querying_transaction_after_category_removal_should_reflect_the_change() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;

    crate::category_service::insert_category(&conn, "bar")?;
    crate::category_service::insert_category(&conn, "foo")?;
    let categories = &crate::category_service::get_categories(&conn)?;

    let category_1 = &categories[0];
    let category_2 = &categories[1];

    insert_transaction(&conn, 1.0, "test", None, None, vec!(category_1.id, category_2.id))?;
    let page = query_page(&conn, 10, 1)?;
    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    assert!(page.transactions[0].categories.len() == 2, "Expected 2 categories, got {:?}", page.transactions[0].categories);

    crate::category_service::delete_category(&conn, category_1.id)?;

    let page = query_page(&conn, 10, 1)?;
    assert!(page.transactions.len() == 1, "Expected 1 entry, got {:?}", page.transactions);
    let transaction = &page.transactions[0];
    assert!(transaction.categories.len() == 1, "Expected 1 categories, got {:?}", transaction.categories);
    assert!(page.total_pages == 1, "Expected 1 page, got {:?}", page.total_pages);
    assert!(transaction.categories[0].id == category_2.id, "Expected category id {:?}, got {:?}", category_2.id, transaction.categories[0].id);
    assert!(transaction.categories[0].label == category_2.label, "Expected category label {:?}, got {:?}", category_2.label, transaction.categories[0].label);
    Ok(())
}
