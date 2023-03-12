use super::Context;
use ffi::*;
use libc::c_int;
use {Error, Frame, Rational};

pub struct Sink<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Sink<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Sink<'b> {
        Sink { ctx }
    }
}

impl<'a> Sink<'a> {
    pub fn frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_frame(self.ctx.as_mut_ptr(), frame.as_mut_ptr()) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn samples(&mut self, frame: &mut Frame, samples: usize) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_samples(
                self.ctx.as_mut_ptr(),
                frame.as_mut_ptr(),
                samples as c_int,
            ) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn set_frame_size(&mut self, value: u32) {
        unsafe {
            av_buffersink_set_frame_size(self.ctx.as_mut_ptr(), value);
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { av_buffersink_get_w(self.ctx.as_ptr()) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { av_buffersink_get_h(self.ctx.as_ptr()) as u32 }
    }

    pub fn time_base(&self) -> crate::Rational {
        unsafe { Rational::from(av_buffersink_get_time_base(self.ctx.as_ptr())) }
    }

    pub fn frame_rate(&self) -> crate::Rational {
        unsafe { Rational::from(av_buffersink_get_frame_rate(self.ctx.as_ptr())) }
    }

    // TODO(tslnc04): figure out how to deal with the format being either sample
    // format or pixel format

    pub fn sample_rate(&self) -> i32 {
        unsafe { av_buffersink_get_sample_rate(self.ctx.as_ptr()) as i32 }
    }

    pub fn channels(&self) -> i32 {
        unsafe { av_buffersink_get_channels(self.ctx.as_ptr()) as i32 }
    }

    // TODO(tslnc04): figure out how to convert this to the ffmpeg_rs type
    pub fn channel_layout(&self) -> i64 {
        unsafe { av_buffersink_get_channel_layout(self.ctx.as_ptr()) as i64 }
    }
}
