use wgpu::{VertexBufferLayout, Buffer, Device, BufferSlice};
use wgpu::util::DeviceExt;
use std::marker::PhantomData;

pub trait GPUSerializable: Sized {
    fn gpu_serialize(data: &[Self]) -> &[u8];

    fn buffer_layout<'a>() -> VertexBufferLayout<'a>;
}

pub struct GPUBuffer<T: GPUSerializable> {
    buffer: Buffer,
    num_items: u32,
    _phantom: PhantomData<T>,
}

impl<T: GPUSerializable> GPUBuffer<T> {
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

        GPUBuffer {
            buffer,
            num_items: data.len() as u32,
            _phantom: Default::default(),
        }
    }
}

