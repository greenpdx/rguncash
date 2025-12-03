# gnucash-ext

Extended functionality for gnucash-sys, providing business entities, queries, and builder patterns.

## Features

- **Business Entities** - Customer, Vendor, Employee, Job, Invoice, Entry, and more
- **Price Database** - Enhanced price handling and queries
- **Query Framework** - QofQuery wrapper for searching GnuCash objects
- **Builders** - Fluent APIs for creating transactions and invoices

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gnucash-ext = { path = "path/to/gnucash-sys/app" }
```

## Quick Start

```rust
use gnucash_ext::{
    init_engine, Book, Customer, Invoice, Entry, Numeric,
    TransactionBuilder,
};

fn main() {
    init_engine();

    let book = Book::new();

    // Create a customer
    let customer = Customer::new(&book);
    customer.begin_edit();
    customer.set_id("CUST001");
    customer.set_name("Acme Corporation");
    customer.set_active(true);
    customer.commit_edit();

    // Create an invoice
    let invoice = Invoice::new(&book);
    invoice.begin_edit();
    invoice.set_id("INV-001");
    invoice.set_owner(&customer.to_owner());
    invoice.commit_edit();

    // Add line items
    let entry = Entry::new(&book);
    entry.set_description("Consulting services");
    entry.set_quantity(Numeric::new(8, 1));
    entry.set_inv_price(Numeric::new(15000, 100));
    invoice.add_entry(&entry);

    println!("Invoice total: {}", invoice.total());
}
```

## Modules

### Business (`gnucash_ext::business`)

Business entities for invoicing and customer management:

| Type | Description |
|------|-------------|
| `Customer` | Customer who receives invoices |
| `Vendor` | Vendor/supplier who sends bills |
| `Employee` | Employee who submits expense vouchers |
| `Job` | Project linked to a customer |
| `Invoice` | Invoice, bill, or expense voucher |
| `Entry` | Line item in an invoice |
| `Owner` | Polymorphic owner type |
| `Address` | Mailing address |
| `BillTerm` | Payment terms |
| `TaxTable` | Tax rates |

### Query (`gnucash_ext::query`)

QofQuery framework for searching:

```rust
use gnucash_ext::{Query, obj_types, QofQueryOp};

let query = Query::for_type(obj_types::SPLIT);
query.set_book(&book);
query.set_max_results(100);

let splits = query.run_splits();
```

### Builder (`gnucash_ext::builder`)

Fluent builders for complex entities:

```rust
use gnucash_ext::{TransactionBuilder, Numeric};

let txn = TransactionBuilder::new(&book)
    .description("Grocery shopping")
    .date(15, 3, 2024)
    .transfer(&checking, &groceries, Numeric::new(5000, 100), None)
    .build()?;
```

## Examples

Run the business example:

```bash
cargo run -p gnucash-ext --example simple_business
```

## Documentation

- [API Reference](docs/API.md) - Detailed API documentation
- [Examples](examples/) - Example programs

## Dependencies

This crate depends on gnucash-sys, which requires:

- GnuCash development libraries (`libgnucash-dev`)
- glib-2.0 development libraries

## License

MIT
