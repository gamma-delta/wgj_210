use ahash::AHashMap;
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, GamemodeDrawer},
    simulator::board::Board,
    utils::draw::mouse_position_pixel,
    HEIGHT, WIDTH,
};

use super::{SelectState, SYMBOL_GAP};

const TILES_ACROSS: usize = (WIDTH / SYMBOL_GAP) as usize;
const TILES_DOWN: usize = (HEIGHT / SYMBOL_GAP) as usize;

pub(super) struct Drawer {
    pub board: Board,
    pub symbol_indices: AHashMap<u32, usize>,

    pub view: Vec2,
    pub drag_origin: Option<Vec2>,

    pub selection: SelectState,
}

impl GamemodeDrawer for Drawer {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo) {
        let mouse_world = self.view
            + if let Some(origin) = self.drag_origin {
                vec2(mx, my) - origin
            } else {
                vec2(mx, my)
            } / SYMBOL_GAP;
        let view = if let Some(origin) = self.drag_origin {
            let (mx, my) = mouse_position_pixel();
            self.view - (vec2(mx, my) - origin)
        } else {
            self.view
        };

        for tile_dx in -1..=TILES_ACROSS as i32 {
            for tile_dy in -1..=TILES_DOWN as i32 {
                // symbol positions
                // each tile is 2x symbol size
                let tile_pos = view + vec2(tile_dx as f32 * 2.0, tile_dy as f32 * 2.0);
                let tile_x = tile_pos.x.round();
                let tile_y = tile_pos.y.round();

                // each tile is 12x12!
                // here are pixel offsets
                let dx = tile_dx as f32 * 2.0 * SYMBOL_GAP;
                let dy = tile_dy as f32 * 2.0 * SYMBOL_GAP;
                // when we pan a little to see the right, view increases,
                // and we expect to see the tiles shift left.
                let px = dx + view.x.abs().fract() * 2.0 * SYMBOL_GAP;
                let py = dy + view.y.abs().fract() * 2.0 * SYMBOL_GAP;

                let sx = if tile_x == 0.0 { 0.0 } else { 2.0 * SYMBOL_GAP };
                let sy = if tile_y == 0.0 { 0.0 } else { 2.0 * SYMBOL_GAP };

                draw_texture_ex(
                    assets.textures.checkerboard,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(sx, sy, SYMBOL_GAP * 2.0, SYMBOL_GAP * 2.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
