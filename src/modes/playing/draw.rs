use ahash::AHashMap;
use cogs_gamedev::grids::ICoord;
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, GamemodeDrawer},
    simulator::{
        board::Board,
        symbols::{Symbol, SYMBOL_DISPLAY_SIZE},
    },
    utils::draw::{hexcolor, mouse_position_pixel},
    HEIGHT, WIDTH,
};

use super::{
    coord_to_px, px_to_coord, SelectState, BOARD_ORIGIN_X, BOARD_ORIGIN_Y, SYMBOLS_ACROSS,
    SYMBOLS_DOWN, SYMBOL_GAP,
};

pub(super) struct Drawer {
    pub board: Board,
    pub symbol_indices: AHashMap<u32, usize>,

    pub selection: SelectState,
}

impl GamemodeDrawer for Drawer {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo) {
        clear_background(BLACK);

        draw_rectangle(
            BOARD_ORIGIN_X - 2.0,
            BOARD_ORIGIN_Y - 2.0,
            SYMBOL_GAP * SYMBOLS_ACROSS as f32 + 3.0,
            SYMBOL_GAP * SYMBOLS_DOWN as f32 + 3.0,
            WHITE,
        );

        for symbol_x in 0..SYMBOLS_ACROSS {
            for symbol_y in 0..SYMBOLS_DOWN {
                let pos = ICoord::new(symbol_x as _, symbol_y as _);
                let corner = coord_to_px(pos);

                draw_rectangle(
                    corner.x,
                    corner.y,
                    SYMBOL_DISPLAY_SIZE,
                    SYMBOL_DISPLAY_SIZE,
                    hexcolor(0x92e8c0_ff),
                );

                if let Some(here) = self.board.symbols.get(&pos) {
                    let idx = *self
                        .symbol_indices
                        .get(&here.code)
                        .ok_or_else(|| {
                            format!("{:?} at {} didn't have an entry in the atlas", here, pos)
                        })
                        .unwrap();
                    here.draw(corner, idx, assets);
                }
            }
        }

        match &self.selection {
            SelectState::HoldingFragment { origin, symbols } => {
                for (pos, sym) in symbols {
                    let zero_pos = *pos + ICoord::new(-origin.x, -origin.y);
                }
            }
            _ => {}
        }
    }
}
