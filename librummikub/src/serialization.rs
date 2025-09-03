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
use crate::{Board, Color, Line, LineType, Move, MoveLocation, Tile};
use std::fmt::{Display, Formatter, Write};

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
 * LINES *
 *********/

impl Display for Line {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.r#type {
			LineType::SingleNumber => write!(f, "S:")?,
			LineType::NumberSequence => write!(f, "M:")?,
		}

		for (i, tile) in self.tiles.iter().enumerate() {
			write!(f, "{}", tile)?;
			if i < self.tiles.len() - 1 {
				write!(f, "-")?;
			}
		}

		Ok(())
	}
}

/**********
 * BOARDS *
 **********/

impl Display for Board {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		// TODO Reconsider omission of board history from the serialization
		write!(f, "[")?;
		for (i, line) in self.lines.iter().enumerate() {
			write!(f, "{}", line)?;
			if i < self.lines.len() - 1 {
				write!(f, "; ")?;
			}
		}

		write!(f, "]")?;

		Ok(())
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
			Move::SplitLine { line_idx, location } => {
				write!(f, "{}s{}", line_idx, location)
			}
			Move::AddTile { line_idx, tile, location } => {
				write!(f, "{}a{}{}", line_idx, location, tile)
			}
			Move::RemoveTile { line_idx, location } => {
				write!(f, "{}r{}", line_idx, location)
			}
			Move::CreateLine { tiles } => {
				write!(f, "-c-")?;
				for (i, tile) in tiles.iter().enumerate() {
					write!(f, "{}", tile)?;

					if i < tiles.len() - 1 {
						write!(f, "-")?;
					}
				}
				Ok(())
			}
		}
	}
}