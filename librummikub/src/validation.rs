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
use log::trace;
use crate::{Board, Line, Move, Sequence, Tile};

/* *******
 * TILES *
 *********/

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

/*********
 * MOVES *
 *********/

impl Move {
    fn validate(&self, board: &Board) -> Result<(), String> {
        trace!("Validating move {}", self);
        //TODO Implement
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

/*********
 * LINES *
 *********/

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

impl Board {
    /// Validate that the board is in a good state according to the game rules.
    pub fn validate(&self) -> Result<(), String> {
        // TODO Implement
        Ok(())
    }

    /// Validate that a given move can be executed -- **not** necessarily that the board will end
    /// in a legal state at the end of that move. To confirm a *sequence* of move, see [validate_sequence]
    pub fn validate_move(&self, r#move: &Move) -> Result<(), String> {
        // TODO Implement
        Ok(())
    }

    pub fn validate_sequence(&self, sequence: &Sequence) -> Result<(), String> {
        // TODO Implement
        Ok(())
    }
}