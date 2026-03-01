Perfect. Below is a **clean, copy-paste ready `IMPLEMENTATION.md`** file for your project.

You can drop this directly into your repo.

---

# IMPLEMENTATION.md

# EPT – Expense Tracking CLI

A minimal, single-user, stateless CLI expense tracker.

---

# 🏗 Architecture Overview

## Design Principles

* Single user
* Single database table
* No in-memory state
* No monthly tables
* No stored "active month"
* All time-based logic derived from `created_at`
* Stateless command execution
* Database is the single source of truth

---

# 🗄 Database Design

## Table: `expenses`

| Column     | Type         | Description                         |
| ---------- | ------------ | ----------------------------------- |
| id         | INTEGER (PK) | Auto-increment primary key          |
| category   | TEXT         | Expense category (e.g., food, rent) |
| amount     | REAL         | Expense amount                      |
| note       | TEXT         | Optional description                |
| created_at | DATETIME     | Timestamp of expense                |

### Important Rules

* Do NOT store month
* Do NOT store week
* Do NOT store day
* Do NOT create multiple tables per month
* All date filtering must use range queries

---

# 🧠 Command List

---

## 🔹 Setup

### Initialize Database

```
ept init
```

Creates database file if it does not exist.

---

## 🔹 Adding Data

### Add Expense

```
ept add <category> <amount>
```

Example:

```
ept add food 25
```

Optional flags:

```
ept add food 25 --note "lunch"
ept add food 25 --date 2026-03-02
```

Rules:

* If `--date` is not provided, use current timestamp.
* Always store full datetime.

---

## 🔹 Viewing Data

### List All Expenses

```
ept list
```

### List By Month

```
ept list --month 2026-03
```

### List By Date

```
ept list --date 2026-03-02
```

---

# 📊 Summaries

---

## Daily Summary

```
ept summary daily
```

Or:

```
ept summary daily --date 2026-03-02
```

---

## Weekly Summary

```
ept summary weekly
```

Rules:

* Week starts on Monday
* Always compute week range dynamically

---

## Monthly Summary

```
ept summary monthly
```

Or:

```
ept summary monthly --month 2026-03
```

---

## Category Summary

```
ept summary category --month 2026-03
```

---

# ✏ Edit & Delete

### Delete Expense

```
ept delete <id>
```

### Edit Expense

```
ept edit <id>
```

---

# 🧮 Date Handling Rules (CRITICAL)

Never store:

* month
* week
* day

Always compute date ranges.

---

## Monthly Range

Input:

```
2026-03
```

Compute:

```
start = 2026-03-01 00:00:00
end   = 2026-04-01 00:00:00
```

Query:

```
WHERE created_at >= start
AND created_at < end
```

Important:
Use `< end` instead of `<=` to avoid time edge bugs.

---

## Weekly Range

Definition:

* Week starts Monday

Logic:

```
today = current_date
weekday = today.weekday()   # Monday = 0

start = today - weekday days
end = start + 7 days
```

---

## Daily Range

```
start = selected_date 00:00:00
end = selected_date + 1 day
```

Query using:

```
WHERE created_at >= start
AND created_at < end
```

---

# 🧱 Folder Structure

```
ept/
│
├── main_cli
├── database
├── expense_service
├── summary_service
└── date_utils
```

---

# ⚙ Command Execution Flow

Example:

```
ept summary monthly
```

Execution steps:

1. Parse CLI arguments
2. Call summary_service.monthly()
3. date_utils computes date range
4. Query database with range filter
5. Aggregate SUM(amount)
6. Format and print result

Each command:

* Independent
* Stateless
* No shared runtime state
* No cached data

---

# 🚫 What NOT To Do

* Do NOT create a new table every month
* Do NOT store "current month" in config
* Do NOT manually calculate month lengths
* Do NOT store week numbers
* Do NOT compare dates as strings
* Do NOT depend on in-memory state between runs

---

# 🧠 Why This Architecture Is Strong

* Stateless
* Deterministic
* Easy to test
* Easy to extend
* No schema duplication
* No technical debt from monthly tables
* Scales naturally to web app later
* Clean time-series data design

---

# 🚀 Optional Professional Feature

## Stats Command

```
ept stats
```

Suggested output:

* Total spent this month
* Most expensive category
* Average daily spending
* Highest single expense

---

# ✅ Final Principle

Expenses exist on a timeline.

All reports are filtered views of that timeline.

Time is never stored in fragments — it is always derived from `created_at`.

---

If you want, I can also generate:

* A matching `README.md`
* SQL schema file
* Test plan document
* CLI argument design spec
* Or language-specific implementation notes

Just tell me your language.
