use super::triangle::{Triangle, TrianglePipeline};
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub struct WireFrame;

impl TrianglePipeline for WireFrame {
    fn render(&self, framebuffer: &mut Framebuffer, triangle: &Triangle) {
        let [first, second, third] = triangle.vertices;

        line(framebuffer, first, second);
        line(framebuffer, second, third);
        line(framebuffer, third, first);
    }
}
