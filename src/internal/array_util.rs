pub trait Sum<T> {
    fn sum(&self) -> T;
}

impl <const LENGTH: usize> Sum<usize> for [usize; LENGTH] {
    /// Return the sum of all elements in an array.
    fn sum(&self) -> usize {
        let mut sum = 0;

        for x in self {
            sum += x;
        }

        return sum;
    }
}

pub trait Product<T> {
    fn product(&self) -> T;
}

impl <const LENGTH: usize> Product<usize> for [usize; LENGTH] {
    /// Return the product of all elements in an array.
    /// Returns 1 when LENGTH == 0.
    fn product(&self) -> usize {
        let mut product = 1;

        for x in self {
            product *= x;
        }

        return product;
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