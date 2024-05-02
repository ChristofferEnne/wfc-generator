use crate::tiles::tile::Tile;

// The tilemaker is a tool used to read in base tiles from a readable format.
// It then creates all the alternate tiles such as rotated and flipped tiles.
// A user can then use this instead of having to manually create all tiles as 
// this is both unreliable due to human error and tedious.