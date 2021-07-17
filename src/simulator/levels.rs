//! Serialization of levels, scores, and so on.

use ahash::AHashMap;
use cogs_gamedev::grids::ICoord;
use smallvec::SmallVec;

use super::Symbol;

pub struct Level {
    /// Original state of the board.
    original_board: Board,
}

/// The playfield the player moves symbols around.
///
/// The board only keeps track
#[derive(Debug, Clone)]
pub struct Board {
    /// Symbols on the board
    pub symbols: AHashMap<ICoord, Symbol>,
    /// Symbols that are held together in fragments.
    /// Each entry in the Vec is a list of coordinates that are stuck together.
    pub fragments: Vec<SmallVec<[ICoord; 4]>>,
}

impl Board {
    /// Test this board for all the symbols in grammatically correct sentences and all the symbols
    /// outside of them.
    ///
    /// The first vec has the coordinates of all the grammatically correct sentences (in batches).
    /// The second vec has all the grammatically incorrect ones.
    pub fn check_grammar(&self) -> (Vec<ICoord>, Vec<ICoord>) {
        todo!()
    }
}
