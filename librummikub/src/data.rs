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

/// The color of a tile.
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black
}

/// An individual, colored tile. Wildcards are represented with a value of 0.
pub struct Tile {
    /// Numerical value of the tile; 0 is wildcard.
    pub value: u8,
    pub color: Color
}

/// Where a tile is being added or removed
pub enum MoveLocation {
    Start,
    End
}

pub enum Move {
    Split {
        /// The index of the line in the board
        line_idx: u8,
        location: u8
    },

    Add {
        line_idx: u8,
        tile: Tile,
        location: MoveLocation,
    },

    Remove {
        line_idx: u8,
        location: MoveLocation
    }
}

/// A line of tiles in the board, either all the same number in all different colors, or
pub type Line = Vec<Tile>;

/// List of tile sequences, that would be found on the board
pub type Board = Vec<Vec<Tile>>;

/// List of tiles
pub type Hand = Vec<Tile>;

/// List of moves in a sequence to complete a player's turn
pub type Sequence = Vec<Move>;