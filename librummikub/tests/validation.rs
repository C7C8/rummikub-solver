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

use librummikub::{Board, Color, Line, LineType, Move, MoveLocation, Tile};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;


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

/// Simple triples should pass
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

/// Simple quads should pass
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

/// Triples with wildcards should pass
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

/// Triples with **multiple** wildcards should pass
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

///Triples of all wildcards should pass too
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

/// Fail lines that are less than 3 tiles
#[test]
fn line_invalidate_too_small() {
    assert_eq!(
        Line {
            r#type: LineType::SingleNumber,
            tiles: vec![
                Tile {value: 13, color: Color::Blue},
            ]
        }.validate().err().unwrap(),
        "Line length 1 too short, must be at least 3");
}

/// Fail lines that would otherwise be valid were it not for invalid tiles
#[test]
fn line_invalidate_bad_tile() {
    assert_eq!(
        Line {
            r#type: LineType::SingleNumber,
            tiles: vec![
                Tile {value: 14, color: Color::Blue},
                Tile {value: 14, color: Color::Red},
                Tile {value: 14, color: Color::Yellow},
            ]
        }.validate().err().unwrap(),
        "Invalid tile at index 0: Tile value 14 exceeds maximum 13");
}

/// Single number lines cannot repeat colors
#[test]
fn line_invalidate_single_number_color_repeat() {
    assert_eq!(
        Line {
            r#type: LineType::SingleNumber,
            tiles: vec![
                Tile {value: 1, color: Color::Blue},
                Tile {value: 1, color: Color::Red},
                Tile {value: 1, color: Color::Red},
            ]
        }.validate().err().unwrap(),
        "Tile at index 2 has duplicate color R"
    );
}

/// Single number lines cannot repeat colors, even if one the repeated color tile is a wildcard
#[test]
fn line_invalidate_single_number_color_repeat_wildcard() {
    // TODO Allow for game rule change where wildcards make colors wild too
    assert_eq!(
        Line {
            r#type: LineType::SingleNumber,
            tiles: vec![
                Tile {value: 1, color: Color::Blue},
                Tile {value: 1, color: Color::Red},
                Tile {value: 0, color: Color::Red},
            ]
        }.validate().err().unwrap(),
        "Tile at index 2 has duplicate color R"
    );
}

/// Single number lines must only contain one number (or wildcard)
#[test]
fn line_invalidate_single_number_bad_number() {
    assert_eq!(
        Line {
        r#type: LineType::SingleNumber,
        tiles: vec![
            Tile {value: 1, color: Color::Blue},
            Tile {value: 1, color: Color::Red},
            Tile {value: 2, color: Color::Yellow},
        ]
    }.validate().err().unwrap(),
        "Tile at index 2 has value 2 but 1 was expected"
    );
}

/****************************
 * LINE SEQUENCE HAPPY PATH *
 ****************************/

/// Simple triplet sequences should pass
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

/// Sequences can start at any number
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

/// Sequences can include wildcards
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

/// Sequences can have very long lengths
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

/// Sequences can have multiple wildcards
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

/// Sequences must all be one color
#[test]
fn line_invalidate_sequence_bad_color() {
    assert_eq!(
        Line {
            r#type: LineType::NumberSequence,
            tiles: vec![
                Tile {value: 8, color: Color::Red},
                Tile {value: 9, color: Color::Red},
                Tile {value: 10, color: Color::Blue},
            ]
        }.validate().err().unwrap(),
        "Tile at index 2 has color B but R was expected"
    );
}

/// Sequences cannot skip numbers
#[test]
fn line_invalidate_sequence_number_skip() {
    assert_eq!(
        Line {
            r#type: LineType::NumberSequence,
            tiles: vec![
                Tile {value: 8, color: Color::Blue},
                Tile {value: 9, color: Color::Blue},
                Tile {value: 10, color: Color::Blue},
                // 11 skipped
                Tile {value: 12, color: Color::Blue},
                Tile {value: 13, color: Color::Blue},
            ]
        }.validate().err().unwrap(),
        "Tile at index 3 was expected to have number 11 but had 12"
    );
}

/// Sequences can have wildcards so long as the color of the wildcard matches
#[test]
fn line_invalidate_sequence_bad_wildcard_color() {
    // TODO add option for wildcards to be color-wild as well
    assert_eq!(
        Line {
            r#type: LineType::NumberSequence,
            tiles: vec![
                Tile {value: 8, color: Color::Blue},
                Tile {value: 9, color: Color::Blue},
                Tile {value: 0, color: Color::Red},
                Tile {value: 11, color: Color::Blue},
            ]
        }.validate().err().unwrap(),
        "Tile at index 2 has color R but B was expected"
    );
}

/// Sequences of all wildcards should fail. Note that in a real game this will probably
/// never happen.
#[test]
fn line_invalidate_sequence_all_wildcards() {
    // TODO Verify that this should be illegal
    assert_eq!(
        Line {
            r#type: LineType::NumberSequence,
            tiles: vec![
                Tile {value: 0, color: Color::Blue},
                Tile {value: 0, color: Color::Blue},
                Tile {value: 0, color: Color::Blue},
            ]
        }.validate().err().unwrap(),
        "Line is all wildcards"
    );
}

/**********
 * BOARDS *
 **********/

/// Validate valid boards
#[test]
fn board_validate_happy_path() {
    let board = Board {
        lines: vec![
            Line {
                r#type: LineType::SingleNumber,
                tiles: vec![
                    Tile {value: 1, color: Color::Red},
                    Tile {value: 1, color: Color::Blue},
                    Tile {value: 1, color: Color::Yellow},
                ]
            },
            Line {
                r#type: LineType::NumberSequence,
                tiles: vec![
                    Tile {value: 1, color: Color::Red},
                    Tile {value: 2, color: Color::Red},
                    Tile {value: 3, color: Color::Red},
                ]
            }
        ],
        history: vec![],
    };

    assert!(board.validate().is_ok());
}

/// Invalidate error-containing boards
#[test]
fn board_validate_error() {
    let board = Board {
        lines: vec![
            Line {
                r#type: LineType::SingleNumber,
                tiles: vec![
                    Tile {value: 1, color: Color::Red},
                    Tile {value: 2, color: Color::Blue},
                    Tile {value: 1, color: Color::Yellow},
                ]
            },
            Line {
                r#type: LineType::NumberSequence,
                tiles: vec![
                    Tile {value: 1, color: Color::Red},
                    Tile {value: 2, color: Color::Red},
                    Tile {value: 3, color: Color::Red},
                ]
            }
        ],
        history: vec![],
    };

    assert_eq!(
        board.validate().err().unwrap(),
        "Invalid line at index 0: Tile at index 1 has value 2 but 1 was expected"
    );
}
