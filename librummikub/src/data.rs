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
use std::fmt::{Display, Formatter, Write};

/// The color of a tile.
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black
}

impl Color {
    pub fn to_code(&self) -> char {
        match self {
            Color::Red => 'R',
            Color::Blue => 'B',
            Color::Yellow => 'Y',
            Color::Black => 'X'
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_code().to_string())
    }
}

/// An individual, colored tile. Wildcards are represented with a value of 0.
pub struct Tile {
    /// Numerical value of the tile; 0 is wildcard.
    pub value: u8,
    pub color: Color
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value_code = if self.value == 0 { String::from('*') } else { self.value.to_string() };
        write!(f, "{}{}", self.color, value_code)
    }
}

/// Where a tile is being added or removed
pub enum MoveLocation {
    Beginning,
    End
}

impl Display for MoveLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveLocation::Beginning=> f.write_char('<'),
            MoveLocation::End => f.write_char('>')
        }
    }
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

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Split { line_idx, location } => {
                write!(f, "{}s{}", line_idx, location)
            }
            Move::Add { line_idx, tile, location } => {
                write!(f, "{}a{}{}", line_idx, location, tile)
            }
            Move::Remove { line_idx, location } => {
                write!(f, "{}r{}", line_idx, location)
            }
        }
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
