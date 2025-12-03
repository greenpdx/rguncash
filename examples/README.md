# gnucash-sys Examples

This directory contains example programs demonstrating how to use the gnucash-sys Rust bindings for libgnucash.

## Prerequisites

- GnuCash development libraries installed (`libgnucash-dev` or equivalent)
- Rust toolchain

## Building Examples

Build all examples:
```bash
cargo build --examples
```

Build a specific example:
```bash
cargo build --example simple_book
```

Run an example:
```bash
cargo run --example simple_book
```

## Examples Overview

### Basic Operations

| Example | Description |
|---------|-------------|
| `simple_book` | Create a new book with a basic account hierarchy |
| `simple_session` | Open existing files or create new ones using the Session API |

### Account Operations

| Example | Description |
|---------|-------------|
| `list_accounts` | List all accounts from an existing GnuCash file |
| `account_tree` | Display the complete account hierarchy as a tree with statistics |
| `account_analysis` | Analyze transactions in a specific account showing debits/credits |

### Transaction Operations

| Example | Description |
|---------|-------------|
| `create_transaction` | Create a balanced transaction with multiple splits |
| `opening_balances` | Set up accounts with opening balance transactions |
| `search_transactions` | Search transactions by description or memo |

### Reporting

| Example | Description |
|---------|-------------|
| `export_csv` | Export account transactions to CSV format |
| `balance_sheet` | Generate a balance sheet report (Assets, Liabilities, Equity) |
| `reconcile_account` | View reconciliation status of an account |

### Price Database

| Example | Description |
|---------|-------------|
| `price_database` | Work with the price database (add, query, remove prices) |

## Example Details

### simple_book

Creates a new in-memory book with a basic chart of accounts (Assets, Liabilities, Equity, Income, Expenses).

```bash
cargo run --example simple_book
```

### simple_session

Demonstrates the Session API for opening existing files or creating new ones.

```bash
# Open existing file
cargo run --example simple_session /path/to/file.gnucash

# Create new file
cargo run --example simple_session xml:///tmp/new_file.gnucash
```

### list_accounts

Lists all accounts from a GnuCash file with their types and balances.

```bash
cargo run --example list_accounts /path/to/file.gnucash
```

### account_tree

Displays the account hierarchy as an indented tree and shows statistics.

```bash
cargo run --example account_tree /path/to/file.gnucash
```

### account_analysis

Analyzes a specific account showing all transactions with running balances.

```bash
cargo run --example account_analysis /path/to/file.gnucash "Assets:Current Assets:Checking"
```

### create_transaction

Creates a sample transaction with splits between accounts.

```bash
cargo run --example create_transaction
```

### opening_balances

Demonstrates creating a chart of accounts with opening balance transactions.

```bash
cargo run --example opening_balances
```

### search_transactions

Searches for transactions matching a search term, or lists recent transactions.

```bash
# List recent transactions
cargo run --example search_transactions /path/to/file.gnucash

# Search for specific term
cargo run --example search_transactions /path/to/file.gnucash "grocery"
```

### export_csv

Exports transactions from a specific account to CSV format.

```bash
# Export to stdout
cargo run --example export_csv /path/to/file.gnucash "Assets:Checking"

# Export to file
cargo run --example export_csv /path/to/file.gnucash "Assets:Checking" output.csv
```

### balance_sheet

Generates a balance sheet report showing assets, liabilities, and equity.

```bash
cargo run --example balance_sheet /path/to/file.gnucash
```

### reconcile_account

Shows the reconciliation status of an account (unreconciled, cleared, reconciled transactions).

```bash
cargo run --example reconcile_account /path/to/file.gnucash "Assets:Checking"
```

### price_database

Demonstrates price database operations for tracking currency/stock prices.

```bash
cargo run --example price_database
```

## Account Path Format

Several examples accept an account path argument. Account paths use colons (`:`) as separators:

- `Assets:Checking`
- `Assets:Current Assets:Savings`
- `Expenses:Food:Groceries`

## File Format Support

GnuCash supports multiple file backends. Use URI prefixes for specific formats:

- `xml:///path/to/file.gnucash` - XML format
- `sqlite3:///path/to/file.gnucash` - SQLite format
- Plain path `/path/to/file.gnucash` - Auto-detect format

## Notes

- Examples that modify data create in-memory books and don't save to disk unless explicitly shown
- Read-only examples open files with `SESSION_READ_ONLY` mode
- Always call `session.end()` when done with a session
- The GnuCash engine must be initialized with `init_engine()` before use

## Business Examples

Business entity examples (customers, vendors, invoices) are in the `gnucash-ext` crate:

```bash
# Run business example from gnucash-ext
cargo run -p gnucash-ext --example simple_business
```

See [`app/examples/`](../app/examples/) for business-related examples.

## API Documentation

See [`docs/API.md`](../docs/API.md) for detailed API documentation.
