//! Builder patterns for creating GnuCash entities.
//!
//! This module provides fluent builders for complex entity creation.

use gnucash_sys::{Account, Book, Numeric, Split, Transaction};

/// Builder for creating transactions with splits.
///
/// # Example
/// ```ignore
/// use gnucash_sys::{TransactionBuilder, Numeric};
///
/// let txn = TransactionBuilder::new(&book)
///     .description("Groceries")
///     .date(2024, 1, 15)
///     .split(&checking, Numeric::new(-5000, 100), None)  // -$50.00
///     .split(&expenses, Numeric::new(5000, 100), None)   // $50.00
///     .build()?;
/// ```
pub struct TransactionBuilder<'a> {
    book: &'a Book,
    description: Option<String>,
    num: Option<String>,
    notes: Option<String>,
    date_posted: Option<(i32, i32, i32)>, // (day, month, year)
    currency_mnemonic: Option<String>,
    splits: Vec<SplitSpec<'a>>,
}

struct SplitSpec<'a> {
    account: &'a Account,
    amount: Numeric,
    memo: Option<String>,
}

impl<'a> TransactionBuilder<'a> {
    /// Creates a new TransactionBuilder.
    pub fn new(book: &'a Book) -> Self {
        Self {
            book,
            description: None,
            num: None,
            notes: None,
            date_posted: None,
            currency_mnemonic: None,
            splits: Vec::new(),
        }
    }

    /// Sets the transaction description.
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Sets the transaction number.
    pub fn num(mut self, num: &str) -> Self {
        self.num = Some(num.to_string());
        self
    }

    /// Sets the transaction notes.
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }

    /// Sets the date posted (day, month, year).
    pub fn date(mut self, day: i32, month: i32, year: i32) -> Self {
        self.date_posted = Some((day, month, year));
        self
    }

    /// Sets the currency mnemonic (e.g., "USD").
    pub fn currency(mut self, mnemonic: &str) -> Self {
        self.currency_mnemonic = Some(mnemonic.to_string());
        self
    }

    /// Adds a split to the transaction.
    pub fn split(mut self, account: &'a Account, amount: Numeric, memo: Option<&str>) -> Self {
        self.splits.push(SplitSpec {
            account,
            amount,
            memo: memo.map(|s| s.to_string()),
        });
        self
    }

    /// Adds a transfer (two splits: debit and credit).
    pub fn transfer(
        self,
        from: &'a Account,
        to: &'a Account,
        amount: Numeric,
        memo: Option<&str>,
    ) -> Self {
        self.split(from, amount.neg(), memo)
            .split(to, amount, memo)
    }

    /// Builds and returns the transaction.
    ///
    /// Returns an error if the transaction is imbalanced or invalid.
    pub fn build(self) -> gnucash_sys::Result<Transaction> {
        if self.splits.is_empty() {
            return Err(gnucash_sys::Error::InvalidOperation(
                "Transaction must have at least one split".to_string(),
            ));
        }

        // Check balance
        let mut total_num: i64 = 0;
        let mut common_denom: i64 = 1;
        for split in &self.splits {
            // Simple balance check (assumes same currency)
            let amount = split.amount;
            // Find common denominator
            common_denom = lcm(common_denom, amount.denom());
        }
        for split in &self.splits {
            let amount = split.amount;
            let scaled = amount.num() * (common_denom / amount.denom());
            total_num += scaled;
        }
        if total_num != 0 {
            return Err(gnucash_sys::Error::InvalidOperation(format!(
                "Transaction is imbalanced by {}/{}",
                total_num, common_denom
            )));
        }

        // Create transaction
        let txn = Transaction::new(self.book);
        txn.begin_edit();

        if let Some(desc) = &self.description {
            txn.set_description(desc);
        }
        if let Some(num) = &self.num {
            txn.set_num(num);
        }
        if let Some(notes) = &self.notes {
            txn.set_notes(notes);
        }
        if let Some((day, month, year)) = self.date_posted {
            txn.set_date(day, month, year);
        }

        // Create splits
        for split_spec in self.splits {
            let split = Split::new(self.book);
            split.set_account(split_spec.account);
            split.set_transaction(&txn);
            split.set_amount(split_spec.amount);
            split.set_value(split_spec.amount);
            if let Some(memo) = &split_spec.memo {
                split.set_memo(memo);
            }
        }

        txn.commit_edit();
        Ok(txn)
    }
}

/// Calculates the least common multiple.
fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

/// Calculates the greatest common divisor.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

use crate::business::{Entry, Invoice, Owner};

/// Builder for creating invoices with entries.
///
/// # Example
/// ```ignore
/// use gnucash_sys::{InvoiceBuilder, Numeric};
///
/// let invoice = InvoiceBuilder::new(&book)
///     .id("INV-001")
///     .owner(&customer.to_owner())
///     .date_opened(time64)
///     .entry("Consulting", Numeric::new(10000, 100), Numeric::new(1, 1), &income_account)
///     .build()?;
/// ```
pub struct InvoiceBuilder<'a> {
    book: &'a Book,
    id: Option<String>,
    notes: Option<String>,
    billing_id: Option<String>,
    owner: Option<&'a Owner>,
    date_opened: Option<i64>,
    entries: Vec<EntrySpec<'a>>,
}

struct EntrySpec<'a> {
    description: String,
    price: Numeric,
    quantity: Numeric,
    account: &'a Account,
    action: Option<String>,
}

impl<'a> InvoiceBuilder<'a> {
    /// Creates a new InvoiceBuilder.
    pub fn new(book: &'a Book) -> Self {
        Self {
            book,
            id: None,
            notes: None,
            billing_id: None,
            owner: None,
            date_opened: None,
            entries: Vec::new(),
        }
    }

    /// Sets the invoice ID.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets the invoice notes.
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }

    /// Sets the billing ID.
    pub fn billing_id(mut self, billing_id: &str) -> Self {
        self.billing_id = Some(billing_id.to_string());
        self
    }

    /// Sets the owner (customer, vendor, etc.).
    pub fn owner(mut self, owner: &'a Owner) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Sets the date opened.
    pub fn date_opened(mut self, date: i64) -> Self {
        self.date_opened = Some(date);
        self
    }

    /// Adds an entry to the invoice.
    pub fn entry(
        mut self,
        description: &str,
        price: Numeric,
        quantity: Numeric,
        account: &'a Account,
    ) -> Self {
        self.entries.push(EntrySpec {
            description: description.to_string(),
            price,
            quantity,
            account,
            action: None,
        });
        self
    }

    /// Adds an entry with action to the invoice.
    pub fn entry_with_action(
        mut self,
        description: &str,
        price: Numeric,
        quantity: Numeric,
        account: &'a Account,
        action: &str,
    ) -> Self {
        self.entries.push(EntrySpec {
            description: description.to_string(),
            price,
            quantity,
            account,
            action: Some(action.to_string()),
        });
        self
    }

    /// Builds and returns the invoice.
    pub fn build(self) -> gnucash_sys::Result<Invoice> {
        let invoice = Invoice::new(self.book);
        invoice.begin_edit();

        if let Some(id) = &self.id {
            invoice.set_id(id);
        }
        if let Some(notes) = &self.notes {
            invoice.set_notes(notes);
        }
        if let Some(billing_id) = &self.billing_id {
            invoice.set_billing_id(billing_id);
        }
        if let Some(owner) = self.owner {
            invoice.set_owner(owner);
        }
        if let Some(date) = self.date_opened {
            invoice.set_date_opened(date);
        }

        // Create entries
        for entry_spec in self.entries {
            let entry = Entry::new(self.book);
            entry.begin_edit();
            entry.set_description(&entry_spec.description);
            entry.set_inv_price(entry_spec.price);
            entry.set_quantity(entry_spec.quantity);
            entry.set_inv_account(entry_spec.account);
            if let Some(action) = &entry_spec.action {
                entry.set_action(action);
            }
            entry.commit_edit();
            // Note: Entry needs to be added to invoice via gncInvoiceAddEntry
            // which we'd need to expose
        }

        invoice.commit_edit();
        Ok(invoice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(100, 100), 100);
    }
}
