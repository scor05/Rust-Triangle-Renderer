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

fn triangle(framebuffer: &mut Framebuffer, t1: Vector3, t2: Vector3, t3: Vector3) {
    line(framebuffer, t1, t2);
    line(framebuffer, t2, t3);
    line(framebuffer, t3, t1);
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

    let cos_theta = (rotation * PI / 180.0).cos();
    let sin_theta = (rotation * PI / 180.0).sin();

    let rotated_x = new_vertex.x * cos_theta - new_vertex.y * sin_theta;
    let rotated_y = new_vertex.x * sin_theta + new_vertex.y * cos_theta;

    new_vertex.x = rotated_x;
    new_vertex.y = rotated_y;

    new_vertex *= scale;
    new_vertex += translation;

    new_vertex += center;

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
        const MOVE_SPEED: f32 = 2.0;

        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y += MOVE_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            translation.y -= MOVE_SPEED;
        }

        if window.is_key_pressed(KeyboardKey::KEY_Q) {
            scale += 0.1;
        }
        if window.is_key_pressed(KeyboardKey::KEY_E) {
            scale = (scale - 0.1).max(0.5);
        }

        if window.is_key_down(KeyboardKey::KEY_A) {
            rotation.x -= 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            rotation.x += 2.0;
        }

        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.y -= 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            rotation.y += 2.0;
        }

        if window.is_key_down(KeyboardKey::KEY_Z) {
            rotation.z -= 2.0;
        }
        if window.is_key_down(KeyboardKey::KEY_X) {
            rotation.z += 2.0;
        }

        framebuffer.clear();
        render(&mut framebuffer, translation, scale, rotation, &model);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
