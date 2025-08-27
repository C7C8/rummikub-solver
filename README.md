# rummikub-solver

Rummikub is a discarding tile based number game where you build either
sequences of numbers of the same color, or you build triplets or quads of the
same number in different colors -- with the option of rearranging any existing
played tiles so long as the final state at the end of your turn leaves the
'board' in a valid state.

Or at least, that's how I learned to play it! If I got it wrong, I'll go back
and add a proper ruleset.

## Planned features
* **Modules**
  * `librummikub` - utility functions for validating Rummikub boards, validating moves
  & move sequences, generating a list of valid moves from a given board, and so on
  and so forth.
  * `librummikub-optimization` - optimizers for generating moves / move sequences from
  a given state
  * `rummikub-solver` - CLI application generating the most optimal move based
  on the board and your hand
  * `play-rummikub` - **Stretch goal!** A TUI program for actually playing Rummikub
* **Optimizers**
  * Simple discard-most optimization
  * Min-max optimization
