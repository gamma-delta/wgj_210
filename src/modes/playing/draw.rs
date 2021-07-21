use ahash::{AHashMap, AHashSet};
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

    pub valid_poses: AHashSet<ICoord>,
    pub won: bool,
}

impl GamemodeDrawer for Drawer {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo) {
        clear_background(BLACK);

        draw_rectangle(
            BOARD_ORIGIN_X - 2.0,
            BOARD_ORIGIN_Y - 2.0,
            SYMBOL_GAP * SYMBOLS_ACROSS as f32 + 2.0,
            SYMBOL_GAP * SYMBOLS_DOWN as f32 + 2.0,
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

                    let color = if self.valid_poses.contains(&pos) {
                        hexcolor(0x6abe30_ff) // green
                    } else {
                        hexcolor(0xac3232_ff) // red
                    };

                    here.draw(corner, idx, color, assets);
                }
            }
        }

        if let SelectState::HoldingFragment { origin, symbols } = &self.selection {
            let (mx, my) = mouse_position_pixel();
            for (pos, sym) in symbols {
                let zero_pos = *pos + ICoord::new(-origin.x, -origin.y);
                let corner = vec2(zero_pos.x as f32, zero_pos.y as f32) * SYMBOL_GAP + vec2(mx, my)
                    - Vec2::splat(SYMBOL_DISPLAY_SIZE / 2.0);
                let corner = corner.round();

                let idx = self.symbol_indices[&sym.code];

                sym.draw(corner, idx, hexcolor(0x14182e_ff), assets);
            }
        }
    }
}
