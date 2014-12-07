use std;
use gl;
use gl::types::{GLint, GLuint, GLsizei};

pub struct Texture2D {
    pub id: GLuint
}
impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}
impl Texture2D {
    pub fn bind(&self, unit: u32) {
        check_max_texture_image_units(unit);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

#[cfg(debug)]
fn check_max_texture_image_units(unit: u32) {
    let max_texture_image_units = unsafe {
        let mut i = 0;
        gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut i);
        i as u32
    };
    if unit >= max_texture_image_units {
        panic!("Unit \"{}\" exceeds max texture image units of \"{}\"", unit, max_texture_image_units);
    }
}

#[cfg(not(debug))]
fn check_max_texture_image_units(_unit: u32) {
    // Do nothing
}
