use camera::*;
use gpu::{ContextBuilder, ContextDisplay, RasterProgram, FragmentShader, VertexShader, VertexArrayObject, Framebuffer, RasterGeometry, Image2D, ImageFormat, ColorFormat, Sampler, SamplingWrapping, Wrapping, SamplingInterpolation, Interpolation};
use camera_capture;
use camera_capture::Frame;
use image::ImageBuffer;

pub struct Renderer {
    context: gpu::Context,
    raster_program: gpu::RasterProgram,
    vao: gpu::VertexArrayObject,
    framebuffer: gpu::Framebuffer,
    image_2d: gpu::Image2D,
    _sampler: gpu::Sampler,
    resolution: (usize, usize),
    image_format: ImageFormat
}

impl Renderer {
    pub fn new(resolution: (usize, usize)) -> Self {
        let context = ContextBuilder::new().with_display(ContextDisplay::Screen).build();
        context.make_current().expect("Couldn't make current.");

        let fragment_shader = FragmentShader::new(&context, include_str!("fragment.glsl")).expect("Couldn't create FragmentShader.");
        let vertex_shader = VertexShader::new(&context, include_str!("vertex.glsl")).expect("Couldn't create VertexShader.");
        let raster_program = RasterProgram::new(&context, &fragment_shader, &vertex_shader).expect("Couldn't create RasterProgram.");
        let vao = VertexArrayObject::new(&context);
        let framebuffer = Framebuffer::default(&context);

        let image_format = ImageFormat::new(ColorFormat::RGB, gpu::Type::U8);
        let image_2d = Image2D::allocate(&context, resolution, &image_format);
        let sampler = Sampler::new(&context, &image_2d.image, SamplingWrapping::new(Wrapping::Repeat, Wrapping::Repeat, Wrapping::Repeat), SamplingInterpolation::new(Interpolation::Linear, Interpolation::Linear));

        let context_resolution = context.inner_dimensions();
        raster_program.program.bind_sampler(&sampler, 0);
        raster_program.program.bind_vec2((context_resolution.0 as f32, context_resolution.1 as f32), 1);

        Self { context, raster_program, vao, framebuffer, image_2d, _sampler: sampler, resolution, image_format }
    }

    pub fn render(&mut self, image: &ImageBuffer<image::Rgb<u8>, Frame>) {
        self.image_2d.set_data(self.resolution, &self.image_format, &image, &self.image_format);
        self.raster_program.raster(&self.framebuffer, &self.vao, RasterGeometry::Points, 1);
        self.context.swap_buffers().ok();
    }

    pub fn run(&mut self) -> bool {
        self.context.run()
    }
}

fn main() {
    let resolution = (640, 480);
    let mut renderer = Renderer::new(resolution);

    let camera = camera_capture::create(0).expect("Couldn't create camera.");
    let mut capturer = camera.fps(30.0).unwrap().resolution(resolution.0 as u32, resolution.1 as u32).unwrap().start().unwrap();

    while renderer.run() {
        if let Some(image) = capturer.next() {
            renderer.render(&image);
        }
    }
}