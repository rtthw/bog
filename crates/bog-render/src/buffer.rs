


use core::num::NonZeroU64;



pub const MAX_WRITE_SIZE: usize = 100 * 1024;

const MAX_WRITE_SIZE_U64: NonZeroU64 = NonZeroU64::new(MAX_WRITE_SIZE as u64)
    .expect("MAX_WRITE_SIZE must be non-zero");



#[derive(Debug)]
pub struct Buffer<T> {
    label: &'static str,
    size: u64,
    usage: gpu::BufferUsages,
    raw: gpu::Buffer,
    offsets: Vec<gpu::BufferAddress>,
    type_: core::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub fn new(
        device: &gpu::Device,
        label: &'static str,
        amount: usize,
        usage: gpu::BufferUsages,
    ) -> Self {
        let size = next_copy_size::<T>(amount);

        let raw = device.create_buffer(&gpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        });

        Self {
            label,
            size,
            usage,
            raw,
            offsets: Vec::new(),
            type_: std::marker::PhantomData,
        }
    }

    pub fn resize(&mut self, device: &gpu::Device, new_count: usize) -> bool {
        let new_size = (std::mem::size_of::<T>() * new_count) as u64;

        if self.size < new_size {
            self.offsets.clear();
            self.raw = device.create_buffer(&gpu::BufferDescriptor {
                label: Some(self.label),
                size: new_size,
                usage: self.usage,
                mapped_at_creation: false,
            });
            self.size = new_size;

            true
        } else {
            false
        }
    }

    /// Returns the size of the written bytes.
    pub fn write(
        &mut self,
        device: &gpu::Device,
        encoder: &mut gpu::CommandEncoder,
        belt: &mut gpu::util::StagingBelt,
        offset: usize,
        contents: &[T],
    ) -> usize {
        let bytes: &[u8] = bytemuck::cast_slice(contents);
        let mut bytes_written = 0;

        // Split write into multiple chunks, if necessary.
        while bytes_written + MAX_WRITE_SIZE < bytes.len() {
            belt.write_buffer(
                encoder,
                &self.raw,
                (offset + bytes_written) as u64,
                MAX_WRITE_SIZE_U64,
                device,
            )
            .copy_from_slice(
                &bytes[bytes_written..bytes_written + MAX_WRITE_SIZE],
            );

            bytes_written += MAX_WRITE_SIZE;
        }

        // There will always be some bytes left, since the previous loop guarantees
        // `bytes_written < bytes.len()`.
        let bytes_left = ((bytes.len() - bytes_written) as u64)
            .try_into()
            .expect("non-empty write");

        belt.write_buffer(
            encoder,
            &self.raw,
            (offset + bytes_written) as u64,
            bytes_left,
            device,
        ).copy_from_slice(&bytes[bytes_written..]);

        self.offsets.push(offset as u64);

        bytes.len()
    }

    pub fn slice(
        &self,
        bounds: impl core::ops::RangeBounds<gpu::BufferAddress>,
    ) -> gpu::BufferSlice<'_> {
        self.raw.slice(bounds)
    }

    /// Returns the slice calculated from the offset stored at `index`.
    pub fn slice_from_index(&self, index: usize) -> gpu::BufferSlice<'_> {
        self.raw.slice(self.offset_at(index)..)
    }

    /// Clears temporary data (offsets) from the buffer.
    pub fn clear(&mut self) {
        self.offsets.clear();
    }

    /// Returns the offset at `index`.
    fn offset_at(&self, index: usize) -> &gpu::BufferAddress {
        self.offsets.get(index)
            .expect("no offset at index")
    }
}

fn next_copy_size<T>(amount: usize) -> u64 {
    let align_mask = gpu::COPY_BUFFER_ALIGNMENT - 1;

    (((core::mem::size_of::<T>() * amount).next_power_of_two() as u64
        + align_mask)
        & !align_mask)
        .max(gpu::COPY_BUFFER_ALIGNMENT)
}
