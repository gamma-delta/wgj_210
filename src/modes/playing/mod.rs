mod draw;

use std::mem;

use ahash::AHashMap;
use cogs_gamedev::{controls::InputHandler, grids::ICoord};
use itertools::Itertools;
use macroquad::prelude::{vec2, Vec2};
use smallvec::SmallVec;

use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, Gamemode, GamemodeDrawer, Transition},
    controls::{Control, InputSubscriber},
    simulator::{
        board::Board,
        symbols::{Symbol, SYMBOL_GAP},
    },
    utils::draw::mouse_position_pixel,
    HEIGHT, WIDTH,
};

use self::draw::Drawer;

/// Place to start drawing the board from
const BOARD_ORIGIN_X: f32 = 16.0;
const BOARD_ORIGIN_Y: f32 = 16.0;

const SYMBOLS_ACROSS: usize = 13;
const SYMBOLS_DOWN: usize = 13;

pub struct ModePlaying {
    level_id: String,
    board: Board,

    /// Mapping of symbol codes to atlas indices
    symbol_indices: AHashMap<u32, usize>,

    selection: SelectState,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
enum SelectState {
    None,
    /// We've picked up a fragment.
    /// Remove the symbols from the board and put them in here.
    HoldingFragment {
        /// The square the mouse clicked on to draw this.
        origin: ICoord,
        /// The ORIGINAL positions of the pieces on the board are in here!
        /// This is used so we can put them back in case of something invalid happening.
        ///
        /// `origin - <this>` is the relative position we draw them wrt the mouse
        symbols: SmallVec<[(ICoord, Symbol); 8]>,
    },
}

impl ModePlaying {
    pub fn new_from_level(idx: usize, assets: &Assets) -> Self {
        let level = &assets.levels[idx];
        let board = level.original_board.clone();
        let symbol_indices =
            Symbol::stitch_atlas(board.symbols.values().map(|sym| sym.code), assets);

        Self {
            level_id: level.id.clone(),
            board,
            symbol_indices,
            selection: SelectState::None,
        }
    }
}

impl Gamemode for ModePlaying {
    fn update(
        &mut self,
        controls: &InputSubscriber,
        frame_info: FrameInfo,
        assets: &Assets,
    ) -> Transition {
        let (mx, my) = mouse_position_pixel();

        let hovered_coord = px_to_coord(vec2(mx, my));

        if controls.clicked_down(Control::Click) {
            dbg!(hovered_coord);
        }

        if controls.clicked_down(Control::Debug) {
            dbg!(self.board.symbols.get(&hovered_coord), &self.selection);
        }

        match &self.selection {
            SelectState::None => {
                if controls.clicked_down(Control::Click) {
                    let fragment_idx = self
                        .board
                        .fragments
                        .iter()
                        .enumerate()
                        .find_map(|(idx, frag)| frag.contains(&hovered_coord).then_some(idx));
                    if let Some(frag_idx) = fragment_idx {
                        let frag_poses = self.board.fragments.remove(frag_idx);
                        let extracted = frag_poses
                            .into_iter()
                            // Panic only when fragments doesn't refer to a symbol pos,
                            // so thats ok
                            .map(|pos| (pos, self.board.symbols.remove(&pos).unwrap()))
                            .collect();

                        self.selection = SelectState::HoldingFragment {
                            origin: hovered_coord,
                            symbols: extracted,
                        }
                    }
                }
            }
            SelectState::HoldingFragment { origin, symbols } => {
                if !controls.pressed(Control::Click) {
                    // Check if we can place it back
                    let collision = symbols.iter().any(|(pos, _sym)| {
                        let newpos = *origin + *pos;
                        self.board.symbols.contains_key(&newpos)
                    });
                    if !collision {
                        // lovely!
                        let (origin, symbols) =
                            match mem::replace(&mut self.selection, SelectState::None) {
                                SelectState::HoldingFragment { symbols, origin } => {
                                    (origin, symbols)
                                }
                                _ => unreachable!(),
                            };
                        for (pos, sym) in symbols {
                            let newpos = origin + pos;
                            let clobber = self.board.symbols.insert(newpos, sym);
                            assert_eq!(clobber, None);
                        }
                    }
                }
            }
        }

        Transition::None
    }

    fn get_draw_info(&mut self) -> Box<dyn GamemodeDrawer> {
        Box::new(Drawer {
            board: self.board.clone(),
            symbol_indices: self.symbol_indices.clone(),
            selection: self.selection.clone(),
        })
    }
}

/// Given a coordinate in pixel space, and the center of the screen in world space,
/// get the world space coordinates the pixel lies within.
fn px_to_coord(pos: Vec2) -> ICoord {
    let adjust = (pos - vec2(BOARD_ORIGIN_X, BOARD_ORIGIN_Y)) / SYMBOL_GAP - Vec2::splat(0.5);
    ICoord::new(adjust.x.round() as isize, adjust.y.round() as isize)
}

/// Given a coordinate in world space and the center of the screen in world space,
/// get the pixel coordinates of its center.
fn coord_to_px(pos: ICoord) -> Vec2 {
    (vec2(pos.x as f32, pos.y as f32) * SYMBOL_GAP).round() + vec2(BOARD_ORIGIN_X, BOARD_ORIGIN_Y)
}
