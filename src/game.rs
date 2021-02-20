use super::gpu::{
    pipelines::default::{DefaultPipeline, Instance, Mesh, MeshData, Vertex, View},
    GpuInfo,
};
use std::sync::{Arc, Mutex};
use winit::window::Window;

pub struct Game {
    gpu_info: Arc<Mutex<GpuInfo>>,
    default_pipeline: DefaultPipeline,

    // Test things
    test_mesh: Mesh,
    test: f32,
}

impl Game {
    pub async fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        let mut default_pipeline =
            DefaultPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity()));

        // Test mesh: square with different colored vertices
        let test_mesh = default_pipeline
            .create_mesh(&MeshData {
                vertices: &[
                    Vertex::new([0.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new([1.0, 0.0], [1.0, 0.0, 1.0, 1.0]),
                    Vertex::new([0.0, 1.0], [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new([1.0, 1.0], [1.0, 0.0, 0.0, 1.0]),
                ],
                indices: &[0, 1, 2, 2, 1, 3],
            })
            .unwrap();

        Self {
            gpu_info,
            default_pipeline,

            test_mesh,
            test: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        println!("test: {}", self.test);
        self.test += delta_time;
    }

    pub fn render(&mut self, window: &Window) {
        // Update camera
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        self.default_pipeline.view =
            View::new(na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
                na::Vector2::new(1.0, aspect)
            } else {
                na::Vector2::new(1.0 / aspect, 1.0)
            }));

        // Get target frame to render
        let target = &self
            .gpu_info
            .lock()
            .unwrap()
            .swapchain
            .get_current_frame()
            .unwrap()
            .output
            .view;

        // Do rendering
        self.default_pipeline.render(
            target,
            &mut self.test_mesh,
            &[Instance::new(
                na::Matrix3::identity()
                    .append_scaling(0.3)
                    .append_translation(&na::Vector2::new(self.test.sin(), 0.1)),
            )],
        );
    }
}
