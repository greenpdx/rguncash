//! Safe wrapper for GnuCash Price and PriceDB.

use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid, Numeric};

/// A price quote for a commodity.
pub struct Price {
    ptr: NonNull<ffi::GNCPrice>,
    owned: bool,
}

unsafe impl Send for Price {}

impl Price {
    /// Creates a new price in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gnc_price_create(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gnc_price_create returned null"),
            owned: true,
        }
    }

    /// Creates a Price wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GNCPrice.
    pub unsafe fn from_raw(ptr: *mut ffi::GNCPrice, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GNCPrice {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this price.
    pub fn guid(&self) -> Guid {
        unsafe {
            let instance = self.ptr.as_ptr() as *const std::ffi::c_void;
            let guid_ptr = ffi::qof_instance_get_guid(instance);
            if guid_ptr.is_null() {
                Guid::from_bytes([0; 16])
            } else {
                Guid::from_bytes((*guid_ptr).reserved)
            }
        }
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gnc_price_begin_edit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gnc_price_commit_edit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the price time as time64.
    pub fn time(&self) -> i64 {
        unsafe { ffi::gnc_price_get_time64(self.ptr.as_ptr()) }
    }

    /// Returns the price source.
    pub fn source(&self) -> PriceSource {
        unsafe { ffi::gnc_price_get_source(self.ptr.as_ptr()) }
    }

    /// Returns the source string.
    pub fn source_string(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_price_get_source_string(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the type string (e.g., "last", "bid", "ask").
    pub fn type_string(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_price_get_typestr(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the price value.
    pub fn value(&self) -> Numeric {
        unsafe { ffi::gnc_price_get_value(self.ptr.as_ptr()).into() }
    }

    // ==================== Setters ====================

    /// Sets the price time.
    pub fn set_time(&self, time: i64) {
        unsafe { ffi::gnc_price_set_time64(self.ptr.as_ptr(), time) }
    }

    /// Sets the price source.
    pub fn set_source(&self, source: PriceSource) {
        unsafe { ffi::gnc_price_set_source(self.ptr.as_ptr(), source) }
    }

    /// Sets the price source as string.
    pub fn set_source_string(&self, source: &str) {
        let c_source = std::ffi::CString::new(source).unwrap();
        unsafe { ffi::gnc_price_set_source_string(self.ptr.as_ptr(), c_source.as_ptr()) }
    }

    /// Sets the price type string.
    pub fn set_type_string(&self, type_str: &str) {
        let c_type = std::ffi::CString::new(type_str).unwrap();
        unsafe { ffi::gnc_price_set_typestr(self.ptr.as_ptr(), c_type.as_ptr()) }
    }

    /// Sets the price value.
    pub fn set_value(&self, value: Numeric) {
        unsafe { ffi::gnc_price_set_value(self.ptr.as_ptr(), value.into()) }
    }

    /// Increments the reference count.
    pub fn ref_(&self) {
        unsafe { ffi::gnc_price_ref(self.ptr.as_ptr()) }
    }

    /// Decrements the reference count.
    pub fn unref(&self) {
        unsafe { ffi::gnc_price_unref(self.ptr.as_ptr()) }
    }

    /// Clones this price for a new book.
    pub fn clone_for_book(&self, book: &Book) -> Option<Price> {
        unsafe {
            let ptr = ffi::gnc_price_clone(self.ptr.as_ptr(), book.as_ptr());
            Self::from_raw(ptr, true)
        }
    }
}

impl Clone for Price {
    fn clone(&self) -> Self {
        self.ref_();
        Self {
            ptr: self.ptr,
            owned: true,
        }
    }
}

impl Drop for Price {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gnc_price_unref(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Price")
            .field("guid", &self.guid())
            .field("time", &self.time())
            .field("value", &self.value())
            .field("source", &self.source_string())
            .finish()
    }
}

/// Re-export PriceSource enum.
pub use ffi::PriceSource;

/// The price database for a book.
pub struct PriceDB {
    ptr: NonNull<ffi::GNCPriceDB>,
}

unsafe impl Send for PriceDB {}

impl PriceDB {
    /// Gets the price database for a book.
    pub fn get(book: &Book) -> Option<Self> {
        unsafe {
            let ptr = ffi::gnc_pricedb_get_db(book.as_ptr());
            NonNull::new(ptr).map(|ptr| Self { ptr })
        }
    }

    /// Creates a PriceDB wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GNCPriceDB) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GNCPriceDB {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gnc_pricedb_begin_edit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gnc_pricedb_commit_edit(self.ptr.as_ptr()) }
    }

    /// Adds a price to the database.
    /// Returns true if the price was added successfully.
    pub fn add_price(&self, price: &Price) -> bool {
        unsafe { ffi::gnc_pricedb_add_price(self.ptr.as_ptr(), price.as_ptr()) != 0 }
    }

    /// Removes a price from the database.
    /// Returns true if the price was removed successfully.
    pub fn remove_price(&self, price: &Price) -> bool {
        unsafe { ffi::gnc_pricedb_remove_price(self.ptr.as_ptr(), price.as_ptr()) != 0 }
    }

    /// Returns the number of prices in the database.
    pub fn num_prices(&self) -> usize {
        unsafe { ffi::gnc_pricedb_get_num_prices(self.ptr.as_ptr()) as usize }
    }

    /// Checks if the database has any prices.
    pub fn has_prices(&self) -> bool {
        self.num_prices() > 0
    }
}

impl std::fmt::Debug for PriceDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceDB")
            .field("num_prices", &self.num_prices())
            .finish()
    }
}
