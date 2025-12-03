//! Safe wrapper for QofSession - connection to a GnuCash data store.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;
use std::sync::Once;

use crate::ffi;
use crate::Book;

pub use crate::ffi::SessionOpenMode;
pub use crate::ffi::QofBackendError;

static ENGINE_INIT: Once = Once::new();

/// Initialize the GnuCash engine. Must be called before any other operations.
/// This is safe to call multiple times - it will only initialize once.
pub fn init_engine() {
    ENGINE_INIT.call_once(|| {
        unsafe {
            ffi::gnc_engine_init(0, std::ptr::null_mut());
        }
    });
}

/// Check if the engine is initialized.
pub fn is_engine_initialized() -> bool {
    unsafe { ffi::gnc_engine_is_initialized() != 0 }
}

/// A GnuCash Session - connection to a data store (file or database).
///
/// A Session encapsulates a connection to a GnuCash data file or database.
/// It manages file locking, loading, and saving of data.
pub struct Session {
    ptr: NonNull<ffi::QofSession>,
}

unsafe impl Send for Session {}

impl Session {
    /// Creates a new session with a new empty book.
    pub fn new() -> Self {
        init_engine();
        let book = unsafe { ffi::qof_book_new() };
        let ptr = unsafe { ffi::qof_session_new(book) };
        Self {
            ptr: NonNull::new(ptr).expect("qof_session_new returned null"),
        }
    }

    /// Opens a GnuCash data file or database.
    ///
    /// # Arguments
    /// * `uri` - Path to the file (e.g., "file:///path/to/file.gnucash" or just "/path/to/file.gnucash")
    /// * `mode` - How to open the file (read-only, create new, etc.)
    ///
    /// # Returns
    /// `Ok(Session)` on success, `Err(QofBackendError)` on failure.
    pub fn open(uri: &str, mode: SessionOpenMode) -> Result<Self, QofBackendError> {
        init_engine();

        let book = unsafe { ffi::qof_book_new() };
        let ptr = unsafe { ffi::qof_session_new(book) };
        let session = Self {
            ptr: NonNull::new(ptr).expect("qof_session_new returned null"),
        };

        // Normalize the URI - add file:// prefix if it's a plain path
        let uri = if uri.contains("://") {
            uri.to_string()
        } else {
            format!("file://{}", uri)
        };

        let c_uri = CString::new(uri).map_err(|_| QofBackendError::ERR_BACKEND_BAD_URL)?;

        unsafe {
            ffi::qof_session_begin(session.ptr.as_ptr(), c_uri.as_ptr(), mode);
        }

        let err = session.get_error();
        if err != QofBackendError::ERR_BACKEND_NO_ERR {
            return Err(err);
        }

        // Load the data
        unsafe {
            ffi::qof_session_load(session.ptr.as_ptr(), None);
        }

        let err = session.get_error();
        if err != QofBackendError::ERR_BACKEND_NO_ERR {
            return Err(err);
        }

        Ok(session)
    }

    /// Opens a file read-only (convenience method).
    pub fn open_readonly(path: &str) -> Result<Self, QofBackendError> {
        Self::open(path, SessionOpenMode::SESSION_READ_ONLY)
    }

    /// Returns the raw pointer to the underlying QofSession.
    pub fn as_ptr(&self) -> *mut ffi::QofSession {
        self.ptr.as_ptr()
    }

    /// Returns the book associated with this session.
    pub fn book(&self) -> Option<Book> {
        unsafe {
            let ptr = ffi::qof_session_get_book(self.ptr.as_ptr());
            Book::from_raw(ptr, false)
        }
    }

    /// Returns the last error from the session.
    pub fn get_error(&self) -> QofBackendError {
        unsafe { ffi::qof_session_get_error(self.ptr.as_ptr()) }
    }

    /// Pops and returns the last error from the session.
    pub fn pop_error(&self) -> QofBackendError {
        unsafe { ffi::qof_session_pop_error(self.ptr.as_ptr()) }
    }

    /// Returns the error message string.
    pub fn get_error_message(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::qof_session_get_error_message(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the file path for this session.
    pub fn file_path(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::qof_session_get_file_path(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the URL for this session.
    pub fn url(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::qof_session_get_url(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Saves the session data.
    pub fn save(&self) -> Result<(), QofBackendError> {
        unsafe {
            ffi::qof_session_save(self.ptr.as_ptr(), None);
        }
        let err = self.get_error();
        if err != QofBackendError::ERR_BACKEND_NO_ERR {
            Err(err)
        } else {
            Ok(())
        }
    }

    /// Ensures all data is loaded from the session.
    pub fn ensure_all_data_loaded(&self) {
        unsafe {
            ffi::qof_session_ensure_all_data_loaded(self.ptr.as_ptr());
        }
    }

    /// Ends the session (releases locks, etc.).
    pub fn end(&self) {
        unsafe {
            ffi::qof_session_end(self.ptr.as_ptr());
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ffi::qof_session_end(self.ptr.as_ptr());
            ffi::qof_session_destroy(self.ptr.as_ptr());
        }
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("url", &self.url())
            .field("file_path", &self.file_path())
            .finish()
    }
}
