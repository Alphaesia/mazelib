//! Maze carriers.
//!
//! Maze carriers are objects that have the [five core components][super]
//! of a maze.
//!
//! # Recommended Reading
//! 1. [`Maze`] -- the maze carrier trait.
//! 2. [`CellManager`] -- the primary interface of a maze carrier
//!    (`CellManager` is a supertrait of `Maze`).

use crate::interface::cell::CellManager;

/// A maze carrier. Holds [all the components of a maze][crate::interface].
///
/// # Background
///
/// While all cell managers are maze carriers because they
/// have all the components of a maze (the coordinate space,
/// buffer, and themselves), there is a logical difference
/// between them. Cell managers are just one component performing
/// a specific task. The fact that they are also maze carriers
/// and act as the primary interface for a maze is just a
/// quirk. Consequently, it can be useful to model them as
/// both a component and as a maze carrier, hence the traits
/// are separate.
///
/// While maze carriers could have been implemented as a
/// struct wrapping a cell manager, that would have just
/// created needless ceremony and friction. With just an
/// sub-trait, we automatically inherit the cell manager's
/// interface (which as previously noted is a good public
/// interface for mazes). We can also create a
/// blanket implementation for it.
///
/// # Usage
///
/// You should use the `Maze` trait when you are modelling
/// a cell manager as a maze carrier, and the `CellManager`
/// trait when you are modelling it as a cell manager
/// component.
pub trait Maze : CellManager {}

impl <T: CellManager> Maze for T {}