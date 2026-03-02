# ept — Expense Tracker CLI

A minimal, stateless command-line expense tracker built in Rust.

## Installation

```bash
cargo install --path .
```

Or build and run directly:

```bash
cargo build --release
./target/release/ept
```

## Setup

Run once to create the database at `~/.ept/expenses.db`:

```bash
ept init
```

## Commands

### Add an expense

```bash
ept add <category> <amount>
ept add food 25
ept add food 25 --note "lunch"
ept add food 25 --date 2026-03-02
```

### List expenses

```bash
ept list                        # all expenses
ept list --month 2026-03        # filter by month
ept list --date 2026-03-02      # filter by date
```

### Summaries

```bash
ept summary daily               # today's total
ept summary daily --date 2026-03-02
ept summary weekly              # current week (Mon–Sun)
ept summary monthly             # current month total
ept summary monthly --month 2026-03
ept summary category            # breakdown by category (current month)
ept summary category --month 2026-03
```

### Stats

```bash
ept stats                       # total, avg daily, top category, highest expense
```

### Edit & Delete

```bash
ept edit <id> --amount 30
ept edit <id> --category food --note "updated"
ept edit <id> --date 2026-03-01
ept delete <id>
```

## Database

- Location: `~/.ept/expenses.db` (SQLite)
- Created by `ept init`
- Single table `expenses` with columns: `id`, `category`, `amount`, `note`, `created_at`
- All date filtering uses range queries on `created_at` — no month/week columns stored
