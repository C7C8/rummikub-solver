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
use clap::{Parser, Subcommand};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

#[derive(Parser, Debug)]
#[command(name="Rummikub Solver", version, about, long_about)]
struct Cli{
    /// Enable verbose output.
    #[arg(short, long)]
    verbose: bool,

    /// Your current hand.
    #[arg(long, required = true)]
    hand: String, // TODO Choose proper data type

    /// The current board.
    #[arg(long, required = true)]
    board: String, // TODO Choose proper data type

    /// Search timeout in seconds. If the timeout elapses, the best sequence out of the available
    /// moves will be chosen.
    #[arg(short, long, default_value = "60")]
    timeout: u16,

    #[command(subcommand)]
    optimizer: Optimizers
}

#[derive(Debug, Subcommand)]
#[derive(Clone)]
pub enum Optimizers {
    /// Naively optimize for discarding the most possible tiles from your hand.
    NaiveMaxDiscard {
        /// The maximum amount of moves that are permissible as part of the sequence
        #[arg(long, default_value = "32")]
        max_moves: u8
    },

    /// Same as NaiveMaxDiscard, but minimizes the number of moves per sequence.
    MinimizeMoves {
        // The minimum amount of moves tht are permissible as part of the sequence
        #[arg(long, default_value = "32")]
        min_moves: u8
    },

    /// Blindly maximizes the amount of moves in the sequence. Good for messing up your opponents'
    /// planned sequences.
    MaximizeMoves {
        /// The maximum amount of moves that are permissible as part of the sequence
        #[arg(long, default_value = "32")]
        max_moves: u8
    }
}

fn main() {
    // Setup - argument parsing &
    let args = Cli::parse();

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(if args.verbose { LevelFilter::Trace } else { LevelFilter::Warn }))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
