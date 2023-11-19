use essay_plot_api::{Canvas, driver::FigureApi, Bounds, Clip};
use wgpu::TextureView;
use image::{ImageBuffer, Rgba};

use crate::{PlotCanvas, PlotRenderer};

pub fn draw_hardcopy(
    width: u32,
    height: u32,
    figure: &mut dyn FigureApi,
    path: impl AsRef<std::path::Path>,
) {
    let mut wgpu = WgpuHardcopy::new(width, height);

    let mut plot_canvas = PlotCanvas::new(
        &wgpu.device,
        wgpu.texture.format(),
    );

    plot_canvas.set_canvas_bounds(width, height);
    plot_canvas.set_scale_factor(4. / 3.);

    let view = wgpu.create_view();
    wgpu.clear_screen(&view.view);

    let mut plot_renderer = PlotRenderer::new(
        &mut plot_canvas, 
        &wgpu.device, 
        Some(&wgpu.queue), 
        Some(&view.view)
    );

        //canvas.clear_screen(&view);

    let bounds = Bounds::<Canvas>::from([
        (0., 0.),
        (width as f32, height as f32)
    ]);

    figure.draw(&mut plot_renderer, &bounds);
        //plot_renderer.draw_path(path, style, &Clip::None).unwrap();

    plot_renderer.flush_inner(&Clip::None);
    view.flush();

    wgpu.save(path);
}

pub(crate) struct WgpuHardcopy {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) device: wgpu::Device,
    pub(crate) texture: wgpu::Texture,
    pub(crate) texture_size: wgpu::Extent3d,
    pub(crate) queue: wgpu::Queue,
}

impl WgpuHardcopy {
    pub fn new(width: u32, height: u32) -> WgpuHardcopy {
        // let event_loop = EventLoop::new();
        let wgpu_canvas = pollster::block_on(init_wgpu_args(width, height));

        // wgpu_canvas.event_loop = Some(event_loop);

        wgpu_canvas
    }

    pub fn draw(
        &mut self, 
        //draw: impl FnOnce(&WgpuHardcopy, &wgpu::TextureView),
        figure: &mut dyn FigureApi,
        path: impl AsRef<std::path::Path>,
    ) {
        let view = self.texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        //self.clear_screen(&view);

        //save(self, &view);

        // frame.present();
        pollster::block_on(self.extract_buffer(path));
    }

    pub fn save(
        &mut self, 
        path: impl AsRef<std::path::Path>,
    ) {
        //let view = self.texture
        //    .create_view(&wgpu::TextureViewDescriptor::default());

        //self.clear_screen(&view);

        // frame.present();
        pollster::block_on(self.extract_buffer(path));
    }

    async fn extract_buffer(
        &mut self,
        path: impl AsRef<std::path::Path>
    ) {
        let u32_size = std::mem::size_of::<u32>() as u32;
        let o_size = (u32_size * self.width * self.height) as wgpu::BufferAddress;

        let o_desc = wgpu::BufferDescriptor {
            size: o_size,
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };

        let o_buffer = self.device.create_buffer(&o_desc);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &o_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * self.width),
                    rows_per_image: Some(self.height),
                }
            },
            self.texture_size,
        );

        self.queue.submit(Some(encoder.finish()));

        {
            let buffer_slice = o_buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });

            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
                self.width, 
                self.height, 
                data
            ).unwrap();

            //buffer_slice.get_mapped_range()

            buffer.save(path).unwrap()
        }
    }

    pub fn create_view(&mut self) -> CanvasView {
        let view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        CanvasView {
            // frame,
            view
        }
    }

    pub(crate) fn clear_screen(&self, view: &wgpu::TextureView) {

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
    }

    pub(crate) fn set_stale(&self) {
        
    }
}

pub struct CanvasView {
    //frame: SurfaceTexture,
    pub(crate) view: TextureView,
}

impl CanvasView {
    pub(crate) fn flush(self) {
        //self.frame.present();
    }
}

async fn init_wgpu_args(width: u32, height: u32) -> WgpuHardcopy {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find adapter");

    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .expect("Failed to create device");

    let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

    let texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[texture_format],
        label: None,
    };
    let texture = device.create_texture(&texture_desc);

    /*
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);
    */

    //let size = texture_desc.size;

    WgpuHardcopy {
        width,
        height,
        device,
        texture,
        texture_size: texture_desc.size,
        queue,
    }
}
