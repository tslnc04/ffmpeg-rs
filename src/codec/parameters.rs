use std::{any::Any, sync::Arc};

use super::Context;
use ffi::*;
use libc::c_int;
use media;
use {codec, format};

pub struct Parameters {
    ptr: *mut AVCodecParameters,
    owner: Option<Arc<dyn Any>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    pub unsafe fn wrap(ptr: *mut AVCodecParameters, owner: Option<Arc<dyn Any>>) -> Self {
        Parameters { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr
    }
}

impl Parameters {
    pub fn new() -> Self {
        unsafe {
            Parameters {
                ptr: avcodec_parameters_alloc(),
                owner: None,
            }
        }
    }

    #[deprecated(since = "5.1.1", note = "Use codec_type instead")]
    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    #[deprecated(since = "5.1.1", note = "Use codec_id instead")]
    pub fn id(&self) -> codec::Id {
        unsafe { codec::Id::from((*self.as_ptr()).codec_id) }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width as u32 }
    }

    #[inline]
    pub fn set_width(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).width = value as c_int;
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height as u32 }
    }

    #[inline]
    pub fn set_height(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).height = value as c_int;
        }
    }

    #[inline]
    pub fn format(&self) -> codec::AVPixelFormat {
        unsafe { std::mem::transmute::<_, AVPixelFormat>((*self.as_ptr()).format) }
    }

    #[inline]
    pub fn set_format(&mut self, format: format::Pixel) {
        let format: codec::AVPixelFormat = format.into();
        unsafe {
            (*self.as_mut_ptr()).format = format as c_int;
        }
    }

    #[inline]
    pub fn codec_type(&self) -> media::Type {
        unsafe { (*self.as_ptr()).codec_type.into() }
    }

    #[inline]
    pub fn set_codec_type(&mut self, codec_type: media::Type) {
        unsafe {
            (*self.as_mut_ptr()).codec_type = codec_type.into();
        }
    }

    #[inline]
    pub fn codec_id(&self) -> codec::Id {
        unsafe { (*self.as_ptr()).codec_id.into() }
    }

    #[inline]
    pub fn set_codec_id(&mut self, codec_id: codec::Id) {
        unsafe {
            (*self.as_mut_ptr()).codec_id = codec_id.into();
        }
    }

    #[inline]
    pub fn extradata(&mut self) -> &[u8] {
        let extradata_size = unsafe { (*self.as_ptr()).extradata_size } as usize;
        unsafe { std::slice::from_raw_parts((*self.as_ptr()).extradata, extradata_size) }
    }

    #[inline]
    pub fn set_extradata(&mut self, mut extradata: Vec<u8>) {
        unsafe {
            (*self.as_mut_ptr()).extradata_size = extradata.len() as libc::c_int;
        }
        extradata.extend(vec![0; AV_INPUT_BUFFER_PADDING_SIZE as usize]);
        let mut slice = extradata.into_boxed_slice();
        let ptr = slice.as_mut_ptr();

        // Leave the memory untracked so it can be freed when the parameters are
        // dropped through the avcodec_parameters_free call.
        std::mem::forget(slice);
        unsafe {
            (*self.as_mut_ptr()).extradata = ptr;
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Parameters {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_parameters_free(&mut self.as_mut_ptr());
            }
        }
    }
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        let mut ctx = Parameters::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl<C: AsRef<Context>> From<C> for Parameters {
    fn from(context: C) -> Parameters {
        let mut parameters = Parameters::new();
        let context = context.as_ref();
        unsafe {
            avcodec_parameters_from_context(parameters.as_mut_ptr(), context.as_ptr());
        }
        parameters
    }
}
