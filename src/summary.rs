use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};

pub fn total_in_range(conn: &Connection, start: NaiveDateTime, end: NaiveDateTime) -> Result<f64> {
    conn.query_row(
        "SELECT COALESCE(SUM(amount), 0.0) FROM expenses
         WHERE created_at >= ?1 AND created_at < ?2",
        params![
            start.format("%Y-%m-%d %H:%M:%S").to_string(),
            end.format("%Y-%m-%d %H:%M:%S").to_string()
        ],
        |row| row.get(0),
    )
}

pub fn by_category_in_range(
    conn: &Connection,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT category, SUM(amount) FROM expenses
         WHERE created_at >= ?1 AND created_at < ?2
         GROUP BY category
         ORDER BY SUM(amount) DESC",
    )?;
    stmt.query_map(
        params![
            start.format("%Y-%m-%d %H:%M:%S").to_string(),
            end.format("%Y-%m-%d %H:%M:%S").to_string()
        ],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?)),
    )?
    .collect()
}

pub fn print_stats(conn: &Connection, start: NaiveDateTime, end: NaiveDateTime) -> Result<()> {
    let total = total_in_range(conn, start, end)?;

    let days = (end.date() - start.date()).num_days();
    let avg_daily = if days > 0 { total / days as f64 } else { total };

    let top_category = by_category_in_range(conn, start, end)?
        .into_iter()
        .next();

    let highest: f64 = conn.query_row(
        "SELECT COALESCE(MAX(amount), 0.0) FROM expenses
         WHERE created_at >= ?1 AND created_at < ?2",
        params![
            start.format("%Y-%m-%d %H:%M:%S").to_string(),
            end.format("%Y-%m-%d %H:%M:%S").to_string()
        ],
        |row| row.get(0),
    )?;

    println!("  Total spent:            ₹{:.2}", total);
    println!("  Avg daily spending:     ₹{:.2}", avg_daily);
    println!("  Highest single expense: ₹{:.2}", highest);
    if let Some((cat, amt)) = top_category {
        println!("  Top category:           {} (₹{:.2})", cat, amt);
    }
    Ok(())
}
