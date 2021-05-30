use std::marker::PhantomData;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferSlice, Device, VertexBufferLayout};

pub trait GpuSerializable: Sized {
    fn gpu_serialize(data: &[Self]) -> &[u8];

    fn buffer_layout<'a>() -> VertexBufferLayout<'a>;
}

pub struct GpuBuffer<T: GpuSerializable> {
    buffer: Buffer,
    num_items: u32,
    _phantom: PhantomData<T>,
}

impl<T: GpuSerializable> GpuBuffer<T> {
    pub fn len(&self) -> u32 {
        self.num_items
    }

    pub fn all(&self) -> BufferSlice {
        self.buffer.slice(..)
    }

    pub fn new(data: &[T], device: &Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: T::gpu_serialize(data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        GpuBuffer {
            buffer,
            num_items: data.len() as u32,
            _phantom: Default::default(),
        }
    }
}
