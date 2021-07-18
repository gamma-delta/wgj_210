use ahash::{AHashMap, AHashSet};
use anyhow::anyhow;
use cogs_gamedev::grids::ICoord;
use itertools::Itertools;
use smallvec::SmallVec;

use super::{parse, symbols::Symbol};

/// The playfield the player moves symbols around.
///
/// The board does *not* keep track of the big atlas of symbol textures.
#[derive(Debug, Clone)]
pub struct Board {
    /// Symbols on the board
    pub symbols: AHashMap<ICoord, Symbol>,
    /// Symbols that are held together in fragments.
    /// Each entry in the Vec is a list of coordinates that are stuck together.
    pub fragments: Vec<SmallVec<[ICoord; 8]>>,
}

impl Board {
    /// Test this board for all the symbols in grammatically correct sentences and all the symbols
    /// outside of them.
    ///
    /// The first vec has the coordinates of all the grammatically correct sentences (in batches).
    /// The second vec has all the encountered errors.
    /// If it's empty then everything was OK!
    ///
    /// All the positions *not* in the grammatically correct group are problematic.
    /// If there are any free-floating symbols detail about that will be pushed to the errors vec.
    pub fn check_grammar(&self) -> (Vec<ICoord>, Vec<anyhow::Error>) {
        let (okays, mut errors): (Vec<_>, Vec<_>) = self
            .symbols
            .iter()
            .filter_map(|(pos, symbol)| {
                // First search for all the start blocks
                if symbol.part_of_speech.is_particle_start() {
                    Some(*pos)
                } else {
                    None
                }
            })
            .map(|start| parse::check_from_start(&self.symbols, start))
            .partition_result();

        let mut okays_set = AHashSet::new();
        let okays = okays
            .into_iter()
            .flatten()
            // i swear i'm getting more unhinged as i write this
            // inspect? really??
            .inspect(|pos| {
                okays_set.insert(*pos);
            })
            .collect_vec();

        let remaining = self
            .symbols
            .keys()
            .filter(|pos| !okays_set.contains(*pos))
            .collect_vec();
        if !remaining.is_empty() {
            errors.push(anyhow!("there were leftover symbols at: {:?}", remaining));
        }

        (okays, errors)
    }
}
