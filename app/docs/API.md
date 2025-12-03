# gnucash-ext API Reference

This document provides detailed API documentation for the gnucash-ext crate, which provides extended functionality for gnucash-sys including business entities, queries, and builders.

## Table of Contents

- [Overview](#overview)
- [Business Module](#business-module)
  - [Customer](#customer)
  - [Vendor](#vendor)
  - [Employee](#employee)
  - [Job](#job)
  - [Invoice](#invoice)
  - [Entry](#entry)
  - [Owner](#owner)
  - [Address](#address)
  - [BillTerm](#billterm)
  - [TaxTable](#taxtable)
- [Price Module](#price-module)
  - [Price](#price)
  - [PriceDB](#pricedb)
- [Query Module](#query-module)
  - [Query](#query)
- [Builder Module](#builder-module)
  - [TransactionBuilder](#transactionbuilder)
  - [InvoiceBuilder](#invoicebuilder)
- [Re-exports](#re-exports)

---

## Overview

The gnucash-ext crate extends gnucash-sys with:

- **Business entities** - Customer, Vendor, Employee, Job, Invoice, Entry
- **Price database** - Enhanced price handling
- **Query framework** - QofQuery wrapper for searching
- **Builders** - Fluent APIs for creating complex entities

**Source:** [`src/lib.rs`](../src/lib.rs)

```rust
use gnucash_ext::{
    // Core types (re-exported from gnucash-sys)
    init_engine, Book, Account, Transaction, Split, Numeric, Guid,
    Session, SessionOpenMode,

    // Business entities
    Customer, Vendor, Employee, Job, Invoice, Entry, Owner, Address,
    BillTerm, TaxTable, TaxTableEntry,

    // Price
    Price, PriceDB, PriceSource,

    // Query
    Query, QofQueryOp, obj_types, params,

    // Builders
    TransactionBuilder, InvoiceBuilder,
};
```

---

## Business Module

The business module provides safe wrappers for GnuCash's invoicing and customer management features.

**Source:** [`src/business/mod.rs`](../src/business/mod.rs)

### Customer

A customer who receives invoices.

**Source:** [`src/business/customer.rs`](../src/business/customer.rs)

```rust
use gnucash_ext::{Book, Customer, Numeric};

let customer = Customer::new(&book);
customer.begin_edit();
customer.set_id("CUST001");
customer.set_name("Acme Corporation");
customer.set_notes("Our best customer");
customer.set_active(true);
customer.set_discount(Numeric::new(10, 100));  // 10%
customer.set_credit(Numeric::new(100000, 100)); // $1000 credit limit
customer.commit_edit();

// Get address
if let Some(addr) = customer.addr() {
    addr.set_name("Acme Corp");
    addr.set_addr1("123 Main St");
}

// Convert to Owner for invoicing
let owner = customer.to_owner();
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Customer::new(book: &Book) -> Self` | Create new customer |
| `unsafe Customer::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Edit Cycle

| Method | Description |
|--------|-------------|
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |

#### Getters

| Method | Description |
|--------|-------------|
| `guid() -> Guid` | Get GUID |
| `id() -> Option<String>` | Get customer ID |
| `name() -> Option<String>` | Get name |
| `notes() -> Option<String>` | Get notes |
| `addr() -> Option<Address>` | Get billing address |
| `ship_addr() -> Option<Address>` | Get shipping address |
| `discount() -> Numeric` | Get discount percentage |
| `credit() -> Numeric` | Get credit limit |
| `is_active() -> bool` | Check if active |

#### Setters

| Method | Description |
|--------|-------------|
| `set_id(&str)` | Set customer ID |
| `set_name(&str)` | Set name |
| `set_notes(&str)` | Set notes |
| `set_discount(Numeric)` | Set discount |
| `set_credit(Numeric)` | Set credit limit |
| `set_active(bool)` | Set active flag |

#### Conversion

| Method | Description |
|--------|-------------|
| `to_owner() -> Owner` | Convert to Owner for invoicing |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Vendor

A vendor/supplier who sends bills.

**Source:** [`src/business/vendor.rs`](../src/business/vendor.rs)

```rust
use gnucash_ext::{Book, Vendor};

let vendor = Vendor::new(&book);
vendor.begin_edit();
vendor.set_id("VEND001");
vendor.set_name("Office Supplies Inc");
vendor.set_notes("Primary office supplies vendor");
vendor.set_active(true);
vendor.commit_edit();

let owner = vendor.to_owner();
```

#### Methods

| Method | Description |
|--------|-------------|
| `Vendor::new(book: &Book) -> Self` | Create new vendor |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `id() -> Option<String>` | Get vendor ID |
| `name() -> Option<String>` | Get name |
| `notes() -> Option<String>` | Get notes |
| `addr() -> Option<Address>` | Get address |
| `is_active() -> bool` | Check if active |
| `set_id(&str)` | Set vendor ID |
| `set_name(&str)` | Set name |
| `set_notes(&str)` | Set notes |
| `set_active(bool)` | Set active flag |
| `to_owner() -> Owner` | Convert to Owner |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Employee

An employee who can submit expense vouchers.

**Source:** [`src/business/employee.rs`](../src/business/employee.rs)

```rust
use gnucash_ext::{Book, Employee, Numeric};

let employee = Employee::new(&book);
employee.begin_edit();
employee.set_id("EMP001");
employee.set_username("jsmith");
employee.set_language("en_US");
employee.set_workday(Numeric::new(8, 1));      // 8 hours/day
employee.set_rate(Numeric::new(5000, 100));    // $50.00/hour
employee.set_active(true);
employee.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `Employee::new(book: &Book) -> Self` | Create new employee |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `id() -> Option<String>` | Get employee ID |
| `username() -> Option<String>` | Get username |
| `language() -> Option<String>` | Get language preference |
| `addr() -> Option<Address>` | Get address |
| `workday() -> Numeric` | Get hours per workday |
| `rate() -> Numeric` | Get hourly rate |
| `is_active() -> bool` | Check if active |
| `set_id(&str)` | Set employee ID |
| `set_username(&str)` | Set username |
| `set_language(&str)` | Set language |
| `set_workday(Numeric)` | Set hours per day |
| `set_rate(Numeric)` | Set hourly rate |
| `set_active(bool)` | Set active flag |
| `to_owner() -> Owner` | Convert to Owner |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Job

A job/project linked to a customer or vendor.

**Source:** [`src/business/job.rs`](../src/business/job.rs)

```rust
use gnucash_ext::{Book, Job, Customer};

let customer = Customer::new(&book);
// ... set up customer ...

let job = Job::new(&book);
job.begin_edit();
job.set_id("JOB001");
job.set_name("Website Redesign");
job.set_reference("Project #2024-001");
job.set_owner(&customer.to_owner());
job.set_active(true);
job.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `Job::new(book: &Book) -> Self` | Create new job |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `id() -> Option<String>` | Get job ID |
| `name() -> Option<String>` | Get name |
| `reference() -> Option<String>` | Get reference number |
| `owner() -> Owner` | Get owning customer/vendor |
| `is_active() -> bool` | Check if active |
| `set_id(&str)` | Set job ID |
| `set_name(&str)` | Set name |
| `set_reference(&str)` | Set reference |
| `set_owner(&Owner)` | Set owning entity |
| `set_active(bool)` | Set active flag |
| `to_owner() -> Owner` | Convert to Owner |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Invoice

An invoice, bill, or expense voucher.

**Source:** [`src/business/invoice.rs`](../src/business/invoice.rs)

```rust
use gnucash_ext::{Book, Invoice, Customer, Entry, Numeric};

let customer = Customer::new(&book);
// ... set up customer ...

let invoice = Invoice::new(&book);
invoice.begin_edit();
invoice.set_id("INV-001");
invoice.set_owner(&customer.to_owner());
invoice.set_notes("Consulting services");
invoice.set_date_opened(1704067200);  // Unix timestamp
invoice.commit_edit();

// Add entries
let entry = Entry::new(&book);
entry.set_description("Consulting - Day 1");
entry.set_quantity(Numeric::new(8, 1));
entry.set_inv_price(Numeric::new(15000, 100));
invoice.add_entry(&entry);

// Check totals
println!("Total: {}", invoice.total());
println!("Is posted: {}", invoice.is_posted());
println!("Is paid: {}", invoice.is_paid());
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Invoice::new(book: &Book) -> Self` | Create new invoice |
| `Invoice::copy(other: &Invoice) -> Self` | Copy invoice |
| `unsafe Invoice::from_raw(ptr, owned) -> Option<Self>` | Create from raw pointer |

#### Edit Cycle

| Method | Description |
|--------|-------------|
| `begin_edit()` | Begin edit session |
| `commit_edit()` | Commit changes |

#### Getters

| Method | Description |
|--------|-------------|
| `guid() -> Guid` | Get GUID |
| `id() -> Option<String>` | Get invoice ID |
| `owner() -> Option<Owner>` | Get owner |
| `date_opened() -> i64` | Get date opened |
| `date_posted() -> i64` | Get date posted |
| `date_due() -> i64` | Get due date |
| `billing_id() -> Option<String>` | Get billing ID |
| `notes() -> Option<String>` | Get notes |
| `doc_link() -> Option<String>` | Get document link |
| `terms() -> Option<BillTerm>` | Get payment terms |
| `total() -> Numeric` | Get total amount |
| `total_subtotal() -> Numeric` | Get subtotal (before tax) |
| `total_tax() -> Numeric` | Get tax amount |
| `is_posted() -> bool` | Check if posted |
| `is_paid() -> bool` | Check if paid |
| `is_credit_note() -> bool` | Check if credit note |

#### Setters

| Method | Description |
|--------|-------------|
| `set_id(&str)` | Set invoice ID |
| `set_owner(&Owner)` | Set owner |
| `set_date_opened(i64)` | Set date opened |
| `set_date_posted(i64)` | Set date posted |
| `set_billing_id(&str)` | Set billing ID |
| `set_notes(&str)` | Set notes |
| `set_doc_link(&str)` | Set document link |
| `set_terms(&BillTerm)` | Set payment terms |
| `set_is_credit_note(bool)` | Set credit note flag |
| `set_bill_to(&Owner)` | Set bill-to owner |
| `set_to_charge_amount(Numeric)` | Set charge amount |

#### Entry Management

| Method | Description |
|--------|-------------|
| `add_entry(&Entry)` | Add entry to invoice |
| `remove_entry(&Entry)` | Remove entry |
| `sort_entries()` | Sort entries |
| `remove_entries()` | Remove all entries |

#### Posting

| Method | Description |
|--------|-------------|
| `post_to_account(&Account, posted_date, due_date, memo, accumulate, autopay) -> Option<Transaction>` | Post invoice |
| `unpost(reset_tax_tables: bool) -> bool` | Unpost invoice |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Entry

A line item in an invoice or bill.

**Source:** [`src/business/entry.rs`](../src/business/entry.rs)

```rust
use gnucash_ext::{Book, Entry, Numeric, Account};

let entry = Entry::new(&book);
entry.begin_edit();
entry.set_date(1704067200);
entry.set_description("Consulting services");
entry.set_quantity(Numeric::new(8, 1));         // 8 units
entry.set_inv_price(Numeric::new(15000, 100));  // $150.00/unit
entry.set_inv_account(&income_account);
entry.set_inv_taxable(true);
entry.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `Entry::new(book: &Book) -> Self` | Create new entry |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `date() -> i64` | Get entry date |
| `date_entered() -> i64` | Get date entered |
| `description() -> Option<String>` | Get description |
| `action() -> Option<String>` | Get action |
| `notes() -> Option<String>` | Get notes |
| `quantity() -> Numeric` | Get quantity |
| `inv_price() -> Numeric` | Get invoice price |
| `inv_discount() -> Numeric` | Get invoice discount |
| `inv_account() -> Option<Account>` | Get invoice account |
| `bill_price() -> Numeric` | Get bill price |
| `bill_account() -> Option<Account>` | Get bill account |
| `invoice() -> Option<Invoice>` | Get parent invoice |
| `bill() -> Option<Invoice>` | Get parent bill |
| `set_date(i64)` | Set entry date |
| `set_description(&str)` | Set description |
| `set_action(&str)` | Set action |
| `set_notes(&str)` | Set notes |
| `set_quantity(Numeric)` | Set quantity |
| `set_inv_price(Numeric)` | Set invoice price |
| `set_inv_discount(Numeric)` | Set invoice discount |
| `set_inv_account(&Account)` | Set invoice account |
| `set_bill_price(Numeric)` | Set bill price |
| `set_bill_account(&Account)` | Set bill account |
| `inv_tax_table() -> Option<TaxTable>` | Get tax table |
| `set_inv_tax_table(&TaxTable)` | Set tax table |
| `inv_tax_included() -> bool` | Check if tax included |
| `set_inv_tax_included(bool)` | Set tax included |
| `inv_taxable() -> bool` | Check if taxable |
| `set_inv_taxable(bool)` | Set taxable |

**Example:** [`examples/simple_business.rs`](../examples/simple_business.rs)

---

### Owner

A polymorphic type representing Customer, Vendor, Employee, or Job.

**Source:** [`src/business/owner.rs`](../src/business/owner.rs)

```rust
use gnucash_ext::{Owner, Customer, OwnerType};

// Create from entity
let customer = Customer::new(&book);
let owner = customer.to_owner();

// Check type
if owner.is_customer() {
    println!("Owner is a customer");
}

// Get info
println!("Type: {:?}", owner.owner_type());
println!("Name: {:?}", owner.name());
println!("GUID: {:?}", owner.guid());
```

#### Methods

| Method | Description |
|--------|-------------|
| `Owner::new() -> Self` | Create undefined owner |
| `Owner::from_raw(GncOwner) -> Self` | Create from raw |
| `as_ptr() -> *mut GncOwner` | Get pointer |
| `as_mut_ptr() -> *mut GncOwner` | Get mutable pointer |
| `owner_type() -> OwnerType` | Get owner type |
| `guid() -> Option<Guid>` | Get GUID |
| `name() -> Option<String>` | Get name |
| `equal(&Owner) -> bool` | Check equality |
| `compare(&Owner) -> i32` | Compare owners |
| `is_undefined() -> bool` | Check if undefined |
| `is_customer() -> bool` | Check if customer |
| `is_vendor() -> bool` | Check if vendor |
| `is_employee() -> bool` | Check if employee |
| `is_job() -> bool` | Check if job |

#### OwnerType Enum

| Variant | Description |
|---------|-------------|
| `GNC_OWNER_UNDEFINED` | Undefined |
| `GNC_OWNER_CUSTOMER` | Customer |
| `GNC_OWNER_VENDOR` | Vendor |
| `GNC_OWNER_EMPLOYEE` | Employee |
| `GNC_OWNER_JOB` | Job |

#### TypedOwner

A strongly-typed enum for type-safe owner handling:

```rust
use gnucash_ext::TypedOwner;

let typed: TypedOwner = TypedOwner::Customer(&customer);
let owner = typed.to_owner();
```

---

### Address

A mailing address.

**Source:** [`src/business/address.rs`](../src/business/address.rs)

```rust
use gnucash_ext::Address;

// Get from customer
if let Some(addr) = customer.addr() {
    addr.set_name("John Smith");
    addr.set_addr1("123 Main Street");
    addr.set_addr2("Suite 100");
    addr.set_addr3("Anytown, ST 12345");
    addr.set_phone("555-1234");
    addr.set_fax("555-1235");
    addr.set_email("john@example.com");
}
```

#### Methods

| Method | Description |
|--------|-------------|
| `Address::new(book: &Book) -> Self` | Create new address |
| `name() -> Option<String>` | Get name |
| `addr1() -> Option<String>` | Get line 1 |
| `addr2() -> Option<String>` | Get line 2 |
| `addr3() -> Option<String>` | Get line 3 |
| `addr4() -> Option<String>` | Get line 4 |
| `phone() -> Option<String>` | Get phone |
| `fax() -> Option<String>` | Get fax |
| `email() -> Option<String>` | Get email |
| `set_name(&str)` | Set name |
| `set_addr1(&str)` | Set line 1 |
| `set_addr2(&str)` | Set line 2 |
| `set_addr3(&str)` | Set line 3 |
| `set_addr4(&str)` | Set line 4 |
| `set_phone(&str)` | Set phone |
| `set_fax(&str)` | Set fax |
| `set_email(&str)` | Set email |
| `clear()` | Clear all fields |

---

### BillTerm

Payment terms for invoices.

**Source:** [`src/business/billterm.rs`](../src/business/billterm.rs)

```rust
use gnucash_ext::{Book, BillTerm};

let terms = BillTerm::new(&book);
terms.begin_edit();
terms.set_name("Net 30");
terms.set_description("Payment due in 30 days");
terms.set_due_days(30);
terms.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `BillTerm::new(book: &Book) -> Self` | Create new bill term |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `name() -> Option<String>` | Get name |
| `description() -> Option<String>` | Get description |
| `due_days() -> i32` | Get due days |
| `discount_days() -> i32` | Get discount days |
| `discount() -> Numeric` | Get discount amount |
| `cutoff() -> i32` | Get cutoff day |
| `refcount() -> i64` | Get reference count |
| `set_name(&str)` | Set name |
| `set_description(&str)` | Set description |
| `set_due_days(i32)` | Set due days |
| `set_discount_days(i32)` | Set discount days |
| `set_discount(Numeric)` | Set discount |
| `set_cutoff(i32)` | Set cutoff |

---

### TaxTable

Tax rates and rules.

**Source:** [`src/business/tax.rs`](../src/business/tax.rs)

```rust
use gnucash_ext::{Book, TaxTable, Numeric};

let tax = TaxTable::new(&book);
tax.begin_edit();
tax.set_name("Sales Tax");
tax.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `TaxTable::new(book: &Book) -> Self` | Create new tax table |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `name() -> Option<String>` | Get name |
| `refcount() -> i64` | Get reference count |
| `set_name(&str)` | Set name |

---

## Price Module

Enhanced price handling (extends gnucash-sys Price).

**Source:** [`src/price.rs`](../src/price.rs)

### Price

A price quote for a commodity.

```rust
use gnucash_ext::{Book, Price, Numeric, PriceSource};

let price = Price::new(&book);
price.begin_edit();
price.set_time(1704067200);
price.set_source(PriceSource::PRICE_SOURCE_USER_PRICE);
price.set_type_string("last");
price.set_value(Numeric::new(150, 1));
price.commit_edit();
```

#### Methods

| Method | Description |
|--------|-------------|
| `Price::new(book: &Book) -> Self` | Create new price |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `guid() -> Guid` | Get GUID |
| `time() -> i64` | Get timestamp |
| `source() -> PriceSource` | Get source |
| `source_string() -> Option<String>` | Get source as string |
| `type_string() -> Option<String>` | Get type string |
| `value() -> Numeric` | Get price value |
| `set_time(i64)` | Set timestamp |
| `set_source(PriceSource)` | Set source |
| `set_source_string(&str)` | Set source string |
| `set_type_string(&str)` | Set type string |
| `set_value(Numeric)` | Set value |
| `ref_()` | Increment ref count |
| `unref()` | Decrement ref count |
| `clone_for_book(&Book) -> Option<Price>` | Clone for another book |

### PriceDB

The price database for a book.

```rust
use gnucash_ext::{Book, PriceDB};

let pricedb = PriceDB::get(&book).expect("No price database");
pricedb.add_price(&price);
println!("Prices in DB: {}", pricedb.num_prices());
```

#### Methods

| Method | Description |
|--------|-------------|
| `PriceDB::get(book: &Book) -> Option<Self>` | Get database for book |
| `begin_edit()` / `commit_edit()` | Edit cycle |
| `add_price(&Price) -> bool` | Add price |
| `remove_price(&Price) -> bool` | Remove price |
| `num_prices() -> usize` | Count prices |
| `has_prices() -> bool` | Check if non-empty |

---

## Query Module

The QofQuery framework for searching GnuCash objects.

**Source:** [`src/query.rs`](../src/query.rs)

### Query

```rust
use gnucash_ext::{Query, QofQueryOp, obj_types, params, Book};

// Create a query for splits
let query = Query::for_type(obj_types::SPLIT);
query.set_book(&book);
query.set_max_results(100);

// Add predicates
query.add_boolean_match(
    &[params::SPLIT_RECONCILE],
    false,
    QofQueryOp::QOF_QUERY_AND
);

// Run query
let splits = query.run_splits();
for split in splits {
    println!("Split: {:?}", split.memo());
}
```

#### Constructor Methods

| Method | Description |
|--------|-------------|
| `Query::new() -> Self` | Create empty query |
| `Query::for_type(obj_type: &str) -> Self` | Create query for type |

#### Configuration

| Method | Description |
|--------|-------------|
| `set_search_for(&str)` | Set object type |
| `set_book(&Book)` | Set book to search |
| `set_max_results(i32)` | Set result limit |
| `clear()` | Clear query |
| `purge_terms()` | Remove all terms |

#### Running

| Method | Description |
|--------|-------------|
| `run_splits() -> Vec<Split>` | Run and return splits |
| `run_transactions() -> Vec<Transaction>` | Run and return transactions |
| `run_accounts() -> Vec<Account>` | Run and return accounts |

#### Predicates

| Method | Description |
|--------|-------------|
| `add_guid_match(&[&str], &Guid, QofQueryOp)` | Match by GUID |
| `add_boolean_match(&[&str], bool, QofQueryOp)` | Match boolean |

#### Operations

| Method | Description |
|--------|-------------|
| `merge(&Query, QofQueryOp)` | Merge queries |
| `invert() -> Query` | Invert query |
| `has_terms() -> bool` | Check for terms |
| `num_terms() -> i32` | Count terms |

#### Object Type Constants

```rust
use gnucash_ext::obj_types;

obj_types::SPLIT        // "Split"
obj_types::TRANSACTION  // "Trans"
obj_types::ACCOUNT      // "Account"
```

#### Parameter Path Constants

```rust
use gnucash_ext::params;

// Split parameters
params::SPLIT_TRANS      // "trans"
params::SPLIT_ACCOUNT    // "account"
params::SPLIT_VALUE      // "value"
params::SPLIT_AMOUNT     // "amount"
params::SPLIT_MEMO       // "memo"
params::SPLIT_RECONCILE  // "reconcile-flag"

// Transaction parameters
params::TRANS_DATE_POSTED   // "date-posted"
params::TRANS_DATE_ENTERED  // "date-entered"
params::TRANS_DESCRIPTION   // "desc"
params::TRANS_NUM           // "num"

// Account parameters
params::ACCOUNT_NAME  // "name"
params::ACCOUNT_CODE  // "code"
params::ACCOUNT_TYPE  // "account-type"

// Generic
params::QOF_PARAM_GUID  // "guid"
```

#### QofQueryOp Enum

| Variant | Description |
|---------|-------------|
| `QOF_QUERY_AND` | AND operation |
| `QOF_QUERY_OR` | OR operation |
| `QOF_QUERY_NAND` | NAND operation |
| `QOF_QUERY_NOR` | NOR operation |
| `QOF_QUERY_XOR` | XOR operation |

---

## Builder Module

Fluent builders for creating complex entities.

**Source:** [`src/builder.rs`](../src/builder.rs)

### TransactionBuilder

Create balanced transactions with multiple splits.

```rust
use gnucash_ext::{TransactionBuilder, Numeric, Book, Account};

let txn = TransactionBuilder::new(&book)
    .description("Grocery shopping")
    .date(15, 3, 2024)  // March 15, 2024
    .num("1001")
    .notes("Weekly groceries")
    .split(&checking, Numeric::new(-5000, 100), Some("Debit"))
    .split(&groceries, Numeric::new(5000, 100), Some("Groceries"))
    .build()?;

// Or use transfer helper
let txn = TransactionBuilder::new(&book)
    .description("Transfer to savings")
    .date(1, 4, 2024)
    .transfer(&checking, &savings, Numeric::new(100000, 100), None)
    .build()?;
```

#### Methods

| Method | Description |
|--------|-------------|
| `TransactionBuilder::new(book: &Book) -> Self` | Create builder |
| `description(&str) -> Self` | Set description |
| `num(&str) -> Self` | Set transaction number |
| `notes(&str) -> Self` | Set notes |
| `date(day, month, year) -> Self` | Set date |
| `currency(&str) -> Self` | Set currency mnemonic |
| `split(&Account, Numeric, Option<&str>) -> Self` | Add split |
| `transfer(&Account, &Account, Numeric, Option<&str>) -> Self` | Add transfer (2 splits) |
| `build() -> Result<Transaction>` | Build transaction |

**Note:** `build()` validates that splits balance to zero.

### InvoiceBuilder

Create invoices with entries.

```rust
use gnucash_ext::{InvoiceBuilder, Numeric, Book, Account, Owner};

let invoice = InvoiceBuilder::new(&book)
    .id("INV-001")
    .owner(&customer.to_owner())
    .date_opened(1704067200)
    .notes("Consulting services")
    .entry("Day 1", Numeric::new(15000, 100), Numeric::new(8, 1), &income)
    .entry("Day 2", Numeric::new(15000, 100), Numeric::new(6, 1), &income)
    .build()?;
```

#### Methods

| Method | Description |
|--------|-------------|
| `InvoiceBuilder::new(book: &Book) -> Self` | Create builder |
| `id(&str) -> Self` | Set invoice ID |
| `notes(&str) -> Self` | Set notes |
| `billing_id(&str) -> Self` | Set billing ID |
| `owner(&Owner) -> Self` | Set owner |
| `date_opened(i64) -> Self` | Set date opened |
| `entry(desc, price, qty, &Account) -> Self` | Add entry |
| `entry_with_action(desc, price, qty, &Account, action) -> Self` | Add entry with action |
| `build() -> Result<Invoice>` | Build invoice |

---

## Re-exports

The crate re-exports commonly used types from gnucash-sys:

```rust
// From gnucash-sys
pub use gnucash_sys::{
    init_engine, is_engine_initialized,
    Account, Book, Error, GNCAccountType, Guid, Numeric,
    Result, Session, SessionOpenMode, Split, Transaction,
};
```

See the [gnucash-sys API documentation](../../docs/API.md) for details on these types.

---

## See Also

- [gnucash-sys API Reference](../../docs/API.md) - Core types documentation
- [Examples](../examples/) - Example programs
- [simple_business.rs](../examples/simple_business.rs) - Business entities example
