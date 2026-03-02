use chrono::{Local, NaiveDate, NaiveDateTime};
use rusqlite::{params, Connection, Result};

pub struct Expense {
    pub id: i64,
    pub category: String,
    pub amount: f64,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
}

pub fn add(
    conn: &Connection,
    category: &str,
    amount: f64,
    note: Option<&str>,
    date: Option<NaiveDate>,
) -> Result<()> {
    let created_at = match date {
        Some(d) => d.and_hms_opt(0, 0, 0).unwrap(),
        None => Local::now().naive_local(),
    };
    let created_at_str = created_at.format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO expenses (category, amount, note, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![category, amount, note, created_at_str],
    )?;
    let id = conn.last_insert_rowid();
    println!("Added expense #{}: {} ₹{:.2}", id, category, amount);
    Ok(())
}

pub fn list(
    conn: &Connection,
    range: Option<(NaiveDateTime, NaiveDateTime)>,
) -> Result<Vec<Expense>> {
    if let Some((start, end)) = range {
        let mut stmt = conn.prepare(
            "SELECT id, category, amount, note, created_at FROM expenses
             WHERE created_at >= ?1 AND created_at < ?2
             ORDER BY created_at DESC",
        )?;
        stmt.query_map(
            params![
                start.format("%Y-%m-%d %H:%M:%S").to_string(),
                end.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
            row_to_expense,
        )?
        .collect()
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, category, amount, note, created_at FROM expenses
             ORDER BY created_at DESC",
        )?;
        stmt.query_map([], row_to_expense)?.collect()
    }
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let rows = conn.execute("DELETE FROM expenses WHERE id = ?1", params![id])?;
    if rows == 0 {
        println!("No expense found with id #{}", id);
    } else {
        println!("Deleted expense #{}", id);
    }
    Ok(())
}

pub fn edit(
    conn: &Connection,
    id: i64,
    category: Option<&str>,
    amount: Option<f64>,
    note: Option<&str>,
    date: Option<NaiveDate>,
) -> Result<()> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM expenses WHERE id = ?1",
        params![id],
        |row| row.get::<_, i64>(0),
    )? > 0;

    if !exists {
        println!("No expense found with id #{}", id);
        return Ok(());
    }

    if let Some(cat) = category {
        conn.execute(
            "UPDATE expenses SET category = ?1 WHERE id = ?2",
            params![cat, id],
        )?;
    }
    if let Some(amt) = amount {
        conn.execute(
            "UPDATE expenses SET amount = ?1 WHERE id = ?2",
            params![amt, id],
        )?;
    }
    if let Some(n) = note {
        conn.execute(
            "UPDATE expenses SET note = ?1 WHERE id = ?2",
            params![n, id],
        )?;
    }
    if let Some(d) = date {
        let dt = d
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        conn.execute(
            "UPDATE expenses SET created_at = ?1 WHERE id = ?2",
            params![dt, id],
        )?;
    }
    println!("Updated expense #{}", id);
    Ok(())
}

fn row_to_expense(row: &rusqlite::Row) -> rusqlite::Result<Expense> {
    let created_at_str: String = row.get(4)?;
    let created_at =
        NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S").map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                4,
                rusqlite::types::Type::Text,
                Box::new(e),
            )
        })?;
    Ok(Expense {
        id: row.get(0)?,
        category: row.get(1)?,
        amount: row.get(2)?,
        note: row.get(3)?,
        created_at,
    })
}

pub fn print_table(expenses: &[Expense]) {
    if expenses.is_empty() {
        println!("No expenses found.");
        return;
    }
    println!(
        "{:<5} {:<12} {:<16} {:>10}  {}",
        "ID", "Date", "Category", "Amount", "Note"
    );
    println!("{}", "-".repeat(60));
    for e in expenses {
        println!(
            "{:<5} {:<12} {:<16} {:>10}  {}",
            e.id,
            e.created_at.format("%Y-%m-%d"),
            e.category,
            format!("₹{:.2}", e.amount),
            e.note.as_deref().unwrap_or("")
        );
    }
}
