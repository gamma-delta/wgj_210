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
    simulator::{board::Board, symbols::Symbol},
    utils::draw::mouse_position_pixel,
    HEIGHT, WIDTH,
};

use self::draw::Drawer;

const SYMBOL_DISPLAY_SIZE: f32 = 10.0;
const SYMBOL_GAP: f32 = 11.0;

pub struct ModePlaying {
    level_id: String,
    board: Board,

    /// Mapping of symbol codes to atlas indices
    symbol_indices: AHashMap<u32, usize>,

    /// Coordinate the camera is centered on
    view: Vec2,
    /// If this is Some, we're right-clicking to drag the camera view,
    /// and this contains the original mouse position in pixel coordinates.
    ///
    /// `self.view` isn't actually updated until the camera is released; the panning is implemented in
    /// `draw` to make it less laggy.
    drag_origin: Option<Vec2>,

    selection: SelectState,
}

#[derive(Debug, Clone)]
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
    pub fn new_temp() -> Self {
        Self {
            level_id: "temp".to_string(),
            board: Board {
                symbols: AHashMap::new(),
                fragments: Vec::new(),
            },
            symbol_indices: AHashMap::new(),
            view: Vec2::ZERO,
            drag_origin: None,
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

        // Where are we truly centered on?
        let view = if let Some(origin) = self.drag_origin {
            self.view - (vec2(mx, my) - origin) / SYMBOL_GAP
        } else {
            self.view
        };
        let hovered_coord = px_to_coord(vec2(mx, my), view);

        if controls.clicked_down(Control::RightClick) {
            dbg!(hovered_coord);
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

        match self.drag_origin {
            None => {
                if controls.clicked_down(Control::RightClick) {
                    self.drag_origin = Some(vec2(mx, my));
                }
            }
            Some(origin) => {
                if !controls.pressed(Control::RightClick) {
                    // snap the view to that position
                    let dmouse = vec2(mx, my) - origin;
                    self.view -= dmouse / SYMBOL_GAP;
                    println!("view updated to {}", self.view);
                    self.drag_origin = None;
                }
            }
        }

        Transition::None
    }

    fn get_draw_info(&mut self) -> Box<dyn GamemodeDrawer> {
        Box::new(Drawer {
            board: self.board.clone(),
            symbol_indices: self.symbol_indices.clone(),
            drag_origin: self.drag_origin,
            selection: self.selection.clone(),
            view: self.view,
        })
    }
}

/// Given a coordinate in pixel space, and the center of the screen in world space,
/// get the world space coordinates the pixel lies within.
fn px_to_coord(pos: Vec2, view: Vec2) -> ICoord {
    let adjust = (pos - vec2(WIDTH, HEIGHT) / 2.0) / SYMBOL_GAP + view;
    ICoord::new(adjust.x.round() as isize, adjust.y.round() as isize)
}

/// Given a coordinate in world space and the center of the screen in world space,
/// get the pixel coordinates of its center.
fn coord_to_px(pos: ICoord, view: Vec2) -> Vec2 {
    ((vec2(pos.x as f32, pos.y as f32) - view) * SYMBOL_GAP + vec2(WIDTH, HEIGHT) / 2.0).round()
}
