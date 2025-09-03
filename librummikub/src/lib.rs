/*
 * Copyright (c) 2025 Christopher R. Myers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
mod serialization;
mod validation;

/**********
 * COLORS *
 **********/

/// The color of a tile.
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black
}

/*********
 * TILES *
 *********/

/// An individual, colored tile. Wildcards are represented with a value of 0.
pub struct Tile {
    /// Numerical value of the tile; 0 is wildcard.
    pub value: u8,
    pub color: Color
}
/// Where a tile is being added or removed
pub enum MoveLocation {
    Beginning,
    End
}

/// A game move that can be executed. Does not necessarily leave the board in a rules-compliant
/// state after execution.
pub enum Move {

    /// Split an exisitng line into two lines at a given point.
    SplitLine {
        /// The index of the line in the board
        line_idx: u16,

        /// Index of the split location. The split will be *after* the location.
        location: u8
    },

    /// Add a tile to the beginning or end of a line.
    AddTile {
        /// The index of the line in the board
        line_idx: u16,

        /// Tile to add
        tile: Tile,

        /// Where to add the tile - on the beginning or the end
        location: MoveLocation,
    },

    /// Remove a tile at the beginning or end of a line.
    RemoveTile {
        /// The index of the line in the board
        line_idx: u16,

        /// Where to remove the tile - the beginning or the end
        location: MoveLocation
    },

    /// Create a new line entirely.
    CreateLine {
        tiles: Vec<Tile>
    }
}
/// Type of line - either a single number repeated across multiple colors, or tiles in an
/// ascending numerical sequence all sharing the same color
pub enum LineType {
    /// Single number line, i.e. multiple tiles sharing one number (or wildcards) with all different colors
    SingleNumber,

    /// Multiple number line in ascending numerical order, all in the same color.
    NumberSequence
}

/// A line of tiles in the board, either all the same number in all different colors, or all the same
/// color in an ascending sequence.
pub struct Line {
    pub tiles: Vec<Tile>,
    pub r#type: LineType,
}

/// A game 'board', representing the lines of tiles contained within and a history of move sequences.
pub struct Board {
    pub lines: Vec<Line>,
    pub history: Vec<Sequence>
}

/********
 * MISC *
 ********/

/// List of tiles
pub type Hand = Vec<Tile>;
/// List of moves in a sequence to complete a player's turn
pub type Sequence = Vec<Move>;