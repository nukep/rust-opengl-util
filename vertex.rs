use std;
use std::rc::Rc;
use gl;
use gl::types::{GLuint, GLint, GLsizei, GLenum};

pub struct VertexArray {
    vao: VertexArrayVAO,

    // VBOs must be ref-counted because of the many-to-many relationship with VAOs and VBOs
    vbo_refs: Vec<Rc<VertexBuffer>>,
    // IBOs have a one-to-many relationship with VAOs
    ibo_ref: Option<Rc<IndexBuffer>>
}
struct VertexArrayVAO {
    id: GLuint
}
impl Drop for VertexArrayVAO {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) };
    }
}
impl VertexArray {
    pub fn new(cb: |&mut VertexArrayInitContext|) -> VertexArray {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        };

        let mut va = VertexArray {
            vao: VertexArrayVAO {id: vao },
            vbo_refs: Vec::new(),
            ibo_ref: None
        };

        {
            let mut ctx = VertexArrayInitContext { va: &mut va };
            cb(&mut ctx);
        }

        va
    }
    pub fn bind_vao(&self, cb: |&VertexArrayContext|) {
        unsafe { gl::BindVertexArray(self.vao.id) };
        let ctx = VertexArrayContext { va: self };
        cb(&ctx);
    }
    fn add_vbo(&mut self, vbo: Rc<VertexBuffer>) {
        self.vbo_refs.push(vbo);
    }
    fn set_ibo(&mut self, ibo: Rc<IndexBuffer>) {
        self.ibo_ref = Some(ibo);
    }
}

pub struct VertexArrayInitContext<'a> {
    va: &'a mut VertexArray
}
impl<'a> VertexArrayInitContext<'a> {
    pub fn bind_vbo(&mut self, vbo: Rc<VertexBuffer>, cb: |VertexArrayBufferContext|) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id) };
        self.va.add_vbo(vbo);
        cb(VertexArrayBufferContext);
    }
    pub fn bind_ibo(&mut self, ibo: Rc<IndexBuffer>) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo.id) };
        self.va.set_ibo(ibo);
    }
}

pub struct VertexArrayContext<'a> {
    va: &'a VertexArray
}
impl<'a> VertexArrayContext<'a> {
    pub fn draw_arrays(&self, mode: GLuint, first: GLint, count: GLsizei) {
        unsafe { gl::DrawArrays(mode, first, count) };
    }
    pub fn draw_elements(&self, mode: GLuint, count: GLsizei, offset: uint) {
        let data_type = self.va.ibo_ref.as_ref().expect("No IBO is bound to the VAO").data_type;
        unsafe { gl::DrawElements(mode, count, data_type, std::ptr::null().offset(offset as int)) };
    }
}

pub struct VertexArrayBufferContext;
impl VertexArrayBufferContext {
    pub fn attr_pointer(&self, a: Attrib, data_size: GLint, data_type: GLuint, stride: GLsizei, offset: uint) {
        unsafe {
            gl::EnableVertexAttribArray(a.id);
            gl::VertexAttribPointer(a.id, data_size, data_type, gl::FALSE, stride, std::ptr::null().offset(offset as int));
        };
    }
}

pub struct VertexBuffer {
    id: GLuint
}
impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}
impl VertexBuffer {
    pub fn from_slice<T>(s: &[T]) -> VertexBuffer {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (s.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                           std::mem::transmute(s.as_ptr()),
                           gl::STATIC_DRAW);
        }

        VertexBuffer { id: vbo }
    }

    pub fn rc_from_slice<T>(s: &[T]) -> Rc<VertexBuffer> {
        Rc::new(VertexBuffer::from_slice(s))
    }
}

pub struct IndexBuffer {
    id: GLuint,
    data_type: GLenum
}
impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}
impl IndexBuffer {
    pub fn from_slice<T>(s: &[T], data_type: GLenum) -> IndexBuffer {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (s.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                           std::mem::transmute(s.as_ptr()),
                           gl::STATIC_DRAW);
        }

        IndexBuffer { id: vbo, data_type: data_type }
    }

    pub fn rc_from_slice<T>(s: &[T], data_type: GLenum) -> Rc<IndexBuffer> {
        Rc::new(IndexBuffer::from_slice(s, data_type))
    }
}

#[deriving(Copy)]
pub struct Attrib {
    pub id: GLuint
}
