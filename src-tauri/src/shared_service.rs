use rusqlite::Connection;
use std::collections::HashMap;
use crate::models::Category;

pub fn query_transaction_category_rows(db: &Connection) -> Result<HashMap<i32, Vec<Category>>, rusqlite::Error> {

    let mut transaction_category_rows_statement = db.prepare(
        "
        SELECT ec.transaction_id, ec.category_id, c.label
        FROM transaction_categories ec
        JOIN categories c
        ON c.id = ec.category_id;
        ",
    )?;

    let mut transaction_category_rows = transaction_category_rows_statement.query([])?;

    let mut transaction_category_labels: HashMap<i32, Vec<Category>> = HashMap::new();

    while let Some(row) = transaction_category_rows.next()? {
        let transaction_id: i32 = row.get(0)?;
        let category_id: i32 = row.get(1)?;
        let category_label: String = row.get(2)?;

        let category = Category {
            id: category_id,
            label: category_label,
        };

        transaction_category_labels
            .entry(transaction_id)
            .or_insert_with(Vec::new)
            .push(category);
    }

    Ok(transaction_category_labels)
}
