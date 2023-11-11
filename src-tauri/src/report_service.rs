use crate::models::{BasicReport, ReportType};
use chrono::NaiveDate;
use rusqlite::{named_params, Connection};
use std::collections::HashMap;

pub fn get_supported_report_types() -> Result<Vec<ReportType>, ()> {
    Ok(vec![ReportType::MONTH, ReportType::YEAR])
}

pub fn get_days_from_month(date: &str) -> i64 {
    let date_parts: Vec<&str> = date.split("-").collect();
    let year: i32 = date_parts[0].parse().unwrap();
    let month: u32 = date_parts[1].parse().unwrap();

    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

fn date_selector(selected_date: &str, report_type: ReportType) -> Result<String, rusqlite::Error> {
    // expected date is of format YYYY-MM-DD
    let date_parts: Vec<&str> = selected_date.split("-").collect();

    match (date_parts.as_slice(), report_type) {
        ([year, _, _], ReportType::YEAR) => Ok(format!("{}-__-__", year)),
        ([year, month, _], ReportType::MONTH) => Ok(format!("{}-{}-__", year, month)),
        _ => rusqlite::Result::Err(rusqlite::Error::InvalidQuery),
    }
}

fn get_dates_totals(db: &Connection, date: &str) -> Result<HashMap<String, f64>, rusqlite::Error> {
    let mut date_statement = db.prepare(
        "
        SELECT SUM(t.value), t.date_created FROM transactions t
        WHERE t.date_created LIKE (:date)
        GROUP BY t.date_created
        ",
    )?;

    let mut date_rows = date_statement.query(named_params! {
        ":date": date,
    })?;

    let mut dates: HashMap<String, f64> = HashMap::new();

    while let Some(row) = date_rows.next()? {
        let sum: f64 = row.get(0)?;
        let date_created: String = row.get(1)?;
        dates.insert(date_created, sum);
    }

    Ok(dates)
}

fn query_category_rows(
    db: &Connection,
    stmt: &str,
    date: &str,
) -> Result<HashMap<String, f64>, rusqlite::Error> {
    let mut categories_statement = db.prepare(stmt)?;

    let mut categories_rows = categories_statement.query(named_params! {
        ":date": date,
    })?;

    let mut categories: HashMap<String, f64> = HashMap::new();
    while let Some(row) = categories_rows.next()? {
        let labels: String = row.get(0)?;
        let sum: f64 = row.get(1)?;
        categories.insert(labels, sum);
    }

    Ok(categories)
}

fn get_category_income(
    db: &Connection,
    date: &str,
) -> Result<HashMap<String, f64>, rusqlite::Error> {
    let categories = query_category_rows(
        db,
        "
        WITH grouped_category AS (
            SELECT 
              tc.transaction_id as transaction_id,
              group_concat(c.id, ', ') as ids,
              group_concat(c.label, ', ') as labels
            FROM categories c
            INNER JOIN transaction_categories tc ON tc.category_id = c.id
            GROUP BY tc.transaction_id
        )

        SELECT gc.labels, SUM(t.value)
        FROM transactions t
        INNER JOIN grouped_category gc ON gc.transaction_id = t.id
        WHERE t.date_created LIKE (:date) AND t.value > 0
        GROUP BY t.date_created, gc.labels
        ",
        date,
    )?;
    Ok(categories)
}

fn get_category_expense(
    db: &Connection,
    date: &str,
) -> Result<HashMap<String, f64>, rusqlite::Error> {
    let categories = query_category_rows(
        db,
        "
        WITH grouped_category AS (
            SELECT 
              tc.transaction_id as transaction_id,
              group_concat(c.id, ', ') as ids,
              group_concat(c.label, ', ') as labels
            FROM categories c
            INNER JOIN transaction_categories tc ON tc.category_id = c.id
            GROUP BY tc.transaction_id
        )

        SELECT gc.labels, SUM(t.value)
        FROM transactions t
        INNER JOIN grouped_category gc ON gc.transaction_id = t.id
        WHERE t.date_created LIKE (:date) AND t.value <= 0
        GROUP BY t.date_created, gc.labels
        ",
        date,
    )?;
    Ok(categories)
}

fn get_uncategorized_total(db: &Connection, date: &str) -> Result<f64, rusqlite::Error> {
    let mut uncategorized_statement = db.prepare(
        "
        SELECT COALESCE(SUM(CAST(t.value AS REAL)), 0) as result
        FROM transactions t
        WHERE t.date_created LIKE (:date) AND t.id NOT IN (
            SELECT tc.transaction_id FROM transaction_categories tc
        )
        ",
    )?;

    let mut uncategorized_rows = uncategorized_statement.query(named_params! {
        ":date": date,
    })?;

    let mut uncategorized = 0.0;
    while let Some(row) = uncategorized_rows.next()? {
        uncategorized = row.get(0)?;
    }
    Ok(uncategorized)
}

pub fn get_basic_report(
    db: &Connection,
    report_type: ReportType,
    selected_date: &str,
) -> Result<BasicReport, rusqlite::Error> {
    let date = date_selector(selected_date, report_type)?;
    let raw_dates = get_dates_totals(db, &date)?;

    let mut total: f64 = 0.0;
    for (_, sum) in &raw_dates {
        total += sum;
    }

    let dates = match report_type {
        ReportType::YEAR => {
            let mut iter = raw_dates.iter();
            let mut grouped: HashMap<String, f64> = HashMap::new();
            let year = date.split("-").take(1).collect::<Vec<&str>>()[0];
            for i in 0..12 {
                grouped.insert(format!("{}-{:02}", year, i + 1), 0.0);
            }
            while let Some((date, sum)) = iter.next() {
                let month = date.split("-").take(2).collect::<Vec<&str>>().join("-");
                grouped.get_mut(&month).map(|v| *v += *sum);
            }
            grouped
        }
        ReportType::MONTH => {
            let mut grouped: HashMap<String, f64> = HashMap::new();
            let month = date.split("-").take(2).collect::<Vec<&str>>().join("-");
            let days = get_days_from_month(&date);
            for d in 0..days {
                grouped.insert(format!("{}-{:02}", month, d + 1), 0.0);
            }
            let mut iter = raw_dates.iter();
            while let Some((date, sum)) = iter.next() {
                // This is fine in this case in comparison to month, the aggeration is done in the
                // db query
                grouped.insert(date.to_string(), *sum);
            }
            grouped
        }
    };

    let report = BasicReport {
        total,
        uncategorized: get_uncategorized_total(db, &date)?,
        dates,
        category_income: get_category_income(db, &date)?,
        category_expenses: get_category_expense(db, &date)?,
    };

    Ok(report)
}

#[cfg(test)]
use crate::migration_service::init_db_in_memory;

#[test]
fn date_selector_should_return_year() -> Result<(), rusqlite::Error> {
    let date = date_selector("2023-11-01", ReportType::YEAR)?;
    assert_eq!(date, "2023-__-__");
    Ok(())
}

#[test]
fn date_selector_should_return_month() -> Result<(), rusqlite::Error> {
    let date = date_selector("2023-11-01", ReportType::MONTH)?;
    assert_eq!(date, "2023-11-__");
    Ok(())
}

#[test]
fn date_selector_should_return_error() -> Result<(), rusqlite::Error> {
    let date = date_selector("01-01", ReportType::MONTH);
    assert!(date.is_err());
    Ok(())
}

#[test]
fn should_return_correct_total_and_uncategorized() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    crate::transaction_service::insert_transaction(&conn, 1.0, "test", None, "2023-11-01", vec![])?;

    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;

    assert_eq!(report.total, 1.0);
    assert_eq!(report.uncategorized, 1.0);

    Ok(())
}

#[test]
fn should_return_zero_for_uncategorized() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    crate::category_service::insert_category(&conn, "test")?;
    let category = &crate::category_service::get_categories(&conn)?[0];

    crate::transaction_service::insert_transaction(
        &conn,
        1.0,
        "test",
        None,
        "2023-11-01",
        vec![category.id],
    )?;

    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;

    assert_eq!(report.total, 1.0);
    assert_eq!(report.uncategorized, 0.0);

    Ok(())
}

#[test]
fn should_return_correct_total_when_mix_of_income_and_expenses() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    crate::category_service::insert_category(&conn, "test")?;
    let category = &crate::category_service::get_categories(&conn)?[0];

    crate::transaction_service::insert_transaction(
        &conn,
        1.0,
        "test",
        None,
        "2023-11-01",
        vec![category.id],
    )?;

    crate::transaction_service::insert_transaction(
        &conn,
        -1.5,
        "test",
        None,
        "2023-11-01",
        vec![category.id],
    )?;

    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;

    assert_eq!(report.total, -0.5);
    assert_eq!(report.uncategorized, 0.0); //should be 0.0 because we have categories

    crate::category_service::delete_category(&conn, category.id)?;

    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;

    assert_eq!(report.total, -0.5);
    assert_eq!(report.uncategorized, -0.5); //should be -0.5 because we have no categories

    Ok(())
}

#[test]
fn report_should_fill_dates_with_empty_values_for_month() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;
    assert_eq!(report.dates.len(), 30);
    Ok(())
}

#[test]
fn report_should_fill_dates_with_empty_values_for_year() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    let report = get_basic_report(&conn, ReportType::YEAR, "2023-11-01")?;
    assert_eq!(report.dates.len(), 12);
    Ok(())
}

#[test]
fn report_should_fill_dates_with_empty_values_for_month_content() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;
    for (_, sum) in report.dates.iter() {
        assert_eq!(*sum, 0.0);
    }

    let mut dates: Vec<String> = report.dates.keys().cloned().collect();
    dates.sort();

    for (index, date) in dates.iter().enumerate() {
        assert!(date.starts_with("2023-11-"));
        let day = format!("{:02}", index + 1).to_string();
        assert!(date.ends_with(&day), "expected: {}, date: {}", day, date);
    }

    Ok(())
}

#[test]
fn year_report_should_combine_sums() -> Result<(), rusqlite::Error> {

    let conn = init_db_in_memory()?;
    crate::transaction_service::insert_transaction(&conn, 1.0, "test", None, "2023-11-01", vec![])?;
    crate::transaction_service::insert_transaction(&conn, 1.0, "test", None, "2023-11-02", vec![])?;

    let report = get_basic_report(&conn, ReportType::YEAR, "2023-11-01")?;
    
    assert_eq!(report.dates.len(), 12);
    assert_eq!(report.dates.get("2023-11").unwrap(), &2.0);

    Ok(())
}

#[test]
fn month_report_should_combine_sums_for_days() -> Result<(), rusqlite::Error> {
    let conn = init_db_in_memory()?;
    crate::transaction_service::insert_transaction(&conn, 1.0, "test", None, "2023-11-01", vec![])?;
    crate::transaction_service::insert_transaction(&conn, 1.0, "test", None, "2023-11-01", vec![])?;

    let report = get_basic_report(&conn, ReportType::MONTH, "2023-11-01")?;

    assert_eq!(report.dates.len(), 30);
    assert_eq!(report.dates.get("2023-11-01").unwrap(), &2.0);

    Ok(())
}

// TODO should add a test where categories are tested, so that grouping is working correctly
// for example if we have category 'foo', 'bar'
// we can have a transaction that is listed under
// 'foo, bar', 'foo' or 'bar'
