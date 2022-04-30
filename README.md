# mazelib

A flexible and ambitious W.I.P. library to create, solve, and analyse mazes of all varieties. *mazelib* (will be able to) handle square mazes, circular mazes, mazes of other shapes in two-dimensions, three-dimensions, and more.

## Documentation

The latest rustdoc can be found on GitHub pages at <https://alphaesia.github.io/mazelib/mazelib/index.html>.

## Planned Features

### Maze Types

Support:

* Tessellations: square (Γ), triangular (Δ), circular (Θ), hexagonal (Σ), orthogonal and diagonal (Ζ), and octagonal (Υ).
* Crack mazes (no tesselation).
* Weave mazes (2D mazes where passages can overlap).
* Directed mazes (one-way passages).
* Higher dimensional mazes (3D, 4D, etc.) where possible.
* Planar mazes (mazes projected onto the surfaces of objects, like spheres).
* Pure-graph mazes (no higher level structure).
* Partial mazes / infinite mazes.
* Custom templates / maze boundaries (like those "Can you escape the dinosaur?" mazes).

### Generation

Generating:
* Perfect mazes (no loops)
* Braid mazes (no dead-ends)
* Partial-braids / multiply-connected mazes
* Unicursal mazes (no junctions)
* Sparse mazes (mazes with empty ares)

All standard maze generators:
* Recursive Backtracker
* Hunt-and-Kill (my favourite)
* Sidewinder
* NAry Tree (generalisation of Binary Tree)
* Growing Tree and Growing Forest
* Prim's (true, simplified, and modified)
* Kruskal's
* Wilson's
* Aldous-Broder's
* Eller's
* Recursive Division'

Support end-users creating custom generators.

### Analysis

Analysis of mazes and their solutions.

Details to be determined, but for example:
* Dead End (count and %)
* Biasness (non-randomness)
* Elitism (solution path length vs maze size)

### Solving

All standard maze solvers:
* Random Mouse
* Wall Follower
* Pledge's
* Trémaux's
* BFS
* A*

Finding any solution path, and the shortest path.

Support end-users creating custom solvers.

### Rendering

Render mazes to a variety of formats, such as:
* Text (Unicode)
* Images (PNG, Bitmap, etc.)
* [Sponge Schematics](https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-3.md)

Support end-users creating custom renderers.

### Performance

The library strives to be as performant as reasonably possible. It should be able to handle multi-gigabyte mazes with millions of cells efficiently and quickly. However, performance-at-all-costs is not a goal, and flexibility is prioritised.
