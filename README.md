# LevelSpecter
An implementation of levelspec - a representation of show.sequence.shot which may be:

- show
- show.sequence
- show.sequence.shot

Each level may either be a name or a wildcard (`%`).

## Naming Rules

- By default, the show and sequence must be uppercase, and start and end with a letter. 
- Shots must be numeric, except for the special case, where they follow the `ASSETDEV` sequence, 
in which case they can be alphanumeric.

## Case Insensitivity
The crate may be made case insensitive by using the "case-insensitive" feature. This may be set
in the consuming crate's Cargo.toml, as a `feature = "case-insensitive", or as a cargo flag,
`--feature case-insensitive`, if building or testing the crate directly.
