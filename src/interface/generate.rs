//! Generating mazes.
//!
//! As an abstraction, generating mazes is simple. You instantiate a generator then hand
//! off your [maze][crate::interface::maze::Maze] to it. Once the generator returns, you
//! have your generated maze.
//!
//! # Recommended Reading
//!
//! 1. [`MazeGenerator`] --- the generator interface.
//! 2. [`crate::implm::generate`] --- a comparison of the built-in generators.

use rand::Rng;

use crate::interface::maze::Maze;

/// A maze generator.
///
/// A generator traverses a maze's [graph][crate::interface::point::CoordinateSpace],
/// setting <abbr title="Connections between points">edges</abbr> to be passages or walls
/// as it goes. How exactly this is done differs between implementations. In fact, this
/// is the main differentiator between generators.
///
/// Generators do not consider mazes at the cellular level. They only jump from point to
/// point.
///
/// # See Also
///
/// [`DefaultMazeGenerator`] --- generate mazes without instantiating them (sugar).
pub trait MazeGenerator<M: Maze> {
    /// Generate a random maze.
    ///
    /// Mazes are operated upon in-place.
    /// # Parameters
    /// * `maze` --- the maze to be filled in. The maze may be partially or completely
    ///              filled in beforehand. The generator will consider any existing
    ///              cells and work them into its generation. Any
    ///              <abbr title="A connection between points">edge</abbr> that is not
    ///              a boundary may be overwritten as part of the generation process.
    fn generate(&mut self, maze: &mut M) {
        self.generate_with_rng(maze, &mut rand::thread_rng());
    }

    /// Generate a maze using a given random number generator.
    ///
    /// Mazes are operated upon in-place.
    ///
    /// You should prefer [`generate()`][Self::generate`] to this method.
    ///
    /// # Parameters
    /// * `maze` --- the maze to be filled in. The maze may be partially or completely
    ///              filled in beforehand. The generator will consider any existing
    ///              cells and work them into its generation. Any
    ///              <abbr title="A connection between points">edge</abbr> that is not
    ///              a boundary may be overwritten as part of the generation process.
    /// * `rng`  --- The sole source of randomness for the generator. Given a
    ///              [`rand::SeedableRng`] with a fixed seed, the generator's behaviour
    ///              is deterministic.
    fn generate_with_rng(&mut self, maze: &mut M, rng: &mut (impl Rng + ?Sized));
}

/// Simple sugar for [`MazeGenerator`]s.
///
/// Lets you elide constructing generators with parameterless constructors (specifically,
/// for generators that implement [`Default`]).
///
/// ```
/// # use mazelib::interface::generate::MazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::maze::block::BoxSpaceBlockCellMazeBuilder;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # let mut maze = BoxSpaceBlockCellMazeBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new_checked([1])).build();
/// #
/// HuntAndKillGenerator::new().generate(&mut maze);
/// ```
/// becomes
/// ```
/// # use mazelib::interface::generate::DefaultMazeGenerator;
/// # use mazelib::implm::buffer::VecBuffer;
/// # use mazelib::implm::cell::block::BlockCellValue;
/// # use mazelib::implm::generate::HuntAndKillGenerator;
/// # use mazelib::implm::maze::block::BoxSpaceBlockCellMazeBuilder;
/// # use mazelib::implm::point::boxy::BoxCoordinateSpace;
/// # let mut maze = BoxSpaceBlockCellMazeBuilder::<VecBuffer<BlockCellValue>, 1>::new(BoxCoordinateSpace::new_checked([1])).build();
/// #
/// HuntAndKillGenerator::generate(&mut maze);
/// ```
pub trait DefaultMazeGenerator<M: Maze>: MazeGenerator<M> {
    /// *See [`MazeGenerator::generate()`].*
    fn generate(maze: &mut M);

    /// *See [`MazeGenerator::generate_with_rng()`].*
    fn generate_with_rng(maze: &mut M, rng: &mut (impl Rng + ?Sized));
}

impl <M: Maze, T: MazeGenerator<M> + Default> DefaultMazeGenerator<M> for T {
    fn generate(maze: &mut M) {
        Self::default().generate(maze)
    }

    fn generate_with_rng(maze: &mut M, rng: &mut (impl Rng + ?Sized)) {
        Self::default().generate_with_rng(maze, rng)
    }
}