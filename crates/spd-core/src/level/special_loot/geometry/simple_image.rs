//! `CustomTilemap.mapSimpleImage`.

pub(crate) fn map_simple_image(tile_w: i32, tile_h: i32, tx: i32, ty: i32, tex_w: i32) -> Vec<i16> {
    let tex_tile_width = tex_w / 16;
    let mut data = Vec::with_capacity((tile_w * tile_h) as usize);
    let mut x = tx;
    let mut y = ty;
    for _ in 0..tile_w * tile_h {
        data.push((x + tex_tile_width * y) as i16);
        x += 1;
        if x - tx == tile_w {
            x = tx;
            y += 1;
        }
    }
    data
}
