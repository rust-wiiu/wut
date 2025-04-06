use super::attribute::AttributeFormat;
use crate::{bindings as c, rrc::RrcGuard};
use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use flagset::{flags, FlagSet};

flags! {
    pub enum ResourceFlags: c::GX2RResourceFlags::Type {
        /// No resource flags specified.
        None = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_NONE,

        /// This resource is to be used as a texture.
        BindTexture = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_TEXTURE,

        /// This resource is to be used as a color buffer.
        BindColorBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_COLOR_BUFFER,

        /// This resource is to be used as a depth buffer.
        BindDepthBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_DEPTH_BUFFER,

        /// This resource is to be used as a scan buffer.
        BindScanBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_SCAN_BUFFER,

        /// This resource is to be used as a vertex buffer.
        BindVertexBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_VERTEX_BUFFER,

        /// This resource is to be used as an index buffer.
        BindIndexBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_INDEX_BUFFER,

        /// This resource is to be used as a uniform block.
        BindUniformBlock = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_UNIFORM_BLOCK,

        /// This resource is to be used as a shader program.
        BindShaderProgram = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_SHADER_PROGRAM,

        /// This resource is to be used as a stream output.
        BindStreamOutput = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_STREAM_OUTPUT,

        /// This resource is to be used as a display list.
        BindDisplayList = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_DISPLAY_LIST,

        /// This resource is to be used as a geometry shader ring buffer.
        BindGSRingBuffer = c::GX2RResourceFlags::GX2R_RESOURCE_BIND_GS_RING_BUFFER,

        /// Invalidate resource for a CPU read.
        UsageCPURead = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_CPU_READ,

        /// Invalidate resource for a CPU write.
        UsageCPUWrite = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_CPU_WRITE,

        /// Invalidate resource for a CPU read and write.
        UsageCPU = (ResourceFlags::UsageCPURead | ResourceFlags::UsageCPUWrite).bits(),

        /// Invalidate resource for a GPU read.
        UsageGPURead = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_GPU_READ,

        /// Invalidate resource for a GPU write.
        UsageGPUWrite = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_GPU_WRITE,

        /// Invalidate resource for a GPU read and write.
        UsageGPU = (ResourceFlags::UsageGPURead | ResourceFlags::UsageGPUWrite).bits(),

        /// Invalidate resource for a DMA read.
        UsageDMARead = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_DMA_READ,

        /// Invalidate resource for a DMA write.
        UsageDMAWrite = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_DMA_WRITE,

        /// Force resource allocation to be in MEM1.
        UsageForceMEM1 = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_FORCE_MEM1,

        /// Force resource allocation to be in MEM2.
        UsageForceMEM2 = c::GX2RResourceFlags::GX2R_RESOURCE_USAGE_FORCE_MEM2,

        /// Disable CPU invalidation.
        DisableCPUInvalidate = c::GX2RResourceFlags::GX2R_RESOURCE_DISABLE_CPU_INVALIDATE,

        /// Disable GPU invalidation.
        DisableGPUInvalidate = c::GX2RResourceFlags::GX2R_RESOURCE_DISABLE_GPU_INVALIDATE,

        /// Resource is locked for read-only access.
        LockedReadOnly = c::GX2RResourceFlags::GX2R_RESOURCE_LOCKED_READ_ONLY,

        /// Resource was allocated by GX2R.
        GX2RAllocated = c::GX2RResourceFlags::GX2R_RESOURCE_GX2R_ALLOCATED,

        /// Resource is locked for all access.
        Locked = c::GX2RResourceFlags::GX2R_RESOURCE_LOCKED,
    }
}

pub struct Buffer<T: AttributeFormat> {
    buffer: c::GX2RBuffer,
    _resource: RrcGuard,
    _marker: PhantomData<T>,
}

impl<T: AttributeFormat> Buffer<T> {
    fn with_capacity(
        capacity: usize,
        flags: impl Into<FlagSet<ResourceFlags>>,
    ) -> Result<Self, ()> {
        let mut s = Self {
            buffer: c::GX2RBuffer {
                flags: flags.into().bits(),
                elemSize: core::mem::size_of::<T>() as u32,
                elemCount: capacity as u32,
                buffer: core::ptr::null_mut(),
            },
            _resource: super::GFX.acquire(),
            _marker: PhantomData,
        };

        if unsafe { c::GX2RCreateBuffer(&mut s.buffer) } == 0 {
            Err(())
        } else {
            Ok(s)
        }
    }

    pub fn from(values: &[T], flags: impl Into<FlagSet<ResourceFlags>>) -> Result<Self, ()> {
        let mut s = Self::with_capacity(values.len(), flags)?;

        s.lock().unwrap().copy_from_slice(values);

        Ok(s)
    }

    pub fn fill(
        length: usize,
        flags: impl Into<FlagSet<ResourceFlags>>,
        value: T,
    ) -> Result<Self, ()> {
        let mut s = Self::with_capacity(length, flags)?;

        {
            let mut g = s.lock().unwrap();
            for i in 0..length {
                g[i] = value;
            }
        }

        Ok(s)
    }

    pub fn default(size: usize, flags: impl Into<FlagSet<ResourceFlags>>) -> Result<Self, ()> {
        Self::fill(size, flags, T::default())
    }

    pub fn as_raw(&self) -> *const c::GX2RBuffer {
        &self.buffer
    }

    pub fn as_raw_mut(&mut self) -> *mut c::GX2RBuffer {
        &mut self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.elemCount as usize
    }

    pub fn flags(&self) -> FlagSet<ResourceFlags> {
        FlagSet::<ResourceFlags>::new_truncated(self.buffer.flags)
    }

    #[inline]
    pub fn stride(&self) -> usize {
        self.buffer.elemSize as usize
    }

    pub fn lock(&mut self) -> Result<BufferGuard<'_, T>, ()> {
        if self.flags().contains(ResourceFlags::Locked) {
            Err(())
        } else {
            Ok(BufferGuard::new(self))
        }
    }
}

impl<T: AttributeFormat> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            c::GX2RDestroyBufferEx(&mut self.buffer, 0);
        }
    }
}

pub struct BufferGuard<'a, T: AttributeFormat> {
    parent: &'a mut Buffer<T>,
    buffer: *mut core::ffi::c_void,
}

impl<'a, T: AttributeFormat> BufferGuard<'a, T> {
    fn new(parent: &'a mut Buffer<T>) -> Self {
        let buffer = unsafe { c::GX2RLockBufferEx(&mut parent.buffer, 0) };

        Self { parent, buffer }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

impl<'a, T: AttributeFormat> Deref for BufferGuard<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.buffer as *const T, self.len()) }
    }
}

impl<'a, T: AttributeFormat> DerefMut for BufferGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.buffer as *mut T, self.len()) }
    }
}

impl<'a, T: AttributeFormat> Drop for BufferGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            c::GX2RUnlockBufferEx(&mut self.parent.buffer, 0);
        }
    }
}

impl<'a, T: AttributeFormat + core::fmt::Debug> core::fmt::Debug for BufferGuard<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // deref self and debug slice
        f.debug_list().entries(self.deref()).finish()
    }
}
