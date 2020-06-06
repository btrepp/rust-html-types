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

# Node/Elements

Node and Elements are general purpose, and mostly complete now. You should be
able to cover most HTML generation purposes using these, though they expect you to understand
the rules of how to properly order it.

# Semantic Module

This semantic module is still in development, but the examples show it's motivation.
Semantic elements have rules around there construction, and have attributes/children that make 
sense. 

There is still a large amount of work to do in this module, expanding it out.

This module will probably never cover 'all' use cases of html, eg strange tags/combinations
people may use, but the aim is to cover must as possible.

Note: These items are always convertible to elements
      It would be possible to convert back (with failures). Though this would be along way
      From being implemented.

# Pretty Printing

While this is optional, I currently don't plan on implementing it. This would be a good candidate 
for a Pull request.

# Testing

As everything is rust datastructures, it is easy to write tests. Ideally this will evolve
into a html library with extensive testing


