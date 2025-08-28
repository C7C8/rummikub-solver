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
use crate::{Color, Move, MoveLocation, Tile};

/*********
 * TILES *
 *********/

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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value_code = if self.value == 0 { String::from('*') } else { self.value.to_string() };
        write!(f, "{}{}", self.color, value_code)
    }
}

/*********
 * MOVES *
 *********/

impl Display for MoveLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveLocation::Beginning=> f.write_char('<'),
            MoveLocation::End => f.write_char('>')
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