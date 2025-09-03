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
use std::collections::HashSet;
use log::{debug, trace};
use crate::{Board, Color, Line, LineType, Move, Sequence, Tile};

/*********
 * TILES *
 *********/

impl Tile {
    /// Validate that this tile obeys the game rules.
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
 * LINES *
 *********/

impl Line {
    /// Validate that this line meets the game rules
    pub fn validate(&self) -> Result<(), String> {
        debug!("Validating line {}", self);

        // Lines must be at least 3 long
        if self.tiles.len() < 3 {
            return Err(format!("Line length {} too short, must be at least 3", self.tiles.len()));
        }

        // Validate each tile individually - valid lines cannot have invalid tiles.
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Err(e) = tile.validate() {
                return Err(format!("Invalid tile at index {}: {}", i, e))
            }
        }

        // Line-type-specific validation
        match self.r#type {
            LineType::SingleNumber => {
                // Single-number lines must be all different colors but the same number. They can
                // have wildcards however, so in case the first item is a wildcard we must identify
                // the single number to check for.
                let mut number: u8 = 0;
                for tile in self.tiles.iter() {
                    if tile.value != 0 {
                        number = tile.value;
                        break;
                    }
                }
                trace!("Single-number line {} has number {}", self, number);

                // TODO Optimize to remove second iteration? Low priority.
                let mut found_colors: HashSet<Color> = HashSet::new();
                for (i, tile) in self.tiles.iter().enumerate() {
                    // Check to make sure the number is as expected - or a wildcard
                    if tile.value != number && tile.value != 0 {
                        return Err(format!("Tile at index {} has value {} but {} was expected", i, tile.value, number));
                    }

                    // Now check if the color has been seen already. If it has, error.
                    if found_colors.contains(&tile.color) {
                        return Err(format!("Tile at index {} has duplicate color {}", i, tile.color));
                    }
                    found_colors.insert(tile.color);
                }

                // Line validated!
                Ok(())
            }
            LineType::NumberSequence => {
                // Identify the color. We'll identify the starting number on the fly.
                let color = self.tiles.first().unwrap().color;
                let mut expected_number: u8 = 0;

                for (i, tile) in self.tiles.iter().enumerate() {
                    if expected_number == 0 && tile.value != 0 {
                        trace!("Identified starting number for line {} as {}", self, tile.value);
                        expected_number = tile.value;
                    }

                    // Color check
                    if tile.color != color {
                        return Err(format!("Tile at index {} has color {} but {} was expected", i, tile.color, color));
                    }

                    if tile.value != 0 && tile.value != expected_number {
                        return Err(format!("Tile at index {} was expected to have number {} but had {}", i, expected_number, tile.value));
                    }

                    if expected_number != 0 {
                        expected_number += 1
                    };
                }

                if expected_number == 0 {
                    return Err("Line is all wildcards".to_string());
                }

                // Line validated!
                Ok(())
            }
        }
    }

    /// Validate that this move can be performed against this line
    pub fn validate_move(&self, r#move: &Move) -> Result<(), String> {
        Ok(())
    }

    /// Execute a given move against this line. In the event of a split move, this will return a
    /// second line that should be added to the board.
    pub fn execute_move(&self, r#move: &Move) -> Result<Option<Line>, String> {
        // TODO implement
        Ok(None)
    }
}

/**********
 * BOARDS *
 **********/

impl Board {
    /// Validate that the board is in a good state according to the game rules.
    pub fn validate(&self) -> Result<(), String> {
        trace!("Validating board {}", self);

        // Validate tiles
        for (i, line) in self.lines.iter().enumerate() {
            if let Err(e) = line.validate() {
                return Err(format!("Invalid line at index {}: {}", i, e));
            }
        }

        Ok(())
    }

    /// Validate that a given move can be executed -- **not** necessarily that the board will end
    /// in a legal state at the end of that move. To confirm a *sequence* of move, see [Board::validate_sequence]
    pub fn validate_move(&self, r#move: &Move) -> Result<(), String> {
        // TODO Implement
        Ok(())
    }

    pub fn validate_sequence(&self, sequence: &Sequence) -> Result<(), String> {
        // TODO Implement
        Ok(())
    }
}