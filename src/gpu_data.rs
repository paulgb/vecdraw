use std::cell::RefMut;
use std::marker::PhantomData;
use std::mem::size_of;
use wgpu::BufferDescriptor;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferSlice, CommandEncoder, Device, VertexBufferLayout};

pub trait GpuSerializable: Sized {
    fn gpu_serialize(data: &[Self]) -> &[u8];

    fn buffer_layout<'a>() -> VertexBufferLayout<'a>;
}

pub struct GpuBuffer<T: GpuSerializable> {
    buffer: Buffer,
    num_items: u32,
    capacity: u32,
    _phantom: PhantomData<T>,
}

impl<T: GpuSerializable> GpuBuffer<T> {
    pub fn len(&self) -> u32 {
        self.num_items
    }

    pub fn all(&self) -> BufferSlice {
        self.buffer.slice(..)
    }

    fn create_buffer(capacity: u32, device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<T>() as u64 * capacity as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn new_with_capacity(capacity: u32, device: &Device) -> Self {
        let buffer = Self::create_buffer(capacity, device);

        GpuBuffer {
            buffer,
            num_items: 0,
            capacity,
            _phantom: PhantomData::default(),
        }
    }

    pub fn new(data: &[T], device: &Device) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            contents: T::gpu_serialize(data),
            label: None,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST
        });

        let capacity = data.len() as u32;

        GpuBuffer {
            buffer,
            capacity,
            num_items: data.len() as u32,
            _phantom: PhantomData::default(),
        }
    }

    pub fn update(&mut self, data: &[T], device: &Device, encoder: RefMut<CommandEncoder>) {
        if data.len() as u32 > self.capacity {
            let capacity = data.len() as u32;
            let buffer = Self::create_buffer(capacity, device);

            self.capacity = capacity;
            self.buffer = buffer;
        }

        let tmp_buffer = device.create_buffer_init(&BufferInitDescriptor {
            contents: T::gpu_serialize(data),
            label: None,
            usage: wgpu::BufferUsage::COPY_SRC,
        });

        self.num_items = data.len() as u32;

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
