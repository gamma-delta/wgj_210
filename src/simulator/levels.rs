use std::str::FromStr;

use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context};
use cogs_gamedev::grids::{Coord, ICoord};
use serde::Deserialize;
use smallvec::SmallVec;

use super::{board::Board, symbols::Symbol};

/// Level as directly serialized from a file.
#[derive(Debug, Deserialize)]
pub struct RawLevel {
    name: String,
    /// Like minecraft crafting, you associate characters with symbols
    /// and then arrange the characters into the wanted shapes.
    symbols: AHashMap<char, String>,
    board: String,
}

impl RawLevel {
    /// Clone and convert this into a level.
    pub fn to_level(&self, filename: String) -> anyhow::Result<Level> {
        let char_symbols = self
            .symbols
            .iter()
            .map(|(&c, pattern)| {
                if c.is_whitespace() || c == '.' || c == '_' {
                    bail!("can't use ambiguous character `{}` as a symbol key", c);
                }
                Symbol::from_str(pattern)
                    .map(|sym| (c, sym))
                    .with_context(|| anyhow!("while trying to convert the symbol at `{}`", c))
            })
            .collect::<Result<AHashMap<_, _>, _>>()?;

        let laid_out: AHashMap<ICoord, char> = self
            .board
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Coord::new(x, y).to_icoord(), c))
            })
            .filter_map(|(pos, c)| {
                if c.is_whitespace() || c == '.' || c == '_' {
                    None
                } else if char_symbols.contains_key(&c) {
                    Some(Ok((pos, c)))
                } else {
                    Some(Err(anyhow!(
                        "the character `{}` at {}:{} did not have an associated symbol",
                        c,
                        pos.x + 1,
                        pos.y + 1
                    )))
                }
            })
            .collect::<Result<_, _>>()?;

        // Flood fill everything together
        let mut flooded_to = AHashSet::new();
        let mut fragments = Vec::new();
        // reuse memory
        let mut working_on = Vec::new();

        for pos in laid_out.keys() {
            if !flooded_to.contains(pos) {
                // i've never met this pos in my life
                let mut fragment = SmallVec::new();
                working_on.push(*pos);

                while let Some(pos) = working_on.pop() {
                    if flooded_to.insert(pos) {
                        fragment.push(pos);

                        for present_new_neighbor in pos.neighbors4().iter().filter(|&nbor| {
                            laid_out.contains_key(nbor) && !flooded_to.contains(nbor)
                        }) {
                            working_on.push(*present_new_neighbor);
                        }
                    }
                }

                // Just in case
                working_on.clear();

                fragments.push(fragment);
            }
        }

        // Render the layout into symbols
        let symbols = laid_out
            .into_iter()
            .map(|(pos, c)| {
                // ok because we previously checked
                (pos, char_symbols.get(&c).unwrap().clone())
            })
            .collect();

        let board = Board { symbols, fragments };

        Ok(Level {
            id: filename,
            name: self.name.clone(),
            original_board: board,
        })
    }
}

/// Level ready to play.
#[derive(Debug)]
pub struct Level {
    pub id: String,
    pub name: String,
    /// Original board state
    pub original_board: Board,
}
