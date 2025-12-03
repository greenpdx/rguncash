//! simple_business_create.rs - Rust port of simple_business_create.py
//!
//! Set up a set of books for business feature use.
//! Based on the Python example from GnuCash bindings.

// Use gnucash_ext which re-exports gnucash_sys types plus business entities
use gnucash_ext::{
    gnucash_sys::ffi::GncAmountType,
    init_engine, Account, Book, Customer, Employee, Entry, GNCAccountType, Invoice, Job, Numeric,
    OwnerType, Session, TaxTable, TaxTableEntry, Vendor,
};

/// Creates an account with the given properties.
/// Returns a mutable account so it can be marked as unowned after attaching to hierarchy.
fn create_account(book: &Book, name: &str, account_type: GNCAccountType) -> Account {
    let mut account = Account::new(book);
    account.begin_edit();
    account.set_name(name);
    account.set_type(account_type);
    account.commit_edit();
    // Mark as unowned since we'll attach to hierarchy (book takes ownership)
    account.mark_unowned();
    account
}

fn main() {
    // Check command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough parameters");
        eprintln!("usage: gnucash-app {{new_book_url}}");
        eprintln!("example:");
        eprintln!("  gnucash-app sqlite3:///home/user/test.gnucash");
        eprintln!("  gnucash-app /home/user/test.gnucash");
        std::process::exit(1);
    }

    let _book_url = &args[1];

    // Initialize the GnuCash engine
    init_engine();
    println!("GnuCash engine initialized");

    // Create a new session with an empty book
    let session = Session::new();
    let book = session.book().expect("Failed to get book from session");
    println!("Created new session and book");

    // Create root account
    let root = create_account(&book, "Root", GNCAccountType::ACCT_TYPE_ROOT);
    book.set_root_account(&root);
    println!("Created root account");

    // Create account hierarchy:
    //   Root
    //   ├── Assets (ASSET)
    //   │   ├── Receivables (RECEIVABLE)
    //   │   └── Bank (ASSET)
    //   ├── Income (INCOME)
    //   └── Liabilities (LIABILITY)
    //       └── Tax payable (LIABILITY)

    // Assets
    let assets = create_account(&book, "Assets", GNCAccountType::ACCT_TYPE_ASSET);
    root.append_child(&assets);

    // Assets:Receivables
    let receivables = create_account(&book, "Receivables", GNCAccountType::ACCT_TYPE_RECEIVABLE);
    assets.append_child(&receivables);

    // Assets:Bank
    let bank = create_account(&book, "Bank", GNCAccountType::ACCT_TYPE_ASSET);
    assets.append_child(&bank);

    // Income
    let income = create_account(&book, "Income", GNCAccountType::ACCT_TYPE_INCOME);
    root.append_child(&income);

    // Liabilities
    let liabilities = create_account(&book, "Liabilities", GNCAccountType::ACCT_TYPE_LIABILITY);
    root.append_child(&liabilities);

    // Liabilities:Tax payable
    let tax_payable = create_account(&book, "Tax payable", GNCAccountType::ACCT_TYPE_LIABILITY);
    liabilities.append_child(&tax_payable);

    println!("Created account hierarchy:");
    println!("  Root");
    println!("  ├── Assets");
    println!("  │   ├── Receivables");
    println!("  │   └── Bank");
    println!("  ├── Income");
    println!("  └── Liabilities");
    println!("      └── Tax payable");

    // Create a Customer
    let customer = Customer::new(&book);
    customer.begin_edit();
    customer.set_id("1");
    customer.set_name("Bill & Bob Industries");
    if let Some(addr) = customer.addr() {
        addr.set_name("Bill & Bob");
        addr.set_addr1("201 Nowhere street");
    }
    customer.commit_edit();
    println!("Created customer: {:?}", customer);

    // Create an Employee
    let employee = Employee::new(&book);
    employee.begin_edit();
    employee.set_id("2");
    employee.set_username("Reliable employee");
    employee.commit_edit();
    println!("Created employee: {:?}", employee);

    // Create a Vendor
    let vendor = Vendor::new(&book);
    vendor.begin_edit();
    vendor.set_id("3");
    vendor.set_name("Dependable vendor");
    vendor.commit_edit();
    println!("Created vendor: {:?}", vendor);

    // Create a Job linked to the vendor
    let job = Job::new(&book);
    job.begin_edit();
    job.set_id("4");
    job.set_name("Good clean, fun");
    job.set_owner(&vendor.to_owner());
    job.commit_edit();
    println!("Created job: {:?}", job);

    // Create a TaxTable with 7% tax rate
    let tax_table = TaxTable::new(&book);
    tax_table.begin_edit();
    tax_table.set_name("good tax");

    // Create a tax table entry (7% = 700000/100000)
    let tax_entry = TaxTableEntry::new();
    tax_entry.set_account(&tax_payable);
    tax_entry.set_type(GncAmountType::GNC_AMT_TYPE_PERCENT);
    tax_entry.set_amount(Numeric::new(700000, 100000)); // 7%
    tax_table.add_entry(&tax_entry);

    tax_table.commit_edit();
    println!("Created tax table: {:?}", tax_table);

    // Create Invoice for Customer
    let invoice_customer = Invoice::new(&book);
    invoice_customer.begin_edit();
    invoice_customer.set_id("5");
    invoice_customer.set_owner(&customer.to_owner());
    invoice_customer.commit_edit();

    // Verify owner extraction
    let customer_extract = invoice_customer.owner();
    assert!(
        customer_extract.owner_type() == OwnerType::GNC_OWNER_CUSTOMER,
        "Customer invoice should have customer owner type"
    );
    println!("Created customer invoice: {:?}", invoice_customer);

    // Create Invoice for Employee
    let invoice_employee = Invoice::new(&book);
    invoice_employee.begin_edit();
    invoice_employee.set_id("6");
    invoice_employee.set_owner(&employee.to_owner());
    invoice_employee.commit_edit();

    let employee_extract = invoice_employee.owner();
    assert!(
        employee_extract.owner_type() == OwnerType::GNC_OWNER_EMPLOYEE,
        "Employee invoice should have employee owner type"
    );
    println!("Created employee invoice: {:?}", invoice_employee);

    // Create Invoice for Vendor (this is a bill)
    let invoice_vendor = Invoice::new(&book);
    invoice_vendor.begin_edit();
    invoice_vendor.set_id("7");
    invoice_vendor.set_owner(&vendor.to_owner());
    invoice_vendor.commit_edit();

    let vendor_extract = invoice_vendor.owner();
    assert!(
        vendor_extract.owner_type() == OwnerType::GNC_OWNER_VENDOR,
        "Vendor invoice should have vendor owner type"
    );
    println!("Created vendor invoice (bill): {:?}", invoice_vendor);

    // Create Invoice for Job
    let invoice_job = Invoice::new(&book);
    invoice_job.begin_edit();
    invoice_job.set_id("8");
    invoice_job.set_owner(&job.to_owner());
    invoice_job.commit_edit();

    let job_extract = invoice_job.owner();
    assert!(
        job_extract.owner_type() == OwnerType::GNC_OWNER_JOB,
        "Job invoice should have job owner type"
    );
    println!("Created job invoice: {:?}", invoice_job);

    // Create an Entry for the customer invoice
    let invoice_entry = Entry::new(&book);
    invoice_entry.begin_edit();
    invoice_entry.set_description("excellent product");
    invoice_entry.set_quantity(Numeric::new(1, 1));
    invoice_entry.set_inv_account(&income);
    invoice_entry.set_inv_price(Numeric::new(1, 1));
    invoice_entry.set_inv_tax_table(&tax_table);
    invoice_entry.set_inv_tax_included(false);
    invoice_entry.set_inv_taxable(true);

    // Get current timestamp for date_entered
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    invoice_entry.set_date_entered(now);
    invoice_entry.commit_edit();

    // Add entry to invoice
    invoice_customer.add_entry(&invoice_entry);
    println!("Created and added entry to customer invoice: {:?}", invoice_entry);

    // Summary
    println!("\n=== Summary ===");
    println!("Book transactions: {}", book.transaction_count());
    println!("Root account children: {}", root.n_children());
    println!("Customer invoice ID: {:?}", invoice_customer.id());
    println!("Invoice total: {:?}", invoice_customer.total());

    println!("\nNote: Invoice posting and payment application require");
    println!("      additional FFI bindings (gncInvoicePostToAccount,");
    println!("      gncOwnerApplyPayment). These can be added as needed.");

    // Note: Saving to file would require proper session begin with a file path
    // For now, we just demonstrate the entity creation
    println!("\nData created in memory. To save:");
    println!("  1. Use Session::open() with a file path");
    println!("  2. Call session.save()");

    // Clean up
    session.end();
    println!("\nSession ended successfully.");
}
