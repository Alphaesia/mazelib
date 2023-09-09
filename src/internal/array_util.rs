use std::mem::MaybeUninit;
use std::num::NonZeroUsize;

use crate::internal::util::NONZERO_USIZE_ONE;

pub trait Sum {
    type Output;

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    fn and(&self, other: &T) -> T;
}

impl <const LENGTH: usize> And<[bool; LENGTH]> for [bool; LENGTH] {
    /// Element-wise AND two lists of booleans
    fn and(&self, other: &[bool; LENGTH]) -> [bool; LENGTH] {
        self.zip_map(other, |lhs, rhs| *lhs && *rhs)
    }
}

pub trait ArrayZipMap<T, const LENGTH: usize> {
    #[must_use]
    fn zip_map<U, R>(&self, other: &[U; LENGTH], map: fn(&T, &U) -> R) -> [R; LENGTH];
}

impl <T, const LENGTH: usize> ArrayZipMap<T, LENGTH> for [T; LENGTH] {
    fn zip_map<U, R>(&self, other: &[U; LENGTH], map: fn(&T, &U) -> R) -> [R; LENGTH] {
        /*
         * SAFETY:
         * Safe because we assign to every element in the array in the loop below
         * and don't read from it beforehand. The array and for-loop clearly have
         * the same size.
         *
         * The reason why we use this unsafe method is because we can't create the
         * result array if R does not implement Default. And because we use NonZero*
         * a lot, which doesn't implement it, we must create an uninitialised array.
         */
        let mut result: [R; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..LENGTH {
            result[i] = map(&self[i], &other[i]);
        }

        return result;
    }
}