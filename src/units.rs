use std::ops::{Add, Deref};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]

/// Struct representing Kg
pub struct Kg(pub f64);

impl Kg {}

// FIX: This doesn't work at the moment
/// Implementing deref for the Kg struct allowing to manipulate the value directly.
impl Deref for Kg {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///
/// Implementing deref for the Kg struct allowing to manipulate the value directly.
impl Deref for Km {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementing Add for the Kg struct assuring that only elements from the same type are added
/// together.
impl Add for Kg {
    type Output = Kg;
    fn add(self, other: Kg) -> Self {
        Kg(self.0 + other.0)
    }
}

/// Implementing the sum iterator for the Kg struct
impl std::iter::Sum for Kg {
    fn sum<I: Iterator<Item = Kg>>(iter: I) -> Kg {
        iter.fold(Kg(0.0), |a, b| a + b)
    }
}

#[derive(Debug)]
pub struct Km(pub f64);

impl Km {}
