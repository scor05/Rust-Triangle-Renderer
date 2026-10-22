use crate::algorithms::triangle::{Triangle, TrianglePipeline};
use crate::framebuffer::Framebuffer;

pub struct FilledScanline;

impl TrianglePipeline for FilledScanline {
    fn render(&self, _framebuffer: &mut Framebuffer, _triangle: &Triangle) {}
}
