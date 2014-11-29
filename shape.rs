use gl;
use super::vertex::{Attrib, VertexArray, VertexBuffer, IndexBuffer};

pub fn gen_cube(length: f32, offset: f32, a_position: Attrib) -> VertexArray {
    // 8 corners in a cube
    let corner: [(f32,f32,f32), ..8] = {
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
    static VERT_IDX: [uint, ..4*6] = [
        0,1,2,3,
        4,0,6,2,
        5,4,7,6,
        1,5,3,7,
        5,1,4,0,
        3,7,2,6
    ];

    // Which vertices to form triangle faces from
    static IDX: [u8, ..6*6] = [
        0,1,2, 1,3,2,
        4,5,6, 5,7,6,
        8,9,10, 9,11,10,
        12,13,14, 13,15,14,
        16,17,18, 17,19,18,
        20,21,22, 21,23,22
    ];

    let mut buffer: Vec<f32> = Vec::new();
    for i in VERT_IDX.iter() {
        let (x,y,z) = corner[*i];
        buffer.push_all(&[x,y,z]);
    }

    VertexArray::new(|vao_ctx| {
        let ibo = IndexBuffer::rc_from_slice::<u8>(IDX.as_slice(), gl::UNSIGNED_BYTE);
        let vbo = VertexBuffer::rc_from_slice::<f32>(buffer.as_slice());

        vao_ctx.bind_ibo(ibo);
        vao_ctx.bind_vbo(vbo, |vbo_ctx| {
            vbo_ctx.attr_pointer(a_position, 3, gl::FLOAT, 0, 0);
        });
    })
}
