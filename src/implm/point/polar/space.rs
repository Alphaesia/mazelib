use std::num::NonZeroUsize;

use embed_doc_image::embed_doc_image;
use rand::Rng;

use crate::implm::point::polar::point::PolarCoordinate;
use crate::implm::point::polar::PolarCoordinateSpaceIterator;
use crate::interface::point::CoordinateSpace;

/// A discretised polar (circular) coordinate space.
///
/// A polar coordinate space is a circle that is divided up into *rings* and *sectors*. Rings are
/// concentric circles starting at the center and emanating outwards. Sectors are slices of the
/// circle. A polar coordinate space has a set number of rings and sectors. All rings and all
/// sectors are equally-sized.
///
/// Here is an example of a polar coordinate space:
///
/// ![A diagram visually explaining polar coordinates. There are five concentric circles divided into eight sectors, but the top-left corner of the circles is missing. The spaces between the circles are labelled rings 0 to 4, where 0 is the closest to the center. The spaces between the sector lines are labelled sectors 0 to 5, starting from the sector just to the right of the top of the diagram. Sectors 6 and 7 are omitted.][polar-coordinate-space-example]
///
/// For more details on what polar coordinates are see
/// [`PolarCoordinate`][super::PolarCoordinate].
///
/// The origin of a polar coordinate space is at the point (0 ∠ 0).
///
/// # Adjacency
///
/// Two rings are adjacent if their ring numbers differ by exactly one. Two sectors are adjacent
/// if their sector numbers differ by exactly one *or* if one sector has a sector number of zero
/// and the other has a sector number of `self.sectors() - 1` (that is to say, they are the first
/// and last sectors).
///
/// Two polar coordinates are adjacent if they are in the same ring and their sectors are adjacent,
/// or if their rings are adjacent and they are in the same sector.
/// 
/// ## Adjacency Example
/// 
/// ```
/// # use mazelib::implm::point::polar::{PolarCoordinate, PolarCoordinateSpace};
/// # use mazelib::interface::point::CoordinateSpace;
/// #
/// let [rings, sectors] = [5, 7];
/// let coord_space = PolarCoordinateSpace::new_checked(rings, sectors);
///
/// assert!(coord_space.are_adjacent(
///     PolarCoordinate { ring: 0, sector: 0 },
///     PolarCoordinate { ring: 0, sector: 1 },  // Offset by 1
/// ));
/// 
/// assert!(coord_space.are_adjacent(
///     PolarCoordinate { ring: 0, sector: 0 },
///     PolarCoordinate { ring: 0, sector: 6 },  // Wraparound
/// ));
///
/// assert!(coord_space.are_adjacent(
///     PolarCoordinate { ring: 0, sector: 0 },
///     PolarCoordinate { ring: 0, sector: 0 },  // Identical
/// ) == false);
///
/// assert!(coord_space.are_adjacent(
///     PolarCoordinate { ring: 0, sector: 0 },
///     PolarCoordinate { ring: 0, sector: 7 },  // Out of bounds
/// ) == false);
/// ```
#[embed_doc_image("polar-coordinate-space-example", "src/doc/img/point/polar/polar-coordinate-space-example.png")]
#[derive(Copy, Clone, Debug)]
pub struct PolarCoordinateSpace {
    rings: NonZeroUsize,
    sectors: NonZeroUsize,
    size: NonZeroUsize
}

impl PolarCoordinateSpace {
    /// Construct a new `PolarCoordinateSpace` from the given dimensions.
    ///
    /// `rings * sectors` must also fit within a `usize`. (Mazes this large won't fit in memory
    /// anyway).
    ///
    /// # Parameters
    ///
    /// `rings`   --- the number of rings the circle should be divided into.  
    /// `sectors` --- the number of sectors the circle should be divided into.
    ///
    /// # Examples
    ///
    /// ```
    /// # unsafe {
    /// # use std::num::NonZeroUsize;
    /// # use mazelib::implm::point::polar::PolarCoordinateSpace;
    /// #
    /// // If you're using hard-coded constants like in this example,
    /// // you may prefer new_checked() for the ergonomics
    /// let coord_space = PolarCoordinateSpace::new(NonZeroUsize::new_unchecked(5), NonZeroUsize::new_unchecked(7));
    /// # }
    /// ```
    ///
    /// # See Also
    ///
    /// [`new_checked()`][Self::new_checked]
    #[must_use]
    pub fn new(rings: NonZeroUsize, sectors: NonZeroUsize) -> Self {
        let size = rings.checked_mul(sectors).expect("The dimensions specified are too large. The number of points in the space does not fit within a usize.");

        Self { rings, sectors, size }
    }

    /// Construct a new `PolarCoordinateSpace` from the given dimensions.
    ///
    /// `rings * sectors` must also fit within a `usize`. (Mazes this large won't fit in memory
    /// anyway).
    ///
    /// # Parameters
    ///
    /// `rings`   --- the number of rings the circle should be divided into.  
    /// `sectors` --- the number of sectors the circle should be divided into.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::polar::PolarCoordinateSpace;
    /// #
    /// let coord_space = PolarCoordinateSpace::new_checked(5, 7);
    /// ```
    #[must_use]
    pub fn new_checked(rings: usize, sectors: usize) -> Self {
        Self::new(NonZeroUsize::new(rings).expect("rings must be non-zero"), NonZeroUsize::new(sectors).expect("sectors must be non-zero"))
    }

    /// Return the number of rings in this coordinate space.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::polar::PolarCoordinateSpace;
    /// #
    /// let [rings, sectors] = [5, 7];
    /// let coord_space = PolarCoordinateSpace::new_checked(rings, sectors);
    /// 
    /// assert_eq!(rings, coord_space.rings().into());
    #[must_use]
    pub fn rings(&self) -> NonZeroUsize {
        self.rings
    }

    /// Return the number of sectors in this coordinate space.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::polar::PolarCoordinateSpace;
    /// #
    /// let [rings, sectors] = [5, 7];
    /// let coord_space = PolarCoordinateSpace::new_checked(rings, sectors);
    ///
    /// assert_eq!(sectors, coord_space.sectors().into());
    #[must_use]
    pub fn sectors(&self) -> NonZeroUsize {
        self.sectors
    }
}

impl CoordinateSpace for PolarCoordinateSpace {
    type PtType = PolarCoordinate;
    type Iter = PolarCoordinateSpaceIterator;

    fn logical_size(&self) -> NonZeroUsize {
        self.size
    }

    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType> {
        let mut neighbours = Vec::with_capacity(4);

        let sectors_minus_one = usize::from(self.sectors) - 1;

        if pt.ring > 0 {
            neighbours.push(pt.offset_ring(-1))
        }

        if pt.ring < sectors_minus_one {
            neighbours.push(pt.offset_ring(1))
        }

        if pt.sector == 0 {
            neighbours.push(pt.with_sector(sectors_minus_one))
        } else {
            neighbours.push(pt.offset_sector(-1))
        }

        if pt.sector == sectors_minus_one {
            neighbours.push(pt.with_sector(0))
        } else {
            neighbours.push(pt.offset_sector(1))
        }

        return neighbours
    }

    fn are_adjacent(&self, pt1: Self::PtType, pt2: Self::PtType) -> bool {
        // Only sectors wrap - rings don't

        return if pt1.ring == pt2.ring {
            pt1.sector.abs_diff(pt2.sector) == 1
                || pt1.sector == 0 && pt2.sector == usize::from(self.sectors) - 1
                || pt1.sector == usize::from(self.sectors) - 1 && pt2.sector == 0
        } else if pt1.sector == pt2.sector {
            pt1.ring.abs_diff(pt2.ring) == 1
        } else {
            false
        }
    }

    fn iter(&self) -> Self::Iter {
        PolarCoordinateSpaceIterator::new(*self, None)
    }

    fn iter_from(&self, pt: Self::PtType) -> Self::Iter {
        PolarCoordinateSpaceIterator::new(*self, Some(pt))
    }

    fn choose(&self, rng: &mut (impl Rng + ?Sized)) -> Self::PtType {
        let ring = rng.gen_range(0..self.rings.into());

        let sector = rng.gen_range(0..self.sectors.into());

        return Self::PtType { ring, sector }
    }
}