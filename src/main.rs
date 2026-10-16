mod framebuffer;
mod line;
mod model;

use framebuffer::Framebuffer;
use line::line;
use model::Model;
use obj::Obj;
use raylib::prelude::*;
use std::f32::consts::PI;

const MODEL_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/obj/nave.obj");
const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = PI / 100.0;

fn triangle(framebuffer: &mut Framebuffer, t1: Vector3, t2: Vector3, t3: Vector3) {
    line(framebuffer, t1, t2);
    line(framebuffer, t2, t3);
    line(framebuffer, t3, t1);
}

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
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    model: &Model,
) {
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

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0_f32;

    while !window.window_should_close() {
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
            rotation.x -= ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            rotation.x += ROT_SPEED;
        }

        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.y -= ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            rotation.y += ROT_SPEED;
        }

        if window.is_key_down(KeyboardKey::KEY_Z) {
            rotation.z -= ROT_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_X) {
            rotation.z += ROT_SPEED;
        }

        framebuffer.clear();
        render(&mut framebuffer, translation, scale, rotation, &model);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
