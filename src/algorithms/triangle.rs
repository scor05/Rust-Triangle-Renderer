use crate::framebuffer::Framebuffer;
use raylib::prelude::Vector3;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vector3; 3],
}

impl Triangle {
    pub fn new(first: Vector3, second: Vector3, third: Vector3) -> Self {
        Self {
            vertices: [first, second, third],
        }
    }
}

pub trait TrianglePipeline {
    fn render(&self, framebuffer: &mut Framebuffer, triangle: &Triangle);
}
