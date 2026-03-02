use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn daily_range(date: NaiveDate) -> (NaiveDateTime, NaiveDateTime) {
    let start = date.and_hms_opt(0, 0, 0).unwrap();
    let end = (date + Duration::days(1)).and_hms_opt(0, 0, 0).unwrap();
    (start, end)
}

pub fn weekly_range(today: NaiveDate) -> (NaiveDateTime, NaiveDateTime) {
    let days_from_monday = today.weekday().num_days_from_monday() as i64;
    let start = today - Duration::days(days_from_monday);
    let end = start + Duration::days(7);
    (
        start.and_hms_opt(0, 0, 0).unwrap(),
        end.and_hms_opt(0, 0, 0).unwrap(),
    )
}

pub fn monthly_range(year: i32, month: u32) -> (NaiveDateTime, NaiveDateTime) {
    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    (
        start.and_hms_opt(0, 0, 0).unwrap(),
        end.and_hms_opt(0, 0, 0).unwrap(),
    )
}

pub fn parse_year_month(s: &str) -> Result<(i32, u32), String> {
    let parts: Vec<&str> = s.splitn(2, '-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid month format '{}', expected YYYY-MM", s));
    }
    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| format!("Invalid year: {}", parts[0]))?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| format!("Invalid month: {}", parts[1]))?;
    if !(1..=12).contains(&month) {
        return Err(format!("Month must be 01-12, got {}", parts[1]));
    }
    Ok((year, month))
}

pub fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}
