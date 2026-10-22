mod algorithms;
mod framebuffer;
mod line;
mod model;

use algorithms::barycentric::FilledBarycentric;
use algorithms::scanline::FilledScanline;
use algorithms::triangle::{Fragment, Triangle, TriangleMode, TrianglePipeline};
use algorithms::wireframe::WireFrame;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::f32::consts::PI;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const MODEL_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/obj/nave.obj");
const MOVE_SPEED: f32 = 0.1;
const ROT_SPEED: f32 = PI / 100.0;
const PIXELS_PER_UNIT: f32 = 25.0;
const BENCHMARK_ITERATIONS: usize = 100;
const BENCHMARK_REPETITIONS: usize = 7;
const SCREENSHOT_DIRECTORY: &str = "screenshots/triangles";
const TEST_GROUP_NAMES: [&str; 4] = [
    "small_and_large",
    "horizontal_and_vertical_edges",
    "original_and_reversed_winding",
    "partially_outside_and_collinear",
];

#[derive(Debug)]
struct BenchmarkResult {
    algorithm: &'static str,
    median: Duration,
    iterations: usize,
    fragments_per_iteration: usize,
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

fn test_triangle_groups() -> Vec<Vec<Triangle>> {
    vec![
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
    ]
}

fn render(
    framebuffer: &mut Framebuffer,
    triangle_groups: &[Vec<Triangle>],
    pipeline: &dyn TrianglePipeline,
) {
    for triangle in triangle_groups.iter().flatten() {
        pipeline.render(framebuffer, triangle);
    }
}

fn render_test_screenshots(
    framebuffer: &mut Framebuffer,
    window: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    triangle_groups: &[Vec<Triangle>],
    pipelines: &[(&str, &dyn TrianglePipeline)],
) -> std::io::Result<Vec<PathBuf>> {
    if triangle_groups.len() != TEST_GROUP_NAMES.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "every triangle test group must have a corresponding scenario name",
        ));
    }

    let output_directory = PathBuf::from(SCREENSHOT_DIRECTORY);
    fs::create_dir_all(&output_directory)?;
    let mut screenshots = Vec::with_capacity(triangle_groups.len() * pipelines.len());

    for (method_name, pipeline) in pipelines {
        for (scenario_name, triangles) in TEST_GROUP_NAMES.iter().zip(triangle_groups) {
            framebuffer.clear();

            for triangle in triangles {
                pipeline.render(framebuffer, triangle);
            }

            // Show the same framebuffer that is about to be exported.
            framebuffer.swap_buffers(window, raylib_thread);

            let file_path = output_directory.join(format!("{scenario_name}__{method_name}.png"));
            framebuffer.render_to_file(file_path.to_string_lossy().as_ref());
            println!("Saved {}", file_path.display());
            screenshots.push(file_path);
        }
    }

    Ok(screenshots)
}

fn benchmark_rasterizer<F>(
    algorithm: &'static str,
    triangle_groups: &[Vec<Triangle>],
    mut rasterize: F,
) -> BenchmarkResult
where
    F: FnMut(&Triangle) -> Vec<Fragment>,
{
    let fragments_per_iteration = triangle_groups
        .iter()
        .flatten()
        .map(|triangle| rasterize(black_box(triangle)).len())
        .sum();

    let mut samples = Vec::with_capacity(BENCHMARK_REPETITIONS);

    for _ in 0..BENCHMARK_REPETITIONS {
        let start = Instant::now();
        let mut generated_fragments = 0_usize;

        for _ in 0..BENCHMARK_ITERATIONS {
            for triangle in black_box(triangle_groups).iter().flatten() {
                let fragments = rasterize(black_box(triangle));
                generated_fragments += fragments.len();
                black_box(fragments);
            }
        }

        black_box(generated_fragments);
        samples.push(start.elapsed());
    }

    samples.sort_unstable();

    BenchmarkResult {
        algorithm,
        median: samples[samples.len() / 2],
        iterations: BENCHMARK_ITERATIONS,
        fragments_per_iteration,
    }
}

fn benchmark_triangles(triangle_groups: &[Vec<Triangle>]) -> Vec<BenchmarkResult> {
    let barycentric = FilledBarycentric;
    let scanline = FilledScanline;

    vec![
        benchmark_rasterizer("barycentric", triangle_groups, |triangle| {
            barycentric.rasterize(triangle)
        }),
        benchmark_rasterizer("scanline", triangle_groups, |triangle| {
            scanline.rasterize(triangle)
        }),
    ]
}

fn print_benchmark_results(results: &[BenchmarkResult]) {
    println!("Triangle rasterization benchmark");

    for result in results {
        println!(
            "{}: median {:?} for {} iterations; {} fragments per iteration",
            result.algorithm, result.median, result.iterations, result.fragments_per_iteration,
        );
    }
}

fn main() {
    let triangle_groups = test_triangle_groups();
    let arguments: Vec<String> = std::env::args().collect();

    if arguments
        .iter()
        .any(|argument| argument == "--benchmark-triangles")
    {
        let results = benchmark_triangles(&triangle_groups);
        print_benchmark_results(&results);
        return;
    }

    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Triangle Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    let wireframe = WireFrame;
    let scanline = FilledScanline;
    let barycentric = FilledBarycentric;

    if arguments
        .iter()
        .any(|argument| argument == "--capture-triangles")
    {
        let pipelines: [(&str, &dyn TrianglePipeline); 3] = [
            ("wireframe", &wireframe),
            ("filled_barycentric", &barycentric),
            ("filled_scanline", &scanline),
        ];

        let screenshots = render_test_screenshots(
            &mut framebuffer,
            &mut window,
            &raylib_thread,
            &triangle_groups,
            &pipelines,
        )
        .expect("could not save triangle test screenshots");

        println!(
            "Saved {} triangle screenshots under {SCREENSHOT_DIRECTORY}",
            screenshots.len()
        );
        return;
    }

    let mode = TriangleMode::FilledScanline;
    let pipeline: &dyn TrianglePipeline = match mode {
        TriangleMode::Wireframe => &wireframe,
        TriangleMode::FilledBarycentric => &barycentric,
        TriangleMode::FilledScanline => &scanline,
    };

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

        render(&mut framebuffer, &triangle_groups, pipeline);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
