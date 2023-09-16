//! Abstract positioning in mazes.
//!
//! When analysing mazes (generating them, solving them, etc.), it's helpful to think only in terms
//! of potential junctions, rather than stepping cell-by-cell throughout the maze. Points represent
//! the positions in the maze that are meaningful for analysis.
//!
//! # Recommended Reading
//!
//! 1. [`CoordinateSpace`] --- the trait that defines the logical size of mazes.
//! 2. [`crate::interface::cell::ConnectionType`] --- to see the different ways points can connect.

use std::fmt::Debug;
use std::hash::Hash;
use std::num::NonZeroUsize;

use rand::Rng;

/// A mathematical graph that represents all possible meaningful positions in a maze.
///
/// It defines where potential junctions are and how they connect to other potential
/// junctions. It does not encode any information about passages or walls, only the
/// locations where they *could* go.
///
/// For a more in-depth explanation, [see the explainer in `interface`](crate::interface#coordinate-space).
///
/// A coordinate space must always have at least one point. It can never be empty.
///
/// # Examples
///
/// Constructing a maze with a specific coordinate space:
///
/// ```
/// #![feature(generic_arg_infer)]  // Optional, lets the compiler infer whether it's 2D, 3D, etc.
///
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::coordinator::block::BoxSpaceBlockCellMazeCoordinator;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// #
/// # type Buffer = VecBuffer<BlockCellValue>;
/// #
/// let dimensions = [7, 7];
///
/// let space = BoxCoordinateSpace::new_checked(dimensions);
///
/// // Will need to pick your own buffer
/// let maze = BoxSpaceBlockCellMazeCoordinator::<Buffer, _>::builder(space).build();
/// ```
///
/// # See Also
///
/// [`Point`] -- the individual parts of a coordinate space
pub trait CoordinateSpace : Sized + Clone + Copy + Send + Sync + Debug {
    /// The [`Point`] type that goes with this coordinate space.
    type PtType: Point;

    /// The type of iterator returned by [`iter()`][Self::iter] and
    /// [`iter_from()`][Self::iter_from].
    type Iter: Iterator<Item = Self::PtType>;

    /// Return the number of points in this coordinate space.
    ///
    /// This value is fixed.
    ///
    /// As the coordinate space must always have at least one point, this method
    /// returns `NonZeroUsize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// // A 3x3 space has 9 points
    /// let space = BoxCoordinateSpace::new_checked([3, 3]);
    ///
    /// assert_eq!(9, usize::from(space.logical_size()));
    /// ```
    #[must_use]
    fn logical_size(&self) -> NonZeroUsize;

    /// Return a vector containing every point in the coordinate space
    /// that is adjacent to `pt` (as determined by [`are_adjacent()`](Self::are_adjacent)).
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinatePair};
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let space = BoxCoordinateSpace::new_checked([3, 3]);
    ///
    /// /*
    ///  * Point = X
    ///  * Neighbours = *
    ///  *
    ///  * ┌───┬───┬───┐
    ///  * │   │   │   │
    ///  * ├───┼───┼───┤
    ///  * │   │ * │   │
    ///  * ├───┼───┼───┤
    ///  * │ * │ X │ * │
    ///  * └───┴───┴───┘
    ///  */
    ///
    /// let pt = (1, 0).into();
    ///
    /// let expected_neighbours: Vec<CoordinatePair> = vec![
    ///     (0, 0).into(),
    ///     (2, 0).into(),
    ///     (1, 1).into()
    /// ];
    ///
    /// assert_eq!(expected_neighbours, space.neighbours_of_pt(pt));
    /// ```
    #[must_use]
    fn neighbours_of_pt(&self, pt: Self::PtType) -> Vec<Self::PtType>;

    /// Return whether two points are adjacent in this coordinate space.
    ///
    /// A point is **not** adjacent to itself.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinatePair};
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let space = BoxCoordinateSpace::new_checked([3, 3]);
    ///
    /// // Adjacent
    /// assert!(space.are_adjacent((1, 0).into(), (0, 0).into()));
    /// assert!(space.are_adjacent((1, 0).into(), (1, 1).into()));
    /// assert!(space.are_adjacent((1, 0).into(), (2, 0).into()));
    ///
    /// // Not adjacent
    /// assert!(!space.are_adjacent((1, 0).into(), (0, 1).into()));
    /// assert!(!space.are_adjacent((1, 0).into(), (2, 2).into()));
    /// assert!(!space.are_adjacent((1, 0).into(), (1, 2).into()));  // No wrapping
    /// ```
    #[must_use]
    fn are_adjacent(&self, pt1: Self::PtType, pt2: Self::PtType) -> bool;

    /// Return an iterator that yields every point in this coordinate space.
    ///
    /// Every point yielded is adjacent to a point previously yielded.
    /// The one exception to this is where there is no possible path from the
    /// current point to any previously yielded point. For example, the first
    /// point, or if the space contains two or more isolated components, the
    /// first point of each component.
    ///
    /// If the coordinate space has any notion of an origin, it is the first
    /// point yielded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let space = BoxCoordinateSpace::new_checked([2, 2]);
    ///
    /// let mut iter = space.iter();
    ///
    /// assert_eq!(Some((0, 0).into()), iter.next());
    /// assert_eq!(Some((1, 0).into()), iter.next());
    /// assert_eq!(Some((0, 1).into()), iter.next());
    /// assert_eq!(Some((1, 1).into()), iter.next());
    ///
    /// assert_eq!(None, iter.next());
    /// ```
    #[must_use]
    fn iter(&self) -> Self::Iter;

    /// Return an iterator that behaves exactly like [`iter()`](Self::iter),
    /// but skips every point up to and including `pt`.
    ///
    /// Points that are skipped are still considered to have been yielded for
    /// the purposes of the path constraint in `iter()`.
    ///
    /// # Examples
    ///
    /// Contrast this example with the example for `iter()`.
    /// ```
    /// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let space = BoxCoordinateSpace::new_checked([2, 2]);
    ///
    /// let mut iter = space.iter_from((1, 0).into());
    ///
    /// assert_eq!(Some((0, 1).into()), iter.next());
    /// assert_eq!(Some((1, 1).into()), iter.next());
    ///
    /// assert_eq!(None, iter.next());
    /// ```
    #[must_use]
    fn iter_from(&self, pt: Self::PtType) -> Self::Iter;

    /// Return a random point in this coordinate space.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand::thread_rng;
    /// # use mazelib::implm::point::boxy::{BoxCoordinateSpace, CoordinatePair};
    /// # use mazelib::interface::point::CoordinateSpace;
    /// #
    /// let mut rng = thread_rng();
    ///
    /// let space = BoxCoordinateSpace::new_checked([3, 3]);
    ///
    /// let pt = space.choose(&mut rng);
    /// ```
    #[must_use]
    fn choose(&self, rng: &mut (impl Rng + ?Sized)) -> Self::PtType;
}

/// A logical location in a maze.
///
/// In less abstract terms, points are basically the potential junctions
/// in a maze.
///
/// Unlike cells, points have semantic meaning, and it is the connection
/// of points (by the [`CoordinateSpace`]) that determine the logical
/// structure of the maze.
///
/// A detailed introduction can be found in the
/// [section on points in the core maze components explanation][crate::interface#point].
///
/// # See Also
///
/// * [`CoordinateSpace`] -- the parent container of points.
/// * [`CellLocation`][crate::interface::cell::CellLocation] -- physical (rather than logical) locations in a maze.
///
/// # Implementation
///
/// With regards to implementation, points are or similar to coordinate
/// tuplets (e.g. `(x, y)`). Points do not have to be coordinates, but
/// must effectively act like them. More specifically, they must have the
/// following properties:
/// * Immutable
/// * Fixed Size
/// * Cloneable
/// * Equality must be reflexive, symmetric, transitive, *and anti-symmetric*
/// * Hashable
pub trait Point: Sized + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + Debug {}