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
fn tile_validate_valid() {
    assert!(Tile { value: 1, color: Color::Red}.validate().is_ok());
}

#[test]
fn tile_validate_invalid() {
    assert_eq!(
        Tile { value: 14, color: Color::Blue}.validate().err().unwrap(),
        "Tile value 14 exceeds maximum 13"
    );
}

/**************************
 * LINE SINGLE HAPPY PATH *
 **************************/

#[test]
fn line_validate_single_number_triple() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile { value: 1, color: Color::Red },
            Tile { value: 1, color: Color::Blue },
            Tile { value: 1, color: Color::Yellow }
        ]
    }.validate().is_ok())
}

#[test]
fn line_validate_single_number_quad() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 1, color: Color::Red},
            Tile {value: 1, color: Color::Yellow},
            Tile {value: 1, color: Color::Black}
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_single_number_triple_wildcard() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 0, color: Color::Yellow},
            Tile {value: 1, color: Color::Red}
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_single_number_triple_wildcard_2x() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 0, color: Color::Blue},
            Tile {value: 0, color: Color::Yellow},
            Tile {value: 1, color: Color::Red}
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_single_number_triple_wildcard_all() {
    // TODO should this actually be legal?
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 0, color: Color::Blue},
            Tile {value: 0, color: Color::Yellow},
            Tile {value: 0, color: Color::Red},
            Tile {value: 0, color: Color::Black},
        ]
    }.validate().is_ok());
}

/******************************
 * LINE SINGLE EXCEPTION PATH *
 ******************************/

#[test]
fn line_invalidate_bad_tile() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 14, color: Color::Blue},
            Tile {value: 14, color: Color::Red},
            Tile {value: 14, color: Color::Red},
        ]
    }.validate().is_err());
}

#[test]
fn line_invalidate_single_number_color_repeat() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 1, color: Color::Red},
            Tile {value: 1, color: Color::Red},
        ]
    }.validate().is_err());
}

#[test]
fn line_invalidate_single_number_color_repeat_wildcard() {
    // TODO Allow for game rule change where wildcards make colors wild too
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 1, color: Color::Red},
            Tile {value: 0, color: Color::Red},
        ]
    }.validate().is_err());
}

#[test]
fn line_invalidate_single_number_bad_number() {
    assert!(Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 1, color: Color::Red},
            Tile {value: 2, color: Color::Yellow},
        ]
    }.validate().is_err());
}

/****************************
 * LINE SEQUENCE HAPPY PATH *
 ****************************/

#[test]
fn line_validate_sequence() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 1, color: Color::Red},
            Tile {value: 2, color: Color::Red},
            Tile {value: 3, color: Color::Red},
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_sequence_middle_start() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 8, color: Color::Red},
            Tile {value: 9, color: Color::Red},
            Tile {value: 10, color: Color::Red},
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_sequence_wildcard() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 8, color: Color::Red},
            Tile {value: 0, color: Color::Red},
            Tile {value: 10, color: Color::Red},
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_sequence_full_length() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 1, color: Color::Red},
            Tile {value: 2, color: Color::Red},
            Tile {value: 3, color: Color::Red},
            Tile {value: 4, color: Color::Red},
            Tile {value: 5, color: Color::Red},
            Tile {value: 6, color: Color::Red},
            Tile {value: 7, color: Color::Red},
            Tile {value: 8, color: Color::Red},
            Tile {value: 9, color: Color::Red},
            Tile {value: 10, color: Color::Red},
            Tile {value: 11, color: Color::Red},
            Tile {value: 12, color: Color::Red},
            Tile {value: 13, color: Color::Red},
        ]
    }.validate().is_ok());
}

#[test]
fn line_validate_sequence_multi_wildcard() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 1, color: Color::Red},
            Tile {value: 2, color: Color::Red},
            Tile {value: 3, color: Color::Red},
            Tile {value: 0, color: Color::Red},
            Tile {value: 5, color: Color::Red},
            Tile {value: 6, color: Color::Red},
            Tile {value: 7, color: Color::Red},
            Tile {value: 8, color: Color::Red},
            Tile {value: 0, color: Color::Red},
            Tile {value: 10, color: Color::Red},
            Tile {value: 11, color: Color::Red},
            Tile {value: 0, color: Color::Red},
            Tile {value: 13, color: Color::Red},
        ]
    }.validate().is_ok());
}

/********************************
 * LINE SEQUENCE EXCEPTION PATH *
 ********************************/

#[test]
fn line_invalidate_sequence_bad_color() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 8, color: Color::Red},
            Tile {value: 9, color: Color::Red},
            Tile {value: 10, color: Color::Blue},
        ]
    }.validate().is_err());
}

#[test]
fn line_invalidate_sequence_number_skip() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 8, color: Color::Blue},
            Tile {value: 9, color: Color::Blue},
            Tile {value: 10, color: Color::Blue},
            // 11 skipped
            Tile {value: 12, color: Color::Blue},
            Tile {value: 13, color: Color::Blue},
        ]
    }.validate().is_err());
}

#[test]
fn line_invalidate_sequence_bad_wildcard_color() {
    assert!(Line {
        r#type: LineType::NumberSequence,
        tiles: vec![
            Tile {value: 8, color: Color::Blue},
            Tile {value: 9, color: Color::Blue},
            Tile {value: 0, color: Color::Red},
            Tile {value: 11, color: Color::Blue},
        ]
    }.validate().is_err());
}
