use std::fmt::Debug;

pub trait CellData: Copy + Clone + Send + Sync + Default + PartialEq + Eq + Debug {
    fn is_passage(&self) -> bool;
    fn is_wall(&self) -> bool;
    fn is_boundary(&self) -> bool;
    fn is_unvisited(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Basic {
    UNVISITED,
    BOUNDARY,
    WALL,
    PASSAGE
}

impl CellData for Basic {
    fn is_passage(&self) -> bool { self == &Self::PASSAGE }
    fn is_wall(&self) -> bool { self == &Self::WALL }
    fn is_boundary(&self) -> bool { self == &Self::BOUNDARY }
    fn is_unvisited(&self) -> bool { self == &Self::UNVISITED }
}

impl Default for Basic {
    fn default() -> Self { Self::UNVISITED }
}
