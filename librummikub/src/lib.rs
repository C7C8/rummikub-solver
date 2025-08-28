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
use log::trace;

/**********
 * COLORS *
 **********/

/// The color of a tile.
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Red => 'R',
            Color::Blue => 'B',
            Color::Yellow => 'Y',
            Color::Black => 'X'
        }.to_string())
    }
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

impl Tile {
    pub fn validate(&self) -> Result<(), String> {
        trace!("Validating tile {}", self);
        if self.value > 13 {
            Err(format!("Tile value {} exceeds maximum 13", self.value))
        } else {
            Ok(())
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value_code = if self.value == 0 { String::from('*') } else { self.value.to_string() };
        write!(f, "{}{}", self.color, value_code)
    }
}

/*********
 * MOVES *
 *********/

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

        /// Index of the split location. The split will be *after* the location.
        location: u8
    },

    Add {
        /// The index of the line in the board
        line_idx: u8,

        /// Tile to add
        tile: Tile,

        /// Where to add the tile - on the beginning or the end
        location: MoveLocation,
    },

    Remove {
        /// The index of the line in the board
        line_idx: u8,

        /// Where to remove the tile - the beginning or the end
        location: MoveLocation
    }
}

impl Move {
    fn validate(&self, board: &Board) -> Result<(), String> {
        trace!("Validating move {}", self);
        match self {
            Move::Split { line_idx, location} => {
                Ok(())
            }
            Move::Add { location, line_idx, tile } => {
                Ok(())
            }
            Move::Remove { line_idx, location} => {
                Ok(())
            }
        }
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

/*********
 * LINES *
 *********/

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

impl Line {
    /// Validate that is line meets the game rules
    fn validate(&self) -> Result<(), String> {
        // TODO implement
        Ok(())
    }

    fn validate_move(&self, r#move: &Move) -> Result<(), String> {
        Ok(())
    }

    fn execute_move(&self, r#move: &Move) -> Result<(), String> {
        // TODO implement
        Ok(())
    }
}

/**********
 * BOARDS *
 **********/

pub struct Board {
    pub lines: Vec<Line>,
    pub history: Vec<Sequence>
}

impl Board {
    /// Validate that the board is in a good state according to the game rules.
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// Validate that a given move can be executed -- **not** necessarily that the board will end
    /// in a legal state at the end of that move. To confirm a *sequence* of move, see [validate_sequence]
    pub fn validate_move(&self, r#move: &Move) -> Result<(), String> {
        Ok(())
    }

    pub fn execute_move(&self, r#move: Move) -> Result<(), String> {
        Ok(())
    }

    pub fn validate_sequence(&self, sequence: &Sequence) -> Result<(), String> {
        Ok(())
    }

    pub fn execute_sequence(&self, sequence: Sequence) -> Result<(), String> {
        Ok(())
    }
}

/********
 * MISC *
 ********/

/// List of tiles
pub type Hand = Vec<Tile>;
/// List of moves in a sequence to complete a player's turn
pub type Sequence = Vec<Move>;