use std::sync::Arc;
use vulkano::{
    image::{view::ImageView, Image},
    pipeline::{
        graphics::viewport::Viewport,
        GraphicsPipeline,
    },
    render_pass::{Framebuffer, RenderPass},
    swapchain::Swapchain,
    sync::GpuFuture,
};
use winit::window::Window;

pub struct RenderContext {
    pub window: Arc<Window>,
    pub swapchain: Arc<Swapchain>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub viewport: Viewport,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

pub fn window_size_dependent_setup(
    images: &[Arc<Image>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images.iter().map(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(
            render_pass.clone(),
            vulkano::render_pass::FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            },
        ).unwrap()
    }).collect()
}