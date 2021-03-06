use gl;
use super::vertex::{Attrib, VertexArray, VertexBuffer, IndexBuffer};

pub fn gen_cube(length: f32, offset: f32, a_position: Attrib) -> VertexArray {
    // 8 corners in a cube
    let corner: [(f32,f32,f32); 8] = {
        let l = offset;
        let m = length + offset;

        [
            (m, m, m),
            (m, m, l),
            (m, l, m),
            (m, l, l),
            (l, m, m),
            (l, m, l),
            (l, l, m),
            (l, l, l),
        ]
    };

    // Which corners to copy to the vertex buffer for each face.
    // In order to maintain distinct normals for each face,
    // corners cannot be shared among different faces.
    static VERT_IDX: [usize; 4*6] = [
        0,1,2,3,
        4,0,6,2,
        5,4,7,6,
        1,5,3,7,
        5,1,4,0,
        3,7,2,6
    ];

    // Which vertices to form triangle faces from
    static IDX: [u8; 6*6] = [
        0,1,2, 1,3,2,
        4,5,6, 5,7,6,
        8,9,10, 9,11,10,
        12,13,14, 13,15,14,
        16,17,18, 17,19,18,
        20,21,22, 21,23,22
    ];

    let mut buffer: Vec<f32>;
    buffer = VERT_IDX.iter().flat_map(|&i| {
        let (x,y,z) = corner[i];
        vec![x, y, z].into_iter()
    }).collect();

    VertexArray::new(|vao_ctx| {
        let ibo = IndexBuffer::rc_from_slice::<u8>(&IDX, gl::UNSIGNED_BYTE);
        let vbo = VertexBuffer::rc_from_slice::<f32>(&buffer);

        vao_ctx.bind_ibo(ibo);
        vao_ctx.bind_vbo(vbo, |vbo_ctx| {
            vbo_ctx.attr_pointer(a_position, 3, gl::FLOAT, 0, 0);
        });
    })
}

pub fn gen_tileset(tiles_width: u32, tiles_height: u32, a_position: Attrib, a_texture_uv: Attrib) -> VertexArray {
    let num_tiles = tiles_width * tiles_height;
    let buffer: Vec<f32> = (0..num_tiles).flat_map(|_| {
        static POS: [f32; 12] = [0.0,0.0, 1.0,0.0, 0.0,1.0, 1.0,0.0, 1.0,1.0, 0.0,1.0];
        POS.iter().map(|o| *o)
    }).chain((0..num_tiles).flat_map(|i| {
        let (x, y) = (i % tiles_width, i / tiles_width);
        let (fx1, fy1) = (x as f32 / tiles_width as f32, y as f32 / tiles_height as f32);
        let (fx2, fy2) = ((x+1) as f32 / tiles_width as f32, (y+1) as f32 / tiles_height as f32);

        let tex = vec![fx1,fy1, fx2,fy1, fx1,fy2, fx2,fy1, fx2,fy2, fx1,fy2];
        tex.into_iter()
    })).collect();

    VertexArray::new(|vao_ctx| {
        let vbo = VertexBuffer::rc_from_slice::<f32>(&buffer);
        vao_ctx.bind_vbo(vbo, |vbo_ctx| {
            vbo_ctx.attr_pointer(a_position, 2, gl::FLOAT, 0, 0);
            vbo_ctx.attr_pointer(a_texture_uv, 2, gl::FLOAT, 0, num_tiles as usize*4*2*6);
        });
    })
}
