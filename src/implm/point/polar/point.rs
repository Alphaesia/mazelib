use std::fmt::{Debug, Display, Formatter};

use embed_doc_image::embed_doc_image;

use crate::interface::point::Point;
use crate::internal::util::offset_usize;

/// A polar coordinate.
///
/// A polar coordinate has two components: the radial coordinate (the radius) and the angular
/// coordinate (the angle). Since our [polar coordinate spaces][super::PolarCoordinateSpace] are
/// discretised, here we refer to these coordinate components as the *ring* and *sector*
/// respectively. The acceptable ranges for these coordinates are `0 <= ring < space.rings()` and
/// `0 <= sector < space.sectors()`. Sector coordinates beyond the coordinate space's sector count
/// will not be automatically wrapped around and will be considered out-of-bounds.
///
/// Polar coordinates are often written out as (`<ring>` ∠ `<sector>`).
///
/// Here's an example of a polar coordinate space with 5 rings and 8 sectors:
///
/// ![A diagram visually explaining polar coordinates. There are five concentric circles divided into eight sectors, but the top-left corner of the circles is missing. The spaces between the circles are labelled rings 0 to 4, where 0 is the closest to the center. The spaces between the sector lines are labelled sectors 0 to 5, starting from the sector just to the right of the top of the diagram. Sectors 6 and 7 are omitted. The cell in the fourth ring and second sector is highlighted in blue and annotated with "(3 ∠ 1)".][polar-coordinate-example]
///
/// The adjacency of two polar coordinates cannot be determined by just the coordinates alone. The
/// first and last sectors are adjacent to each other, and to know what sector is last requires the
/// coordinate space.
/// [See the coordinate space for the details on adjacency](super::PolarCoordinateSpace#adjacency).
/// 
/// # Examples
/// 
/// It's just a regular struct with all fields public, so you can construct it directly.
/// 
/// ```
/// # use mazelib::implm::point::polar::PolarCoordinate;
/// # 
/// PolarCoordinate { ring: 3, sector: 2 };
/// ```
#[embed_doc_image("polar-coordinate-example", "src/doc/img/point/polar/polar-coordinate-example.png")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolarCoordinate {
    /// The ring coordinate.
    pub ring: usize,

    /// The sector coordinate.
    pub sector: usize,
}

impl Point for PolarCoordinate {}

impl PolarCoordinate {
    /// Return a new polar coordinate with the given ring coordinate and `self`'s sector
    /// coordinate.
    #[must_use]
    pub fn with_ring(&self, ring: usize) -> Self {
        Self { ring, sector: self.sector }
    }

    /// Return a new polar coordinate with `self`'s ring coordinate and the given sector
    /// coordinate.
    #[must_use]
    pub fn with_sector(&self, sector: usize) -> Self {
        Self { ring: self.ring, sector }
    }

    /// Return a new polar coordinate with `self`'s ring coordinate, and `self`'s sector coordinate
    /// summed with the given offset.
    #[must_use]
    pub fn offset_ring(&self, offset: isize) -> Self {
        self.with_ring(offset_usize(self.ring, offset))
    }

    /// Return a new polar coordinate with `self`'s ring coordinate summed with the given offset,
    /// and `self`'s sector coordinate.
    #[must_use]
    pub fn offset_sector(&self, offset: isize) -> Self {
        self.with_sector(offset_usize(self.sector, offset))
    }
}

impl Debug for PolarCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ∠ {})", self.ring, self.sector)
    }
}

impl Display for PolarCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}