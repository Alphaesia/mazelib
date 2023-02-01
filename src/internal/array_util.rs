use std::num::NonZeroUsize;
use crate::internal::util::NONZERO_USIZE_ONE;

pub trait Sum {
    type Output;

    fn sum(&self) -> Self::Output;
}

impl <const LENGTH: usize> Sum for [usize; LENGTH] {
    type Output = usize;

    /// Return the sum of all elements in an array.
    fn sum(&self) -> Self::Output {
        let mut sum = 0;

        for x in self {
            sum += x;
        }

        return sum;
    }
}

impl <const LENGTH: usize> Sum for [NonZeroUsize; LENGTH] {
    type Output = usize;

    /// Return the sum of all elements in an array.
    fn sum(&self) -> Self::Output {
        let mut sum = 0;

        for x in self {
            sum += usize::from(*x);
        }

        return sum;
    }
}

pub trait CheckedSum {
    type Output;

    fn checked_sum(&self) -> Option<Self::Output>;
}

impl <const LENGTH: usize> CheckedSum for [usize; LENGTH] {
    type Output = usize;

    /// Return the sum of all elements in an array.
    fn checked_sum(&self) -> Option<Self::Output> {
        let mut sum = 0usize;

        for x in self {
            sum = sum.checked_add(*x)?;
        }

        return Some(sum);
    }
}

impl <const LENGTH: usize> CheckedSum for [NonZeroUsize; LENGTH] {
    type Output = usize;

    /// Return the sum of all elements in an array.
    fn checked_sum(&self) -> Option<Self::Output> {
        let mut sum = 0usize;

        for x in self {
            sum = sum.checked_add(usize::from(*x))?;
        }

        return Some(sum);
    }
}

pub trait Product {
    type Output;

    fn product(&self) -> Self::Output;
}

impl <const LENGTH: usize> Product for [usize; LENGTH] {
    type Output = usize;

    /// Return the product of all elements in an array.
    /// Returns 1 when LENGTH == 0.
    fn product(&self) -> Self::Output {
        let mut product = 1;

        for x in self {
            product *= x;
        }

        return product;
    }
}

impl <const LENGTH: usize> Product for [NonZeroUsize; LENGTH] {
    type Output = NonZeroUsize;

    /// Return the product of all elements in an array.
    /// Returns 1 when LENGTH == 0.
    fn product(&self) -> Self::Output {
        let mut product = NONZERO_USIZE_ONE;

        for x in self {
            product = product.checked_mul(*x).expect("overflow");
        }

        return product;
    }
}

pub trait CheckedProduct {
    type Output;

    /// Return None if overflow occurs.
    fn checked_product(&self) -> Option<Self::Output>;
}

impl <const LENGTH: usize> CheckedProduct for [usize; LENGTH] {
    type Output = usize;

    /// Return the product of all elements in an array.
    /// Returns 1 when LENGTH == 0.
    fn checked_product(&self) -> Option<Self::Output> {
        let mut product = 1usize;

        for x in self {
            product = product.checked_mul(*x)?;
        }

        return Some(product);
    }
}

impl <const LENGTH: usize> CheckedProduct for [NonZeroUsize; LENGTH] {
    type Output = NonZeroUsize;

    /// Return the product of all elements in an array.
    /// Returns 1 when LENGTH == 0.
    fn checked_product(&self) -> Option<Self::Output> {
        let mut product = NONZERO_USIZE_ONE;

        for x in self {
            product = product.checked_mul(*x)?;
        }

        return Some(product);
    }
}

pub trait And<T> {
    fn and(&self, other: T) -> T;
}

impl <const LENGTH: usize> And<[bool; LENGTH]> for [bool; LENGTH] {
    /// Element-wise AND two lists of booleans
    fn and(&self, other: [bool; LENGTH]) -> [bool; LENGTH] {
        self.zip(other).map(|(lhs, rhs)| lhs && rhs)
    }
}