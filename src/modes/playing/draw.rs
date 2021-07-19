use ahash::AHashMap;
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, GamemodeDrawer},
    simulator::board::Board,
    utils::draw::mouse_position_pixel,
    HEIGHT, WIDTH,
};

use super::{px_to_coord, SelectState, SYMBOL_GAP};

const TILES_ACROSS: usize = (WIDTH / SYMBOL_GAP * 2.0) as usize;
const TILES_DOWN: usize = (HEIGHT / SYMBOL_GAP * 2.0) as usize;

pub(super) struct Drawer {
    pub board: Board,
    pub symbol_indices: AHashMap<u32, usize>,

    pub view: Vec2,
    pub drag_origin: Option<Vec2>,

    pub selection: SelectState,
}

impl GamemodeDrawer for Drawer {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo) {
        let (mx, my) = mouse_position_pixel();
        let view = if let Some(origin) = self.drag_origin {
            self.view - (vec2(mx, my) - origin) / SYMBOL_GAP
        } else {
            self.view
        };

        for tile_dx in -1..=TILES_ACROSS as i32 {
            for tile_dy in -1..=TILES_DOWN as i32 {
                // symbol positions
                // each tile is 2x symbol size
                let corner = (vec2(tile_dx as f32, tile_dy as f32)
                    - vec2(view.x.fract(), view.y.fract()))
                    * SYMBOL_GAP
                    * 2.0;
                let corner = corner.round();

                // Check the center of the tile for position to avoid imprecision errors
                let absolute_pos = px_to_coord(corner + Vec2::splat(SYMBOL_GAP), view);
                let sx = if absolute_pos.x == 0 {
                    0.0
                } else {
                    SYMBOL_GAP * 2.0
                };
                let sy = if absolute_pos.y == 0 {
                    0.0
                } else {
                    SYMBOL_GAP * 2.0
                };

                draw_texture_ex(
                    assets.textures.checkerboard,
                    corner.x,
                    corner.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(sx, sy, SYMBOL_GAP * 2.0, SYMBOL_GAP * 2.0)),
                        ..Default::default()
                    },
                );
                if absolute_pos.x % 3 == 0 || absolute_pos.y % 3 == 0 {
                    draw_text(&absolute_pos.to_string(), corner.x, corner.y, 8.0, WHITE);
                }
            }
        }
    }
}
