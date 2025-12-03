# gnucash-sys API Reference

This document provides detailed API documentation for the gnucash-sys crate, which provides FFI bindings and safe Rust wrappers for the GnuCash accounting engine.

## Table of Contents

- [Initialization](#initialization)
- [Core Types](#core-types)
  - [Session](#session)
  - [Book](#book)
  - [Account](#account)
  - [Transaction](#transaction)
  - [Split](#split)
  - [Guid](#guid)
  - [Numeric](#numeric)
- [Price Database](#price-database)
  - [Price](#price)
  - [PriceDB](#pricedb)
- [Enumerations](#enumerations)
- [Constants](#constants)
- [Error Handling](#error-handling)

---

## Initialization

Before using any GnuCash functionality, the engine must be initialized.

**Source:** [`src/session.rs:17-23`](../src/session.rs)

```rust
use gnucash_sys::init_engine;

fn main() {
    init_engine();  // Safe to call multiple times
    // ... use GnuCash APIs
}
```

### Functions

| Function | Description |
|----------|-------------|
| `init_engine()` | Initialize the GnuCash engine. Must be called before any other operations. Safe to call multiple times. |
| `is_engine_initialized() -> bool` | Check if the engine has been initialized. |

**Example:** [`examples/simple_book.rs`](../examples/simple_book.rs)

---

## Core Types

### Session

A connection to a GnuCash data store (file or database).

**Source:** [`src/session.rs:30-213`](../src/session.rs)

```rust
use gnucash_sys::{Session, SessionOpenMode};

// Open existing file
let session = Session::open("/path/to/file.gnucash", SessionOpenMode::SESSION_READ_ONLY)?;

// Create new file
let session = Session::open("xml:///tmp/new.gnucash", SessionOpenMode::SESSION_NEW_STORE)?;

// Access the book
if let Some(book) = session.book() {
    // work with book
}

// Save and close
session.save()?;
session.end();
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Session::new() -> Self` | Create a new session with an empty book |
| `Session::open(uri: &str, mode: SessionOpenMode) -> Result<Self, QofBackendError>` | Open a GnuCash file or database |
| `Session::open_readonly(path: &str) -> Result<Self, QofBackendError>` | Convenience method to open read-only |

#### Instance Methods

| Method | Description |
|--------|-------------|
| `book() -> Option<Book>` | Get the book associated with this session |
| `get_error() -> QofBackendError` | Get the last error |
| `pop_error() -> QofBackendError` | Pop and return the last error |
| `get_error_message() -> Option<String>` | Get error message string |
| `file_path() -> Option<String>` | Get the file path |
| `url() -> Option<String>` | Get the session URL |
| `save() -> Result<(), QofBackendError>` | Save session data |
| `ensure_all_data_loaded()` | Ensure all data is loaded |
| `end()` | End the session (releases locks) |
| `as_ptr() -> *mut QofSession` | Get raw pointer |

#### SessionOpenMode

| Mode | Description |
|------|-------------|
| `SESSION_NORMAL_OPEN` | Normal open (read/write) |
| `SESSION_READ_ONLY` | Open read-only |
| `SESSION_NEW_STORE` | Create new file |
| `SESSION_NEW_OVERWRITE` | Create new, overwrite existing |
| `SESSION_BREAK_LOCK` | Break existing lock |

**Examples:**
- [`examples/simple_session.rs`](../examples/simple_session.rs) - Session handling
- [`examples/list_accounts.rs`](../examples/list_accounts.rs) - Opening files

---

### Book

The top-level container for all financial data.

**Source:** [`src/book.rs:1-195`](../src/book.rs)

```rust
use gnucash_sys::Book;

let book = Book::new();

// Get root account
if let Some(root) = book.root_account() {
    for child in root.children() {
        println!("Account: {:?}", child.name());
    }
}

// Check status
println!("Is dirty: {}", book.is_dirty());
println!("Transaction count: {}", book.transaction_count());
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Book::new() -> Self` | Create a new empty book |
| `unsafe Book::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Instance Methods

| Method | Description |
|--------|-------------|
| `guid() -> Guid` | Get the book's GUID |
| `is_readonly() -> bool` | Check if read-only |
| `mark_readonly()` | Mark as read-only |
| `is_empty() -> bool` | Check if empty |
| `is_dirty() -> bool` | Check for unsaved changes |
| `mark_saved()` | Mark as saved |
| `mark_dirty()` | Mark as having unsaved changes |
| `is_shutting_down() -> bool` | Check if shutting down |
| `use_trading_accounts() -> bool` | Check trading accounts setting |
| `use_split_action_for_num_field() -> bool` | Check split action setting |
| `num_days_autoreadonly() -> i32` | Get auto-readonly days |
| `uses_autoreadonly() -> bool` | Check auto-readonly setting |
| `mark_closed()` | Mark book as closed |
| `root_account() -> Option<Account>` | Get root account |
| `root_account_ptr() -> *mut Account` | Get raw root account pointer |
| `set_root_account(&Account)` | Set root account |
| `transaction_count() -> u32` | Count transactions |
| `as_ptr() -> *mut QofBook` | Get raw pointer |

**Examples:**
- [`examples/simple_book.rs`](../examples/simple_book.rs) - Creating books
- [`examples/opening_balances.rs`](../examples/opening_balances.rs) - Book with accounts

---

### Account

A ledger for tracking splits, organized in a tree hierarchy.

**Source:** [`src/account.rs:1-409`](../src/account.rs)

```rust
use gnucash_sys::{Account, Book, GNCAccountType};

let book = Book::new();
let account = Account::new(&book);

account.begin_edit();
account.set_name("Checking");
account.set_type(GNCAccountType::ACCT_TYPE_BANK);
account.set_description("Primary checking account");
account.commit_edit();

// Query balances
println!("Balance: {}", account.balance().to_f64());
println!("Reconciled: {}", account.reconciled_balance().to_f64());
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Account::new(book: &Book) -> Self` | Create new account |
| `unsafe Account::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Edit Cycle

All modifications must occur within an edit cycle:

```rust
account.begin_edit();
// ... make changes ...
account.commit_edit();
```

| Method | Description |
|--------|-------------|
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |

#### Getters

| Method | Description |
|--------|-------------|
| `guid() -> Guid` | Get GUID |
| `name() -> Option<String>` | Get name |
| `code() -> Option<String>` | Get account code |
| `description() -> Option<String>` | Get description |
| `notes() -> Option<String>` | Get notes |
| `color() -> Option<String>` | Get color |
| `account_type() -> GNCAccountType` | Get account type |
| `full_name() -> Option<String>` | Get full path name (e.g., "Assets:Bank:Checking") |
| `is_placeholder() -> bool` | Check if placeholder |
| `is_hidden() -> bool` | Check if hidden |
| `should_be_hidden() -> bool` | Check if should be hidden (includes parents) |
| `is_root() -> bool` | Check if root account |

#### Setters

| Method | Description |
|--------|-------------|
| `set_name(&str)` | Set name |
| `set_code(&str)` | Set account code |
| `set_description(&str)` | Set description |
| `set_notes(&str)` | Set notes |
| `set_color(&str)` | Set color |
| `set_type(GNCAccountType)` | Set account type |
| `set_placeholder(bool)` | Set placeholder flag |
| `set_hidden(bool)` | Set hidden flag |

#### Hierarchy

| Method | Description |
|--------|-------------|
| `parent() -> Option<Account>` | Get parent account |
| `root() -> Option<Account>` | Get root of tree |
| `n_children() -> i32` | Count immediate children |
| `n_descendants() -> i32` | Count all descendants |
| `depth() -> i32` | Get depth in tree |
| `nth_child(n: i32) -> Option<Account>` | Get nth child |
| `append_child(&Account)` | Add child account |
| `remove_child(&Account)` | Remove child account |
| `has_ancestor(&Account) -> bool` | Check ancestry |
| `lookup_by_name(&str) -> Option<Account>` | Find descendant by name |
| `lookup_by_full_name(&str) -> Option<Account>` | Find by full path |
| `lookup_by_code(&str) -> Option<Account>` | Find by code |
| `mark_unowned()` | Mark as not owned (after adding to hierarchy) |

#### Balances

| Method | Description |
|--------|-------------|
| `balance() -> Numeric` | Current balance |
| `cleared_balance() -> Numeric` | Cleared balance |
| `reconciled_balance() -> Numeric` | Reconciled balance |
| `present_balance() -> Numeric` | Present balance (excludes future) |
| `projected_minimum_balance() -> Numeric` | Projected minimum |
| `balance_as_of_date(i64) -> Numeric` | Balance at date |
| `recompute_balance()` | Recompute balance |

#### Splits & Iteration

| Method | Description |
|--------|-------------|
| `splits_size() -> usize` | Number of splits |
| `commodity_scu() -> i32` | Smallest commodity unit |
| `children() -> AccountChildren` | Iterator over children |
| `descendants() -> AccountDescendants` | Iterator over all descendants |
| `splits() -> AccountSplits` | Iterator over splits |

**Examples:**
- [`examples/simple_book.rs`](../examples/simple_book.rs) - Creating accounts
- [`examples/list_accounts.rs`](../examples/list_accounts.rs) - Listing accounts
- [`examples/account_tree.rs`](../examples/account_tree.rs) - Tree traversal
- [`examples/account_analysis.rs`](../examples/account_analysis.rs) - Account analysis
- [`examples/opening_balances.rs`](../examples/opening_balances.rs) - Account creation

---

### Transaction

A double-entry accounting record containing splits that must balance to zero.

**Source:** [`src/transaction.rs:1-415`](../src/transaction.rs)

```rust
use gnucash_sys::{Transaction, Split, Book, Numeric};

let book = Book::new();
let txn = Transaction::new(&book);

txn.begin_edit();
txn.set_description("Grocery shopping");
txn.set_date(15, 3, 2024);  // March 15, 2024

// Add splits...
let split = Split::new(&book);
split.set_account(&checking);
split.set_transaction(&txn);
split.set_value(Numeric::new(-5000, 100));  // -$50.00

txn.commit_edit();
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Transaction::new(book: &Book) -> Self` | Create new transaction |
| `unsafe Transaction::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Edit Cycle

| Method | Description |
|--------|-------------|
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |
| `rollback_edit()` | Rollback changes |
| `is_open() -> bool` | Check if open for editing |

#### Getters

| Method | Description |
|--------|-------------|
| `guid() -> Guid` | Get GUID |
| `description() -> Option<String>` | Get description |
| `num() -> Option<String>` | Get transaction number |
| `notes() -> Option<String>` | Get notes |
| `doc_link() -> Option<String>` | Get document link URL |
| `txn_type() -> char` | Get type (NONE, INVOICE, PAYMENT, LINK) |
| `is_closing() -> bool` | Check if closing transaction |
| `is_void() -> bool` | Check if voided |
| `void_reason() -> Option<String>` | Get void reason |
| `read_only_reason() -> Option<String>` | Get read-only reason |
| `is_readonly_by_posted_date() -> bool` | Check if read-only by date |

#### Setters

| Method | Description |
|--------|-------------|
| `set_description(&str)` | Set description |
| `set_num(&str)` | Set transaction number |
| `set_notes(&str)` | Set notes |
| `set_doc_link(&str)` | Set document link |
| `set_txn_type(char)` | Set transaction type |
| `set_is_closing(bool)` | Set closing flag |
| `set_read_only(&str)` | Set read-only with reason |
| `clear_read_only()` | Clear read-only flag |

#### Dates

| Method | Description |
|--------|-------------|
| `date_posted() -> i64` | Get posted date (Unix timestamp) |
| `date_entered() -> i64` | Get entered date |
| `date_due() -> i64` | Get due date |
| `void_time() -> i64` | Get void time |
| `set_date(day, month, year)` | Set posted date |
| `set_date_posted(i64)` | Set posted date (timestamp) |
| `set_date_entered(i64)` | Set entered date |
| `set_date_due(i64)` | Set due date |

#### Splits

| Method | Description |
|--------|-------------|
| `split_count() -> i32` | Count splits |
| `get_split(index: i32) -> Option<*mut Split>` | Get split by index |
| `get_split_index(*const Split) -> i32` | Get split's index |
| `sort_splits()` | Sort splits (debits first) |
| `clear_splits()` | Remove all splits |
| `splits() -> TransactionSplits` | Iterator over splits |

#### Balance

| Method | Description |
|--------|-------------|
| `imbalance_value() -> Numeric` | Get imbalance (should be zero) |
| `is_balanced() -> bool` | Check if balanced |
| `account_value(*const Account) -> Numeric` | Total value for account |
| `account_amount(*const Account) -> Numeric` | Total amount for account |

#### Voiding

| Method | Description |
|--------|-------------|
| `void(&str)` | Void with reason |
| `unvoid()` | Unvoid transaction |
| `reverse() -> Option<Transaction>` | Create reversing transaction |
| `reversed_by() -> Option<Transaction>` | Get reversing transaction |

#### Miscellaneous

| Method | Description |
|--------|-------------|
| `use_trading_accounts() -> bool` | Check trading accounts |
| `has_reconciled_splits() -> bool` | Check for reconciled splits |

**Examples:**
- [`examples/create_transaction.rs`](../examples/create_transaction.rs) - Creating transactions
- [`examples/opening_balances.rs`](../examples/opening_balances.rs) - Opening balance transactions
- [`examples/account_analysis.rs`](../examples/account_analysis.rs) - Transaction iteration

---

### Split

A single entry in a transaction, linking an amount to an account.

**Source:** [`src/split.rs:1-373`](../src/split.rs)

```rust
use gnucash_sys::{Split, Book, Numeric};

let split = Split::new(&book);
split.set_account(&account);
split.set_transaction(&txn);
split.set_memo("Groceries at Store");
split.set_amount(Numeric::new(-5000, 100));
split.set_value(Numeric::new(-5000, 100));
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Split::new(book: &Book) -> Self` | Create new split |
| `unsafe Split::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Linkage

| Method | Description |
|--------|-------------|
| `account() -> Option<Account>` | Get account |
| `set_account(&Account)` | Set account |
| `transaction() -> Option<Transaction>` | Get parent transaction |
| `set_transaction(&Transaction)` | Set parent transaction |
| `book() -> Option<Book>` | Get book |
| `reinit()` | Reinitialize to defaults |

#### Memo/Action

| Method | Description |
|--------|-------------|
| `memo() -> Option<String>` | Get memo |
| `set_memo(&str)` | Set memo |
| `action() -> Option<String>` | Get action (Buy, Sell, etc.) |
| `set_action(&str)` | Set action |
| `split_type() -> Option<String>` | Get type (normal/stock-split) |
| `make_stock_split()` | Mark as stock split |

#### Amount/Value

| Method | Description |
|--------|-------------|
| `amount() -> Numeric` | Amount in account's commodity |
| `set_amount(Numeric)` | Set amount |
| `value() -> Numeric` | Value in transaction's currency |
| `set_value(Numeric)` | Set value |
| `share_price() -> Numeric` | Get share price (value/amount) |
| `set_share_price_and_amount(price, amount)` | Set both |

#### Balances

| Method | Description |
|--------|-------------|
| `balance() -> Numeric` | Running balance |
| `noclosing_balance() -> Numeric` | Balance excluding closings |
| `cleared_balance() -> Numeric` | Cleared balance |
| `reconciled_balance() -> Numeric` | Reconciled balance |

#### Reconciliation

| Method | Description |
|--------|-------------|
| `reconcile_state() -> char` | Get state ('n', 'c', 'y', 'f', 'v') |
| `set_reconcile_state(char)` | Set state |
| `date_reconciled() -> i64` | Get reconcile date |
| `set_date_reconciled(i64)` | Set reconcile date |
| `is_reconciled() -> bool` | Check if reconciled |
| `is_cleared() -> bool` | Check if cleared |

#### Other Split

| Method | Description |
|--------|-------------|
| `other_split() -> Option<Split>` | Get other split (2-split txn only) |
| `corr_account_full_name() -> Option<String>` | Corresponding account name |
| `corr_account_name() -> Option<String>` | Corresponding account short name |
| `corr_account_code() -> Option<String>` | Corresponding account code |

#### Voiding

| Method | Description |
|--------|-------------|
| `void_former_amount() -> Numeric` | Original amount before void |
| `void_former_value() -> Numeric` | Original value before void |

#### Peer Splits

| Method | Description |
|--------|-------------|
| `has_peers() -> bool` | Check for peer splits |
| `is_peer(&Split) -> bool` | Check if peer |
| `add_peer(&Split, timestamp)` | Add peer |
| `remove_peer(&Split)` | Remove peer |

**Examples:**
- [`examples/create_transaction.rs`](../examples/create_transaction.rs) - Creating splits
- [`examples/account_analysis.rs`](../examples/account_analysis.rs) - Split iteration
- [`examples/export_csv.rs`](../examples/export_csv.rs) - Reading split data
- [`examples/reconcile_account.rs`](../examples/reconcile_account.rs) - Reconciliation states

---

### Guid

A 128-bit globally unique identifier.

**Source:** [`src/types.rs:11-99`](../src/types.rs)

```rust
use gnucash_sys::Guid;

let guid = Guid::new();  // Random GUID
let parsed = Guid::parse("0123456789abcdef0123456789abcdef");
println!("GUID: {}", guid);  // Prints hex string
```

#### Methods

| Method | Description |
|--------|-------------|
| `Guid::new() -> Self` | Create random GUID |
| `Guid::from_bytes([u8; 16]) -> Self` | Create from bytes |
| `Guid::null() -> &'static Self` | Get null GUID reference |
| `Guid::parse(&str) -> Option<Self>` | Parse from 32-char hex string |
| `as_bytes() -> &[u8; 16]` | Get raw bytes |
| `is_null() -> bool` | Check if null |
| `as_ffi() -> &GncGUID` | Get FFI reference |

#### Traits

- `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`
- `Default` (creates new random GUID)
- `Debug`, `Display` (hex format)
- `From<GncGUID>`, `Into<GncGUID>`
- `Serialize`, `Deserialize` (with `serde` feature)

---

### Numeric

A rational number with 64-bit numerator and denominator for precise decimal arithmetic.

**Source:** [`src/types.rs:101-270`](../src/types.rs)

```rust
use gnucash_sys::Numeric;

let amount = Numeric::new(10050, 100);  // $100.50
let zero = Numeric::zero();

println!("Amount: ${:.2}", amount.to_f64());
println!("Is negative: {}", amount.is_negative());

let negated = amount.neg();
let absolute = negated.abs();
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Numeric::new(num: i64, denom: i64) -> Self` | Create from num/denom |
| `Numeric::zero() -> Self` | Create zero (0/1) |

#### Instance Methods

| Method | Description |
|--------|-------------|
| `num() -> i64` | Get numerator |
| `denom() -> i64` | Get denominator |
| `is_zero() -> bool` | Check if zero |
| `is_negative() -> bool` | Check if negative |
| `is_positive() -> bool` | Check if positive |
| `to_f64() -> f64` | Convert to float |
| `neg() -> Self` | Negate |
| `abs() -> Self` | Absolute value |

#### Traits

- `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`
- `Default` (returns zero)
- `Debug`, `Display`
- `From<i64>`, `From<gnc_numeric>`, `Into<gnc_numeric>`
- `Neg` (unary minus operator)
- `Serialize`, `Deserialize` (with `serde` feature)

**Examples:**
- [`examples/create_transaction.rs`](../examples/create_transaction.rs) - Using Numeric for amounts
- [`examples/account_analysis.rs`](../examples/account_analysis.rs) - Numeric arithmetic

---

## Price Database

### Price

A price quote for a commodity at a specific time.

**Source:** [`src/price.rs:1-175`](../src/price.rs)

```rust
use gnucash_sys::{Price, Book, Numeric, PriceSource};

let price = Price::new(&book);
price.begin_edit();
price.set_time(1704067200);  // Unix timestamp
price.set_source(PriceSource::PRICE_SOURCE_USER_PRICE);
price.set_type_string("last");
price.set_value(Numeric::new(150, 1));  // $150
price.commit_edit();
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Price::new(book: &Book) -> Self` | Create new price |
| `unsafe Price::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Edit Cycle

| Method | Description |
|--------|-------------|
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |

#### Reference Counting

| Method | Description |
|--------|-------------|
| `ref_()` | Increment reference count |
| `unref()` | Decrement reference count |

#### Methods

| Method | Description |
|--------|-------------|
| `clone_in_book(&Book) -> Option<Price>` | Clone to another book |
| `invert() -> Option<Price>` | Create inverted price (1/price) |
| `time() -> i64` | Get timestamp |
| `set_time(i64)` | Set timestamp |
| `source() -> PriceSource` | Get source |
| `set_source(PriceSource)` | Set source |
| `source_string() -> Option<String>` | Get source as string |
| `set_source_string(&str)` | Set source from string |
| `type_string() -> Option<String>` | Get type string |
| `set_type_string(&str)` | Set type string |
| `value() -> Numeric` | Get price value |
| `set_value(Numeric)` | Set price value |

#### Traits

- `PartialEq`, `Eq` (compares all fields)
- `Debug`

**Example:** [`examples/price_database.rs`](../examples/price_database.rs)

---

### PriceDB

A database of price quotes.

**Source:** [`src/price.rs:176-252`](../src/price.rs)

```rust
use gnucash_sys::{PriceDB, Book};

let pricedb = PriceDB::get_db(&book).expect("No price database");

pricedb.add_price(&price);
pricedb.remove_price(&price);
```

#### Methods

| Method | Description |
|--------|-------------|
| `PriceDB::get_db(book: &Book) -> Option<Self>` | Get database for book |
| `unsafe PriceDB::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |
| `set_bulk_update(bool)` | Set bulk update mode |
| `add_price(&Price) -> bool` | Add price |
| `remove_price(&Price) -> bool` | Remove price |
| `PriceDB::lookup_by_guid(&Guid, &Book) -> Option<Price>` | Find by GUID |

**Example:** [`examples/price_database.rs`](../examples/price_database.rs)

---

## Enumerations

### GNCAccountType

**Source:** [`src/account.rs:11`](../src/account.rs) (re-exported from FFI)

| Variant | Description |
|---------|-------------|
| `ACCT_TYPE_BANK` | Bank account |
| `ACCT_TYPE_CASH` | Cash |
| `ACCT_TYPE_ASSET` | Asset |
| `ACCT_TYPE_CREDIT` | Credit card |
| `ACCT_TYPE_LIABILITY` | Liability |
| `ACCT_TYPE_STOCK` | Stock |
| `ACCT_TYPE_MUTUAL` | Mutual fund |
| `ACCT_TYPE_INCOME` | Income |
| `ACCT_TYPE_EXPENSE` | Expense |
| `ACCT_TYPE_EQUITY` | Equity |
| `ACCT_TYPE_RECEIVABLE` | Accounts receivable |
| `ACCT_TYPE_PAYABLE` | Accounts payable |
| `ACCT_TYPE_ROOT` | Root account |
| `ACCT_TYPE_TRADING` | Trading account |

### PriceSource

**Source:** [`src/price.rs:10`](../src/price.rs)

| Variant | Description |
|---------|-------------|
| `PRICE_SOURCE_EDIT_DLG` | User edited |
| `PRICE_SOURCE_FQ` | Finance::Quote |
| `PRICE_SOURCE_USER_PRICE` | User entered |
| `PRICE_SOURCE_XFER_DLG_VAL` | Transfer dialog |
| `PRICE_SOURCE_SPLIT_REG` | Split register |
| `PRICE_SOURCE_SPLIT_IMPORT` | Imported |
| `PRICE_SOURCE_STOCK_SPLIT` | Stock split |
| `PRICE_SOURCE_TEMP` | Temporary |
| `PRICE_SOURCE_INVALID` | Invalid |

### QofBackendError

**Source:** [`src/session.rs:11`](../src/session.rs)

Common error codes for session operations:

| Variant | Description |
|---------|-------------|
| `ERR_BACKEND_NO_ERR` | No error |
| `ERR_BACKEND_NO_HANDLER` | No handler |
| `ERR_BACKEND_NO_BACKEND` | No backend |
| `ERR_BACKEND_BAD_URL` | Bad URL |
| `ERR_BACKEND_LOCKED` | File locked |
| `ERR_BACKEND_READONLY` | Read-only |
| `ERR_BACKEND_TOO_NEW` | File too new |
| `ERR_FILEIO_FILE_NOT_FOUND` | File not found |

---

## Constants

### Reconcile States

**Source:** [`src/split.rs:10-21`](../src/split.rs)

```rust
use gnucash_sys::reconcile;

reconcile::NOT_RECONCILED  // 'n' - Not reconciled
reconcile::CLEARED         // 'c' - Cleared
reconcile::RECONCILED      // 'y' - Reconciled
reconcile::FROZEN          // 'f' - Frozen
reconcile::VOIDED          // 'v' - Voided
```

### Transaction Types

**Source:** [`src/transaction.rs:11-16`](../src/transaction.rs)

```rust
use gnucash_sys::txn_type;

txn_type::NONE     // '\0' - Normal transaction
txn_type::INVOICE  // 'I' - Invoice
txn_type::PAYMENT  // 'P' - Payment
txn_type::LINK     // 'L' - Link
```

### GUID Constants

**Source:** [`src/types.rs:11`](../src/types.rs)

```rust
use gnucash_sys::GUID_ENCODING_LENGTH;  // 32 (hex characters)
```

---

## Error Handling

The crate uses `Result` types for fallible operations.

**Source:** [`src/error.rs`](../src/error.rs)

```rust
use gnucash_sys::{Result, Error, Session, SessionOpenMode};

fn open_file(path: &str) -> Result<Session> {
    Session::open(path, SessionOpenMode::SESSION_READ_ONLY)
        .map_err(|e| Error::Backend(e))
}

match open_file("/path/to/file.gnucash") {
    Ok(session) => { /* work with session */ }
    Err(e) => eprintln!("Failed: {:?}", e),
}
```

---

## Iterators

The crate provides iterators for traversing GnuCash collections.

**Source:** [`src/iter.rs`](../src/iter.rs)

| Iterator | Source | Description |
|----------|--------|-------------|
| `AccountChildren` | `account.children()` | Immediate children |
| `AccountDescendants` | `account.descendants()` | All descendants (depth-first) |
| `AccountSplits` | `account.splits()` | Splits in account |
| `TransactionSplits` | `transaction.splits()` | Splits in transaction |

```rust
// Iterate over account children
for child in account.children() {
    println!("Child: {:?}", child.name());
}

// Iterate over all descendants
for desc in account.descendants() {
    println!("Descendant: {:?}", desc.full_name());
}

// Iterate over splits
for split in account.splits() {
    println!("Split: {:?}", split.value());
}
```

**Examples:**
- [`examples/account_tree.rs`](../examples/account_tree.rs) - Tree traversal
- [`examples/account_analysis.rs`](../examples/account_analysis.rs) - Split iteration
- [`examples/search_transactions.rs`](../examples/search_transactions.rs) - Transaction search

---

## FFI Module

For advanced use cases, raw FFI bindings are available in the `ffi` module.

**Source:** [`src/lib.rs:96-101`](../src/lib.rs)

```rust
use gnucash_sys::ffi;

// Access raw C functions
unsafe {
    let book = ffi::qof_book_new();
    // ...
    ffi::qof_book_destroy(book);
}
```

**Warning:** The FFI module is unsafe and requires careful memory management. Prefer the safe wrapper types when possible.

---

## See Also

- [Examples README](../examples/README.md) - Example programs
- [gnucash-ext](../app/) - Extended functionality (business entities, queries, builders)
