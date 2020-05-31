# Html Types

Html data structures for rust.

# Motivation

To provide data types that will make it easier to test views. Ultimately allowing render functions
that return Nodes or similar to be compared against data structures, rather than strings.

This could also be helpful when constructing future templating/macro based html generators.
As they could focus on building these trees, rather than string types, leaving rendering to 
something else.

# Design

Currently the types try to be as open as possible, allowing the end-user to manipulate the tree
however they see fit. However they will not be able to create a tree that is invalid

Currently implements a general shape over html, covering text nodes, comment nodes, 
and element nodes. Element nodes can be void or non-void.

See tests for usage examples

Also note that rendering uses the public interfaces entirely, so the tree can be rendered
to other data types if necessary, though a string rendering module is included.

# TODO

1. Currently tags need more types fleshed out (eg the compile time constants)
2. Pretty printing string rendering
3. Semantic HTML models, eg another tree saying ol must only have li elements etc.
   Which would lead to not just the correct shape, but the correct intent


