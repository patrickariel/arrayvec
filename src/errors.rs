use std::fmt;
#[cfg(feature="std")]
use std::any::Any;
#[cfg(feature="std")]
use std::error::Error;

/// Error value indicating insufficient capacity
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct CapacityError<T = ()> {
    element: T,
}

impl<T> CapacityError<T> {
    /// Create a new `CapacityError` from `element`.
    pub const fn new(element: T) -> CapacityError<T> {
        CapacityError {
            element: element,
        }
    }

    /// Extract the overflowing element
    pub fn element(self) -> T {
        self.element
    }

    /// Convert into a `CapacityError` that does not carry an element.
    pub fn simplify(self) -> CapacityError {
        CapacityError { element: () }
    }
}

const CAPERROR: &'static str = "insufficient capacity";

#[cfg(feature="std")]
/// Requires `features="std"`.
impl<T: Any> Error for CapacityError<T> {}

impl<T> fmt::Display for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CAPERROR)
    }
}

impl<T> fmt::Debug for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", "CapacityError", CAPERROR)
    }
}

/// Error value indicating that capacity is not completely filled
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnderfilledError {
    capacity: usize,
    len: usize,
}

impl UnderfilledError {
    pub const fn new(capacity: usize, len: usize) -> Self {
        Self { capacity, len }
    }
}

#[cfg(feature="std")]
impl Error for UnderfilledError {}

impl fmt::Display for UnderfilledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "capacity is not filled: expected {}, got {}", self.capacity, self.len)
    }
}