use std::cell::RefMut;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferSlice, CommandEncoder, Device, VertexBufferLayout};

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
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        GpuBuffer {
            buffer,
            num_items: data.len() as u32,
            _phantom: Default::default(),
        }
    }

    pub fn update(&self, data: &[T], device: &Device, encoder: RefMut<CommandEncoder>) {
        assert_eq!(
            self.num_items,
            data.len() as u32,
            "Updates with different number of items are not yet supported."
        );

        let tmp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: T::gpu_serialize(data),
            usage: wgpu::BufferUsage::COPY_SRC,
        });

        let mut encoder = encoder;
        encoder.copy_buffer_to_buffer(
            &tmp_buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<T>() as u64 * data.len() as u64,
        );
    }
}
