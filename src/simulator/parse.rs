use std::convert::{TryFrom, TryInto};

use ahash::AHashMap;
use anyhow::{anyhow, bail};
use cogs_gamedev::grids::{Direction4, ICoord, Rotation};
use enum_map::{enum_map, Enum, EnumMap};
use itertools::{FoldWhile, Itertools};
use once_cell::sync::Lazy;
use smallvec::{smallvec, SmallVec};

use super::symbols::{PartOfSpeech, Symbol};

/// Check if a sequence of blocks is grammatically valid, starting from the start symbol
///
/// If it is, returns `Ok` with a list of all the coordinates in the sentence.
/// If not, returns `Err`.
pub fn check_from_start(
    symbols: &AHashMap<ICoord, Symbol>,
    origin: ICoord,
) -> anyhow::Result<Vec<ICoord>> {
    // I type this code so much i should just put a `neighbors4` method on ICoord...
    let dir = *Direction4::DIRECTIONS
        .iter()
        .filter(|dir| {
            let neighbor = origin + **dir;
            symbols.get(&neighbor).is_some()
        })
        .exactly_one()
        .map_err(|oh_no| {
            anyhow!(
                "wanted exactly 1 occupied neighbor but got {:?}",
                oh_no.collect_vec()
            )
        })?;

    // We can pretty easily parse this with a state machine.
    // Each state has a mapping of symbol types to what the next state is.
    let mut state = SpineState::Start;
    let mut idx = 0;
    let spine_len = loop {
        let pos = origin + dir.deltas() * idx;
        let sym = symbols.get(&pos);

        let posk: PartOfSpeechKind = sym.map(|sym| &sym.part_of_speech).try_into()?;
        // Deliberate annotation cause rust-analyzer doesn't like it
        let next_states: &SmallVec<[(PartOfSpeechKind, SpineState); 2]> = &SPINE_STATES[state];
        let next = next_states
            .iter()
            .find_map(|(target, next)| (&posk == target).then_some(*next))
            .ok_or_else(|| anyhow!("{:?} does not accept {:?}"));

        match next {
            Ok(SpineState::Satisfied) => {
                // we're done here! nice
                break idx;
            }
            Ok(next) => state = next,
            Err(oh_no) => return Err(oh_no),
        }

        idx += 1;
    };

    // add modifier positions to this as they happen.
    // we do .. and not ..= because we always end with an EOF => no symbol.
    let mut seen_poses = (0..spine_len)
        .map(|idx| origin + dir.deltas() * idx)
        .collect_vec();

    // Now, for each noun/verb on the spine, check it for modifiers.
    let adj_dir = dir.rotate(Rotation::Clockwise);
    let adv_dir = dir.rotate(Rotation::CounterClockwise);
    for spine_pos in seen_poses.clone() {
        let base_sym = symbols.get(&spine_pos).ok_or_else(|| {
            anyhow!(
                "somehow previously had symbol on the spine at {} and now don't",
                spine_pos
            )
        })?;
        let moddable = match base_sym.part_of_speech {
            PartOfSpeech::Noun { islands, depth: 0 } => Some((islands, true)),
            PartOfSpeech::Verb { islands, depth: 0 } => Some((islands, false)),
            PartOfSpeech::Noun { .. } | PartOfSpeech::Verb { .. } => bail!(
                "somehow had a modifier {:?} on the spine at {} when we should have checked for it",
                base_sym,
                spine_pos
            ),
            _ => None,
        };
        if let Some((islands, is_noun)) = moddable {
            // Check for modifiers down and modifier-modifiers up

            'each_mod: for (is_adj, look_dir) in [(true, adj_dir), (false, adv_dir)] {
                for transverse in 0.. {
                    let mod_pos = spine_pos + adj_dir.deltas() * transverse;
                    match symbols.get(&mod_pos) {
                        Some(sym) => {
                            let (mod_islands, mod_depth, mod_is_noun) = match sym.part_of_speech {
                                PartOfSpeech::Noun { islands, depth } => (islands, depth, true),
                                PartOfSpeech::Verb { islands, depth } => (islands, depth, false),
                                _ => {
                                    // Do we allow non-modifiers on a modifier line?
                                    // My gut instinct is yes, it leads to better puzzles.
                                    // But i can easily change it.

                                    // For now just break if we find it to quit safely.
                                    break 'each_mod;
                                }
                            };
                            let error_name = if is_adj { "adjective" } else { "adverb" };
                            if is_noun != mod_is_noun {
                                bail!(
                                    "the {} at {}, {:?} was {} a noun but the base was {} a noun",
                                    error_name,
                                    mod_pos,
                                    sym,
                                    mod_is_noun,
                                    is_noun
                                );
                            }
                            if mod_depth != 1 {
                                bail!(
                                    "the {} at {}, {:?}, had a bad depth",
                                    error_name,
                                    mod_pos,
                                    sym
                                );
                            }
                            if mod_islands != islands {
                                bail!("the {} at {}, {:?}, did not have the right island count (wanted {})", error_name, mod_pos, sym, islands);
                            }

                            // This seems to be valid!
                            seen_poses.push(mod_pos);
                        }
                        // No modifier? stop looking entirely.
                        // This is correct behavior in both cases! Once we miss one, we miss all further ones
                        // this even applies if i somehow add modifier-modifier-modifiers.
                        None => break 'each_mod,
                    }
                }
            }
        }
    }

    Ok(seen_poses)
}

/// Parts of speech on the spine of a sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartOfSpeechKind {
    Start,
    Collator,
    Verb,
    Noun,
    Eof,
}

impl TryFrom<Option<&PartOfSpeech>> for PartOfSpeechKind {
    type Error = anyhow::Error;
    fn try_from(value: Option<&PartOfSpeech>) -> Result<Self, Self::Error> {
        Ok(match value {
            Some(PartOfSpeech::ParticleStart) => PartOfSpeechKind::Start,
            Some(PartOfSpeech::ParticleCollate) => PartOfSpeechKind::Collator,
            Some(PartOfSpeech::Noun { depth: 0, .. }) => PartOfSpeechKind::Noun,
            Some(PartOfSpeech::Verb { depth: 0, .. }) => PartOfSpeechKind::Verb,
            None => PartOfSpeechKind::Eof,
            Some(oh_no) => bail!(
                "could not convert {:?} because it had a non-zero depth",
                oh_no
            ),
        })
    }
}

/// What is the next thing in line supposed to be?
#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq)]
enum SpineState {
    /// The very beginning, a dummy check that there's a start sigil here
    Origin,
    /// At the start sigil and want a noun
    Start,
    /// We've read our first noun, looking for a verb or other noun
    Subject1,
    /// After reading our second noun we want a collator or other noun
    SubjectN,
    /// Read a subject collator and must have a verb
    SubjectCollator,
    /// Looking for EOF or another noun
    Verb,
    /// Looking for EOF or another noun, but we're targeting a noun
    Object1,
    /// We're on object #2+ and want a collator
    ObjectN,
    /// Finished reading objects and need an EOF
    ObjectCollator,
    /// And we're satisfied! When we reach this state halt.
    Satisfied,
}

/// The possible values are so small it's likely most efficient to just use a smallvec and iterate each one
static SPINE_STATES: Lazy<EnumMap<SpineState, SmallVec<[(PartOfSpeechKind, SpineState); 2]>>> =
    Lazy::new(|| {
        enum_map! {
            SpineState::Origin => smallvec![(PartOfSpeechKind::Start, SpineState::Start)],
            SpineState::Start => smallvec![(PartOfSpeechKind::Noun, SpineState::Subject1)],
            SpineState::Subject1 => smallvec![(PartOfSpeechKind::Noun, SpineState::SubjectN), (PartOfSpeechKind::Verb, SpineState::Verb)],
            SpineState::SubjectN => smallvec![(PartOfSpeechKind::Noun, SpineState::SubjectN), (PartOfSpeechKind::Collator, SpineState::SubjectCollator)],
            SpineState::SubjectCollator => smallvec![(PartOfSpeechKind::Verb, SpineState::Verb)],
            SpineState::Verb => smallvec![(PartOfSpeechKind::Eof, SpineState::Satisfied), (PartOfSpeechKind::Noun, SpineState::Object1)],
            SpineState::Object1 => smallvec![(PartOfSpeechKind::Noun, SpineState::ObjectN), (PartOfSpeechKind::Eof, SpineState::Satisfied)],
            SpineState::ObjectN => smallvec![(PartOfSpeechKind::Noun, SpineState::ObjectN), (PartOfSpeechKind::Collator, SpineState::ObjectCollator)],
            SpineState::ObjectCollator => smallvec![(PartOfSpeechKind::Eof, SpineState::Satisfied)],
            SpineState::Satisfied => smallvec![],
        }
    });
