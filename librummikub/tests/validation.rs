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
use librummikub::{Color, Move, MoveLocation, Tile};


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
fn test_validate_tile_valid() {
    assert!(Tile { value: 1, color: Color::Red}.validate().is_ok());
}

#[test]
fn test_validate_tile_invalid() {
    assert_eq!(
        Tile { value: 14, color: Color::Blue}.validate().err().unwrap(),
        "Tile value 14 exceeds maximum 13"
    );
}

