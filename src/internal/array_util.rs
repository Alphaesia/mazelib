pub trait Product<T> {
    fn product(&self) -> T;
}

impl <const LENGTH: usize> Product<usize> for [usize; LENGTH] {
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
    fn and(&self, other: [bool; LENGTH]) -> [bool; LENGTH] {
        self.zip(other).map(|(lhs, rhs)| lhs && rhs)
    }
}