//! Generating mazes.
//!
//! As an abstraction, generating mazes is simple. You construct a generator (possibly with
//! a few parameters), then hand off your maze to it. Once the generator returns, you have
//! your generated maze.
//!
//! # Recommended Reading
//!
//! 1. [`MazeGenerator`] --- the generator interface.
//! 2. [`crate::implm::generate`] --- a comparison of the builtin generators.

use crate::interface::cell::CellManager;

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
/// Most implementations will define a static `Implementation::generate(maze: &mut Maze)`
/// that will generate a maze using some default generator parameters. It also saves you
/// having to construct the generator struct yourself.
pub trait MazeGenerator<Maze: CellManager> {
    /// Generate a random maze.
    ///
    /// Mazes are operated upon in-place. Should for whatever reason a generator panic
    /// during execution, the maze may be left in a partially-generated state
    /// (different from what it started as). It will however not be left in an inconsistent
    /// state (i.e. it would be unsafe to read), unless the panic originates from within
    /// the [`CellManager`].
    ///
    /// # Parameters
    /// * `maze` --- the maze to be filled in. The maze may be partially or completely
    ///              filled in beforehand. The generator will consider any existing
    ///              points and work them into its generation. Any
    ///              <abbr title="A connection between points">edge</abbr> that is not
    ///              a boundary may be overwritten as part of the generation process.
    fn generate(&mut self, maze: &mut Maze) {
        self.generate_with_rng(maze, &mut rand::thread_rng());
    }

    /// Generate a maze using a given random number generator.
    ///
    /// Mazes are operated upon in-place. Should for whatever reason a generator panic
    /// during execution, the maze may be left in a partially-generated state
    /// (different from what it started as). It will however not be left in an inconsistent
    /// state (i.e. it would be unsafe to read), unless the panic originates from within
    /// the [`CellManager`].
    ///
    /// You should prefer [`generate()`][Self::generate`] to this method.
    ///
    /// # Parameters
    /// * `maze` --- the maze to be filled in. The maze may be partially or completely
    ///              filled in beforehand. The generator will consider any existing
    ///              points and work them into its generation. Any
    ///              <abbr title="A connection between points">edge</abbr> that is not
    ///              a boundary may be overwritten as part of the generation process.
    /// * `rng`  --- The sole source of randomness for a generator. Given a
    ///              [`rand::SeedableRng`] with a fixed seed, the generator's behaviour
    ///              is deterministic.
    fn generate_with_rng<R: rand::Rng + ?Sized>(&mut self, maze: &mut Maze, rng: &mut R);
}