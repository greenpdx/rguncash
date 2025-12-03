//! Safe wrapper for GncOwner - polymorphic business owner.

use std::ffi::CStr;

use gnucash_sys::ffi;
use gnucash_sys::Guid;

pub use ffi::GncOwnerType as OwnerType;

/// A polymorphic owner - can be Customer, Vendor, Employee, or Job.
#[repr(C)]
pub struct Owner {
    inner: ffi::GncOwner,
}

impl Owner {
    /// Creates a new undefined owner.
    pub fn new() -> Self {
        let mut inner = ffi::GncOwner::default();
        unsafe { ffi::gncOwnerInitUndefined(&mut inner, std::ptr::null_mut()) };
        Self { inner }
    }

    /// Creates an Owner from a raw GncOwner.
    pub fn from_raw(raw: ffi::GncOwner) -> Self {
        Self { inner: raw }
    }

    /// Returns a mutable pointer to the inner GncOwner.
    /// Note: Returns mut even for &self because GnuCash APIs often require mut.
    pub fn as_ptr(&self) -> *mut ffi::GncOwner {
        &self.inner as *const ffi::GncOwner as *mut ffi::GncOwner
    }

    /// Returns a mutable pointer to the inner GncOwner.
    pub fn as_mut_ptr(&mut self) -> *mut ffi::GncOwner {
        &mut self.inner
    }

    /// Returns the owner type.
    pub fn owner_type(&self) -> OwnerType {
        unsafe { ffi::gncOwnerGetType(&self.inner) }
    }

    /// Returns the GUID of the owner.
    pub fn guid(&self) -> Option<Guid> {
        unsafe {
            let ptr = ffi::gncOwnerGetGUID(&self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Guid::from_bytes((*ptr).reserved))
            }
        }
    }

    /// Returns the owner's name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncOwnerGetName(&self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Checks if two owners are equal.
    pub fn equal(&self, other: &Owner) -> bool {
        unsafe { ffi::gncOwnerEqual(&self.inner, &other.inner) != 0 }
    }

    /// Compares two owners.
    pub fn compare(&self, other: &Owner) -> i32 {
        unsafe { ffi::gncOwnerCompare(&self.inner, &other.inner) }
    }

    /// Returns true if this is an undefined owner.
    pub fn is_undefined(&self) -> bool {
        matches!(self.owner_type(), OwnerType::GNC_OWNER_UNDEFINED)
    }

    /// Returns true if this is a customer.
    pub fn is_customer(&self) -> bool {
        matches!(self.owner_type(), OwnerType::GNC_OWNER_CUSTOMER)
    }

    /// Returns true if this is a vendor.
    pub fn is_vendor(&self) -> bool {
        matches!(self.owner_type(), OwnerType::GNC_OWNER_VENDOR)
    }

    /// Returns true if this is an employee.
    pub fn is_employee(&self) -> bool {
        matches!(self.owner_type(), OwnerType::GNC_OWNER_EMPLOYEE)
    }

    /// Returns true if this is a job.
    pub fn is_job(&self) -> bool {
        matches!(self.owner_type(), OwnerType::GNC_OWNER_JOB)
    }
}

impl Default for Owner {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Owner {
    fn clone(&self) -> Self {
        let mut new_owner = Self::new();
        unsafe { ffi::gncOwnerCopy(&self.inner, &mut new_owner.inner) };
        new_owner
    }
}

impl std::fmt::Debug for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Owner")
            .field("type", &self.owner_type())
            .field("name", &self.name())
            .finish()
    }
}

use super::{Customer, Employee, Job, Vendor};

/// A strongly-typed owner that can be one of Customer, Vendor, Employee, or Job.
///
/// This enum provides type-safe access to the underlying owner entity.
#[derive(Debug)]
pub enum TypedOwner<'a> {
    /// A customer owner.
    Customer(&'a Customer),
    /// A vendor owner.
    Vendor(&'a Vendor),
    /// An employee owner.
    Employee(&'a Employee),
    /// A job owner.
    Job(&'a Job),
}

impl<'a> TypedOwner<'a> {
    /// Converts to an Owner for use with GnuCash APIs.
    pub fn to_owner(&self) -> Owner {
        match self {
            TypedOwner::Customer(c) => c.to_owner(),
            TypedOwner::Vendor(v) => v.to_owner(),
            TypedOwner::Employee(e) => e.to_owner(),
            TypedOwner::Job(j) => j.to_owner(),
        }
    }

    /// Returns the name of the owner.
    pub fn name(&self) -> Option<String> {
        self.to_owner().name()
    }

    /// Returns the GUID of the owner.
    pub fn guid(&self) -> Option<Guid> {
        self.to_owner().guid()
    }
}

impl<'a> From<&'a Customer> for TypedOwner<'a> {
    fn from(customer: &'a Customer) -> Self {
        TypedOwner::Customer(customer)
    }
}

impl<'a> From<&'a Vendor> for TypedOwner<'a> {
    fn from(vendor: &'a Vendor) -> Self {
        TypedOwner::Vendor(vendor)
    }
}

impl<'a> From<&'a Employee> for TypedOwner<'a> {
    fn from(employee: &'a Employee) -> Self {
        TypedOwner::Employee(employee)
    }
}

impl<'a> From<&'a Job> for TypedOwner<'a> {
    fn from(job: &'a Job) -> Self {
        TypedOwner::Job(job)
    }
}
