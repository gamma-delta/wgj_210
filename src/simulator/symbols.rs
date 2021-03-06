//! Knowledge about the symbols in a level, and the ability to see if they're grammatical.
//!
//! Symbols are indexed by a `char`, ie each symbol maps to a `char` and that `char`
//! is used to refer to it.

use std::str::FromStr;

use ahash::{AHashMap, AHashSet};
use anyhow::bail;
use cogs_gamedev::grids::{Direction4, ICoord};
use itertools::Itertools;
use macroquad::prelude::{Color, Image, Rect, Texture2D, Vec2};

use crate::assets::Assets;

pub const SYMBOL_SIZE: usize = 5;
pub const SYMBOL_DISPLAY_SIZE: f32 = 15.0;
pub const SYMBOL_GAP: f32 = 17.0;

/// Info about a symbol.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol {
    pub part_of_speech: PartOfSpeech,
    /// The numerical code representing a bitmap.
    /// The least significant bit is the lower-right corner, and it increases
    /// right-to-left bottom-to-top.
    ///
    /// 1 = filled, 0 = empty
    pub code: u32,
}

impl Symbol {
    /// The size of a side of the atlas.
    pub const ATLAS_SIDE: u16 = 256;
    /// Number of symbols that fit in the atlas per row. (And per column.)
    pub const SYMBOLS_PER_ROW: u16 = Symbol::ATLAS_SIDE / 5;

    /// Stitch a bunch of symbol codes into the atlas texture.
    ///
    /// The 5x5 symbols are packed into a 256x256 texture.
    /// (This does mean there can only be `(256 / 5) ^ 2 = 2601` unique symbols,
    /// but I question what you're doing with all those.)
    ///
    /// Also returns a mapping of symbol codes to their index into the texture.
    pub fn stitch_atlas(codes: impl Iterator<Item = u32>, assets: &Assets) -> AHashMap<u32, usize> {
        let mut img =
            Image::gen_image_color(Self::ATLAS_SIDE, Self::ATLAS_SIDE, macroquad::color::BLANK);

        let out = codes
            .unique()
            .enumerate()
            .map(|(idx, code)| {
                let rect = Self::slice(idx);
                for dx in 0..SYMBOL_SIZE {
                    for dy in 0..SYMBOL_SIZE {
                        let px = rect.x as usize + dx;
                        let py = rect.y as usize + dy;

                        let bitpos = SYMBOL_SIZE * dy + dx;
                        let bit = code & (1 << (24 - bitpos));
                        if bit != 0 {
                            // Draw it in white so we can change it to any color
                            img.set_pixel(px as u32, py as u32, macroquad::color::WHITE);
                        }
                    }
                }

                (code, idx)
            })
            .collect();

        assets.symbol_atlas.update(&img);

        out
    }

    /// Get the rectangle needed to slice out this symbol's texture from the atlas.
    pub fn slice(idx: usize) -> Rect {
        let x = idx % Self::SYMBOLS_PER_ROW as usize;
        let y = idx / Self::SYMBOLS_PER_ROW as usize;

        Rect {
            x: x as f32 * SYMBOL_SIZE as f32,
            y: y as f32 * SYMBOL_SIZE as f32,
            w: SYMBOL_SIZE as f32,
            h: SYMBOL_SIZE as f32,
        }
    }

    pub fn draw(&self, corner: Vec2, idx: usize, color: Color, assets: &Assets) {
        use macroquad::prelude::*;
        draw_texture_ex(
            assets.symbol_atlas,
            corner.x,
            corner.y,
            color,
            DrawTextureParams {
                source: Some(Symbol::slice(idx)),
                dest_size: Some(Vec2::splat(SYMBOL_DISPLAY_SIZE)),
                ..Default::default()
            },
        );
    }
}

/// Parse a 5x5 block of characters, separated by newlines.
/// Periods, underscores, and whitespace becomes a blank square;
/// everything else becomesafilled square.
impl FromStr for Symbol {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // we expect the string to be 5 lines of 5 characters
        // separated by newlines.
        let mut map = AHashSet::new();
        let mut code = 0u32;

        for (y, line) in s.lines().enumerate() {
            if y > 5 {
                bail!("too many lines");
            }

            for (x, c) in line.chars().enumerate() {
                if x > 5 {
                    bail!("line {} has too many characters", y);
                }

                if !c.is_whitespace() && c != '.' && c != '_' {
                    map.insert(ICoord::new(x as isize, y as isize));

                    let bitpos = 5 * y + x;
                    code |= 1 << bitpos;
                }
            }
        }

        Ok(Symbol {
            part_of_speech: PartOfSpeech::new_from_code(code),
            code,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PartOfSpeech {
    /// What we start every sentence with
    ParticleStart,
    /// Goes after a list of one or more Nouns
    ParticleCollate,
    /// A noun or noun modifier.
    Noun { islands: u8, depth: u8 },
    /// A verb or verb modifier.
    Verb { islands: u8, depth: u8 },
}

impl PartOfSpeech {
    /// Get the part of speech from the numerical code.
    pub fn new_from_code(code: u32) -> Self {
        // Special cases: check the code directly
        // sadly this *is* the simplest way to check i thought of
        #[allow(clippy::unusual_byte_groupings)]
        let special_out = if code == 0b11111_10001_10001_10001_11111 {
            Some(PartOfSpeech::ParticleStart)
        } else if code == 0b11111_10001_10101_10001_11111 {
            Some(PartOfSpeech::ParticleCollate)
        } else {
            None
        };
        if let Some(special_out) = special_out {
            return special_out;
        }

        let mut map = AHashSet::new();
        for x in 0..5 {
            for y in 0..5 {
                let shift = 5 * y + x;
                let bit = code & (1 << shift);
                if bit != 0 {
                    map.insert(ICoord::new(x, y));
                }
            }
        }

        // Check what part of speech this may be
        let mut maybe_noun = true; // bilateral symmetry?

        let mut islands_found = 0u8;
        let mut cells_flooded_to = AHashSet::new();
        // Reuse allocations
        let mut to_check = Vec::new();

        let mut single_pixels_found = 0u8;

        for x in 0..5 {
            for y in 0..5 {
                let pos = ICoord::new(x, y);

                let present = map.contains(&pos);

                // We only need to do symmetry check for half the box
                if x <= 2 {
                    let negx = 4 - x;
                    let negy = 4 - y;
                    if present != map.contains(&ICoord::new(negx, negy)) {
                        // The opposite pixel is different!
                        maybe_noun = false;
                    }
                }

                if present && !cells_flooded_to.contains(&pos) {
                    islands_found += 1;

                    to_check.push(pos);
                    while let Some(pos) = to_check.pop() {
                        if cells_flooded_to.insert(pos) {
                            let mut has_any_neighbor = false;
                            for dir in Direction4::DIRECTIONS {
                                let neighbor = pos + dir;
                                if map.contains(&neighbor) {
                                    to_check.push(neighbor);
                                    has_any_neighbor = true;
                                }
                            }
                            if !has_any_neighbor {
                                single_pixels_found += 1;
                            }
                        }
                    }

                    // just in case
                    to_check.clear();
                }
            }
        }

        if maybe_noun {
            PartOfSpeech::Noun {
                islands: islands_found,
                depth: single_pixels_found,
            }
        } else {
            PartOfSpeech::Verb {
                islands: islands_found,
                depth: single_pixels_found,
            }
        }
    }

    /// Returns `true` if the part_of_speech is [`ParticleStart`].
    pub fn is_particle_start(&self) -> bool {
        matches!(self, Self::ParticleStart)
    }

    /// Returns `true` if the part_of_speech is [`ParticleCollate`].
    pub fn is_particle_collate(&self) -> bool {
        matches!(self, Self::ParticleCollate)
    }

    /// Returns `true` if the part_of_speech is [`Noun`].
    pub fn is_noun(&self) -> bool {
        matches!(self, Self::Noun { .. })
    }

    /// Returns `true` if the part_of_speech is [`Verb`].
    pub fn is_verb(&self) -> bool {
        matches!(self, Self::Verb { .. })
    }
}
