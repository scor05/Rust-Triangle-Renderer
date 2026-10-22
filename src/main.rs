mod algorithms;
mod framebuffer;
mod line;
mod model;

use algorithms::barycentric::FilledBarycentric;
use algorithms::scanline::FilledScanline;
use algorithms::triangle::{Triangle, TriangleMode, TrianglePipeline};
use algorithms::wireframe::WireFrame;
use framebuffer::Framebuffer;
use model::Model;
use raylib::prelude::*;
use std::f32::consts::PI;
use std::hint::black_box;
use std::time::Instant;

const MODEL_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/obj/nave.obj");
const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = PI / 100.0;
const PIXELS_PER_UNIT: f32 = 25.0;

fn matrix_mult(mat: Vec<Vector3>, vec: Vector3) -> Vector3 {
    let mut result = Vector3::zero();
    result.x += mat[0].x * vec.x + mat[0].y * vec.y + mat[0].z * vec.z;
    result.y += mat[1].x * vec.x + mat[1].y * vec.y + mat[1].z * vec.z;
    result.z += mat[2].x * vec.x + mat[2].y * vec.y + mat[2].z * vec.z;

    result
}

fn transform(
    vertex: Vector3,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    center: Vector3,
) -> Vector3 {
    let mut new_vertex = vertex;
    new_vertex -= center;

    if rotation.x != 0.0 {
        let (sin_theta, cos_theta) = rotation.x.sin_cos();
        let rot_matrix: Vec<Vector3> = vec![
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, cos_theta, -sin_theta),
            Vector3::new(0.0, sin_theta, cos_theta),
        ];

        new_vertex = matrix_mult(rot_matrix, new_vertex);
    }

    if rotation.y != 0.0 {
        let (sin_theta, cos_theta) = rotation.y.sin_cos();
        let rot_matrix: Vec<Vector3> = vec![
            Vector3::new(cos_theta, 0.0, sin_theta),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-sin_theta, 0.0, cos_theta),
        ];
        new_vertex = matrix_mult(rot_matrix, new_vertex);
    }

    if rotation.z != 0.0 {
        let (sin_theta, cos_theta) = rotation.z.sin_cos();
        let rot_matrix: Vec<Vector3> = vec![
            Vector3::new(cos_theta, -sin_theta, 0.0),
            Vector3::new(sin_theta, cos_theta, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];

        new_vertex = matrix_mult(rot_matrix, new_vertex);
    }

    new_vertex *= scale;

    new_vertex += translation;
    new_vertex
}

fn render(
    framebuffer: &mut Framebuffer,
    _translation: Vector3,
    _scale: f32,
    _rotation: Vector3,
    _model: &Model,
    pipeline: &dyn TrianglePipeline,
) {
    let triangles = vec![
        // pequeño y grande
        vec![
            Triangle::new(
                Vector3::new(40.0, 40.0, 0.0),
                Vector3::new(90.0, 50.0, 0.0),
                Vector3::new(60.0, 90.0, 0.0),
            ),
            Triangle::new(
                Vector3::new(300.0, 40.0, 0.0),
                Vector3::new(740.0, 100.0, 0.0),
                Vector3::new(560.0, 350.0, 0.0),
            ),
        ],
        // borde horizontal y vertical
        vec![
            Triangle::new(
                Vector3::new(50.0, 160.0, 0.0),
                Vector3::new(180.0, 160.0, 0.0),
                Vector3::new(110.0, 270.0, 0.0),
            ),
            Triangle::new(
                Vector3::new(220.0, 160.0, 0.0),
                Vector3::new(220.0, 300.0, 0.0),
                Vector3::new(360.0, 230.0, 0.0),
            ),
        ],
        // tres vértices y en orden inverso
        vec![
            Triangle::new(
                Vector3::new(420.0, 400.0, 0.0),
                Vector3::new(520.0, 420.0, 0.0),
                Vector3::new(460.0, 500.0, 0.0),
            ),
            Triangle::new(
                Vector3::new(460.0, 500.0, 0.0),
                Vector3::new(520.0, 420.0, 0.0),
                Vector3::new(420.0, 400.0, 0.0),
            ),
        ],
        // afuera del buffer y en misma línea
        vec![
            Triangle::new(
                Vector3::new(-60.0, 340.0, 0.0),
                Vector3::new(150.0, 380.0, 0.0),
                Vector3::new(30.0, 550.0, 0.0),
            ),
            Triangle::new(
                Vector3::new(500.0, 520.0, 0.0),
                Vector3::new(600.0, 545.0, 0.0),
                Vector3::new(700.0, 570.0, 0.0),
            ),
        ],
        // vértices en misma línea
    ];

    for tri in &triangles[3] {
        pipeline.render(framebuffer, tri);
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Triangle Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    let model = Model::load(MODEL_PATH)
        .unwrap_or_else(|error| panic!("Could not load {MODEL_PATH}: {error}"));

    let wireframe = WireFrame;
    let scanline = FilledScanline;
    let barycentric = FilledBarycentric;

    let mode = TriangleMode::FilledScanline;
    let pipeline: &dyn TrianglePipeline = match mode {
        TriangleMode::Wireframe => &wireframe,
        TriangleMode::FilledBarycentric => &barycentric,
        TriangleMode::FilledScanline => &scanline,
    };

    let translation = Vector3::new(0.0, 0.0, 0.0);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let scale = 1.0_f32;

    while !window.window_should_close() {
        /*
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y -= MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            translation.y += MOVE_SPEED;
        }

        if window.is_key_pressed(KeyboardKey::KEY_Q) {
            scale += 0.1;
        }
        if window.is_key_pressed(KeyboardKey::KEY_E) {
            scale = (scale - 0.1).max(0.5);
        }

        if window.is_key_down(KeyboardKey::KEY_A) {
            rotation.y -= ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            rotation.y += ROT_SPEED;
        }

        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.x += ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            rotation.x -= ROT_SPEED;
        }

        if window.is_key_down(KeyboardKey::KEY_Z) {
            rotation.z -= ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_X) {
            rotation.z += ROT_SPEED;
        }
        */

        framebuffer.clear();

        render(
            &mut framebuffer,
            translation,
            scale,
            rotation,
            &model,
            pipeline,
        );

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
