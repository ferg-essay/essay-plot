use essay_plot_api::TextureId;


pub struct TextureCache {
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,

    store: TextureStore,

    texture_boxes: Vec<TextureBox>,
    texture_items: Vec<TextureItem>,

    is_dirty: bool,
}

impl TextureCache {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        assert!(width % 256 == 0);

        let texture = create_texture(device, width, height);
        let layout = create_bind_group_layout(device);
        let bind_group = create_bind_group(device, &layout, &texture);

        let store = TextureStore::new(width, height);

        Self {
            texture,
            bind_group,
            layout,

            store,
            texture_boxes: Vec::new(),
            texture_items: Vec::new(),

            is_dirty: true,
        }
    }

    pub fn add(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32, 
        height: u32, 
        data: &[u8]
    ) -> TextureId {
        assert!(width * height == data.len() as u32);
        
        let id = TextureId::new(self.texture_items.len());
        
        let mut item = TextureItem::new(device, width, height);

        item.write(queue, data);

        self.texture_items.push(item);

        id
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
    
    pub(crate) fn texture_bind_group(&self, id: TextureId) -> &wgpu::BindGroup {
        &self.texture_items[id.index()].bind_group
    }

    pub fn flush(&mut self, queue: &wgpu::Queue) {
        if self.is_dirty {
            self.is_dirty = false;

            write_texture(
                queue, 
                &self.texture, 
                self.store.data.as_slice(), 
                self.store.width as u32, 
                self.store.height as u32,
            );
        }
    }
}

struct TextureItem {
    texture: wgpu::Texture,
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl TextureItem {
    fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = create_texture(device, width, height);
        let layout = create_bind_group_layout(device);
        let bind_group = create_bind_group(device, &layout, &texture);

        Self {
            texture,
            layout,
            bind_group,
        }
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn write(&mut self, queue: &wgpu::Queue, data: &[u8]) {
        write_texture(
            queue, 
            &self.texture, 
            data,
            self.texture.width(),
            self.texture.height(),
        );
    }
}

struct TextureStore {
    width: usize,
    height: usize,

    data: Vec<u8>,

    tail: usize,
    cursors: Vec<TextureCursor>,
}

impl TextureStore {
    fn new(width: u32, height: u32) -> Self {
        assert!(width > 0 && width % 256 == 0);
        assert!(height > 0);

        let mut data = Vec::new();
        data.resize((width * height) as usize, 0);

        Self {
            width: width as usize,
            height: height as usize,
            data,
            tail: 0,
            cursors: Vec::new(),
        }
    }

    fn add(&mut self, width: usize, height: usize, data: &[u8]) -> TextureBox {
        let cursor = self.cursor(width, height);

        let (x, y) = (cursor.x(), cursor.y());
        let c_w = cursor.width;

        // let offset = 0;//cursor.offset;

        for j in 0..height {
            for i in 0..width {
                self.data[x + i + (j + y) * c_w] = data[i + j * width];
            }
        }

        TextureBox::new(x, y, width, height)
    }

    fn cursor(&mut self, width: usize, height: usize) -> TextureCursor {
        let height = height.max(1);

        let height_chunk = height + 31;
        let height_chunk = height_chunk - height_chunk % 32;

        let len = self.cursors.len();
        for i in (0..len).rev() {
            if self.cursors[i].height == height_chunk {
                if width <= self.cursors[i].width - self.cursors[i].x {
                    return self.cursors[i].add_x(width);
                }

                self.cursors.remove(i);
            }
        }

        return self.add_cursor(height_chunk).add_x(width);
    }

    fn add_cursor(&mut self, height: usize) -> &mut TextureCursor {
        assert!(height > 0 && height % 32 == 0);

        let len = self.cursors.len();
        self.cursors.push(TextureCursor::new(self.width, height, self.tail));

        self.tail += self.width * height;

        if self.data.len() < self.tail {
            self.height = 2 * self.height;

            self.data.resize(self.width * self.height, 0);
        }

        let cursor = &mut self.cursors[len];

        let n_x = cursor.x + 1;
        if n_x % 4 > 0 {
            cursor.x += 4 - cursor.x % 4;
        }

        cursor
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextureBox {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl TextureBox {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }
}

#[derive(Clone)]
struct TextureCursor {
    width: usize,
    height: usize,

    offset: usize,

    x: usize,
}

impl TextureCursor {
    fn new(width: usize, height: usize, offset: usize) -> Self {
        TextureCursor {
            width,
            height,
            offset,
            x: 0,
        }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.offset / self.width
    }

    fn add_x(&mut self, width: usize) -> Self {
        let cursor = self.clone();

        let mut n_x = self.x + width + 1;
        if n_x % 4 > 0 {
            n_x += 4 - n_x % 4;
        }

        self.x = n_x;

        cursor
    }
}

fn create_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size(width, height),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING 
                | wgpu::TextureUsages::COPY_DST,
            label: Some("text_texture"),
            view_formats: &[],
        }
    )
}

fn texture_size(width: u32, height: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    }
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    })
}

fn create_bind_group(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::Texture
) -> wgpu::BindGroup {
    let text_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // wgpu::AddressMode::ClampToEdge
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        .. Default::default()
    });

    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&text_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            label: Some("text_bind_group")
        }
    )
}

fn write_texture(
    queue: &wgpu::Queue, 
    texture: &wgpu::Texture, 
    data: &[u8], 
    width: u32, 
    height: u32) {
    //assert!(width % 256 == 0);

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(width),
            rows_per_image: Some(height),
        },
        texture_size(width, height),
    );
}
