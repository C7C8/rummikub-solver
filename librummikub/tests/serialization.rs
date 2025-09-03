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

use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log::LevelFilter;
use librummikub::{Color, Line, LineType, Move, MoveLocation, Tile};

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

#[test]
fn tile_serialization() {
    let cases = vec![
        (Tile { value: 1, color: Color::Red}, "R1"),
        (Tile { value: 7, color: Color::Yellow}, "Y7"),
        (Tile { value: 0, color: Color::Blue}, "B*"),
        (Tile { value: 1, color: Color::Black}, "X1"),
        (Tile { value: 8, color: Color::Red}, "R8"),
        (Tile { value: 4, color: Color::Yellow}, "Y4"),
        (Tile { value: 7, color: Color::Blue}, "B7"),
        (Tile { value: 2, color: Color::Black}, "X2"),
        (Tile { value: 13, color: Color::Red}, "R13"),
        (Tile { value: 10, color: Color::Yellow}, "Y10"),
    ];

    for case in cases {
        assert_eq!(case.0.to_string(), case.1)
    }
}

#[test]
fn split_move_serialization() {
    let cases = vec![
        (Move::Split {line_idx: 0, location: 1}, "0s1"),
        (Move::Split {line_idx: 1, location: 7}, "1s7"),
        (Move::Split {line_idx: 5, location: 0}, "5s0"),
        (Move::Split {line_idx: 23, location: 100}, "23s100"),
        (Move::Split {line_idx: 100, location: 99}, "100s99")
    ];

    for case in cases {
        assert_eq!(case.0.to_string(), case.1)
    }
}

#[test]
fn add_move_serialization() {
    let cases = vec![
        (Move::Add {line_idx: 0, tile: Tile { value: 1, color: Color::Red}, location: MoveLocation::End}, "0a>R1"),
        (Move::Add {line_idx: 1, tile: Tile { value: 7, color: Color::Blue}, location: MoveLocation::Beginning}, "1a<B7"),
        (Move::Add {line_idx: 2, tile: Tile { value: 0, color: Color::Yellow}, location: MoveLocation::End}, "2a>Y*"),
        (Move::Add {line_idx: 3, tile: Tile { value: 1, color: Color::Black}, location: MoveLocation::Beginning}, "3a<X1"),
        (Move::Add {line_idx: 4, tile: Tile { value: 8, color: Color::Red}, location: MoveLocation::End}, "4a>R8"),
        (Move::Add {line_idx: 5, tile: Tile { value: 4, color: Color::Blue}, location: MoveLocation::Beginning}, "5a<B4"),
        (Move::Add {line_idx: 6, tile: Tile { value: 7, color: Color::Yellow}, location: MoveLocation::End}, "6a>Y7"),
        (Move::Add {line_idx: 7, tile: Tile { value: 2, color: Color::Black}, location: MoveLocation::Beginning}, "7a<X2"),
        (Move::Add {line_idx: 8, tile: Tile { value: 10, color: Color::Red}, location: MoveLocation::End}, "8a>R10"),
        (Move::Add {line_idx: 9, tile: Tile { value: 11, color: Color::Blue}, location: MoveLocation::Beginning}, "9a<B11"),
        (Move::Add {line_idx: 10, tile: Tile { value: 12, color: Color::Yellow}, location: MoveLocation::End}, "10a>Y12"),
        (Move::Add {line_idx: 11, tile: Tile { value: 13, color: Color::Black}, location: MoveLocation::End}, "11a>X13"),
        (Move::Add {line_idx: 12, tile: Tile { value: 0, color: Color::Red}, location: MoveLocation::Beginning}, "12a<R*"),
        (Move::Add {line_idx: 13, tile: Tile { value: 0, color: Color::Blue}, location: MoveLocation::End}, "13a>B*"),
        (Move::Add {line_idx: 14, tile: Tile { value: 0, color: Color::Yellow}, location: MoveLocation::Beginning}, "14a<Y*"),
        (Move::Add {line_idx: 15, tile: Tile { value: 1, color: Color::Black}, location: MoveLocation::End}, "15a>X1"),
        (Move::Add {line_idx: 16, tile: Tile { value: 2, color: Color::Red}, location: MoveLocation::Beginning}, "16a<R2"),
        (Move::Add {line_idx: 17, tile: Tile { value: 3, color: Color::Blue}, location: MoveLocation::End}, "17a>B3"),
        (Move::Add {line_idx: 18, tile: Tile { value: 4, color: Color::Yellow}, location: MoveLocation::Beginning}, "18a<Y4"),
        (Move::Add {line_idx: 19, tile: Tile { value: 5, color: Color::Black}, location: MoveLocation::End}, "19a>X5")
    ];

    for case in cases {
        assert_eq!(case.0.to_string(), case.1)
    }
}

#[test]
fn remove_move_serialization() {
    let cases = vec![
        (Move::Remove {line_idx: 1, location: MoveLocation::Beginning}, "1r<"),
        (Move::Remove {line_idx: 7, location: MoveLocation::End}, "7r>"),
        (Move::Remove {line_idx: 0, location: MoveLocation::Beginning}, "0r<"),
        (Move::Remove {line_idx: 1, location: MoveLocation::End}, "1r>"),
        (Move::Remove {line_idx: 8, location: MoveLocation::Beginning}, "8r<"),
        (Move::Remove {line_idx: 4, location: MoveLocation::End}, "4r>"),
        (Move::Remove {line_idx: 7, location: MoveLocation::Beginning}, "7r<"),
        (Move::Remove {line_idx: 2, location: MoveLocation::End}, "2r>"),
        (Move::Remove {line_idx: 10, location: MoveLocation::Beginning}, "10r<"),
        (Move::Remove {line_idx: 100, location: MoveLocation::End}, "100r>"),
    ];

    for case in cases {
        assert_eq!(case.0.to_string(), case.1)
    }
}

#[test]
fn line_serialization() {
    let cases = vec![
        (
            Line {r#type: LineType::NumberSequence, tiles: vec![
                Tile {value: 1, color: Color::Red},
                Tile {value: 2, color: Color::Red},
                Tile {value: 3, color: Color::Red}
            ]},
            "M:R1;R2;R3;"
        ),
        (
            Line {r#type: LineType::SingleNumber, tiles: vec![
                Tile {value: 1, color: Color::Red},
                Tile {value: 1, color: Color::Blue},
                Tile {value: 1, color: Color::Yellow}
            ]},
            "S:R1;B1;Y1;"
        ),
        (
            Line {r#type: LineType::NumberSequence, tiles: vec![
                Tile {value: 1, color: Color::Red},
                Tile {value: 0, color: Color::Red},
                Tile {value: 3, color: Color::Red}
            ]},
            "M:R1;R*;R3;"
        )
    ];

    for case in cases {
        assert_eq!(case.0.to_string(), case.1)
    }
}