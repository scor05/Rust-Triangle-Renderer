use super::triangle::{COVERAGE_EPSILON, Fragment, Triangle, TrianglePipeline};
use crate::framebuffer::Framebuffer;
use raylib::prelude::Vector3;

pub struct FilledBarycentric;

impl FilledBarycentric {
    pub fn rasterize(&self, triangle: &Triangle) -> Vec<Fragment> {
        let [first, second, third] = triangle.vertices;
        let area = edge_function(first, second, third.x, third.y);

        if area.abs() <= f32::EPSILON {
            return Vec::new();
        }

        let min_x = first.x.min(second.x).min(third.x).floor() as i32;
        let max_x = first.x.max(second.x).max(third.x).ceil() as i32;
        let min_y = first.y.min(second.y).min(third.y).floor() as i32;
        let max_y = first.y.max(second.y).max(third.y).ceil() as i32;

        let capacity = (max_x - min_x).max(0) as usize * (max_y - min_y).max(0) as usize;
        let mut fragments = Vec::with_capacity(capacity);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let sample_x = x as f32 + 0.5;
                let sample_y = y as f32 + 0.5;

                let alpha = edge_function(second, third, sample_x, sample_y) / area;
                let beta = edge_function(third, first, sample_x, sample_y) / area;
                let gamma = edge_function(first, second, sample_x, sample_y) / area;

                if alpha >= -COVERAGE_EPSILON
                    && beta >= -COVERAGE_EPSILON
                    && gamma >= -COVERAGE_EPSILON
                {
                    fragments.push(Fragment { x, y });
                }
            }
        }

        fragments
    }
}

impl TrianglePipeline for FilledBarycentric {
    fn render(&self, framebuffer: &mut Framebuffer, triangle: &Triangle) {
        for fragment in self.rasterize(triangle) {
            if fragment.x >= 0 && fragment.y >= 0 {
                framebuffer.set_pixel(fragment.x as u32, fragment.y as u32);
            }
        }
    }
}

fn edge_function(start: Vector3, end: Vector3, x: f32, y: f32) -> f32 {
    (end.x - start.x) * (y - start.y) - (end.y - start.y) * (x - start.x)
}
