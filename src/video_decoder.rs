use crate::{format_context::FormatContext, frame::Frame, packet::Packet, tools};
use ffmpeg_sys_next::*;
use std::{
  ffi::CString,
  ptr::{null, null_mut},
};

#[derive(Debug)]
pub struct VideoDecoder {
  pub identifier: String,
  pub stream_index: isize,
  pub codec_context: *mut AVCodecContext,
  pub hw_pixel_format: Option<AVPixelFormat>,
}

impl VideoDecoder {
  pub fn new(
    identifier: String,
    format: &FormatContext,
    stream_index: isize,
    hw_accel: bool,
  ) -> Result<Self, String> {
    unsafe {
      let codec = avcodec_find_decoder(format.get_codec_id(stream_index));
      let mut codec_context = avcodec_alloc_context3(codec);

      check_result!(
        avcodec_parameters_to_context(
          codec_context,
          (**(*format.format_context).streams.offset(stream_index)).codecpar
        ),
        {
          avcodec_free_context(&mut codec_context);
        }
      );

      let hw_pixel_format = if hw_accel {
        let hw_configs = get_hw_configs(codec_context);
        log::debug!("Available hw_configs: {:?}", hw_configs);
        let picked_hw_config = hw_configs[0];
        log::debug!("Picked hw_config: {:?}", picked_hw_config);

        let mut hw_device_ctx = null_mut();
        (*codec_context).get_format = Some(find_pixel_format);

        av_hwdevice_ctx_create(
          &mut hw_device_ctx,
          picked_hw_config.0,
          null(),
          null_mut(),
          0,
        );
        (*codec_context).hw_device_ctx = av_buffer_ref(hw_device_ctx as _);
        Some(picked_hw_config.1)
      } else {
        None
      };

      check_result!(avcodec_open2(codec_context, codec, null_mut()), {
        avcodec_free_context(&mut codec_context);
      });

      Ok(VideoDecoder {
        identifier,
        stream_index,
        codec_context,
        hw_pixel_format,
      })
    }
  }

  pub fn new_with_codec(
    identifier: String,
    codec_name: &str,
    width: i32,
    height: i32,
    stream_index: isize,
  ) -> Result<Self, String> {
    unsafe {
      let cn = CString::new(codec_name).unwrap();
      let codec = avcodec_find_decoder_by_name(cn.as_ptr());
      let mut codec_context = avcodec_alloc_context3(codec);

      (*codec_context).width = width;
      (*codec_context).height = height;
      check_result!(avcodec_open2(codec_context, codec, null_mut()), {
        avcodec_free_context(&mut codec_context);
      });

      Ok(VideoDecoder {
        identifier,
        stream_index,
        codec_context,
        hw_pixel_format: Some(AVPixelFormat::AV_PIX_FMT_NONE),
      })
    }
  }

  pub fn get_width(&self) -> i32 {
    unsafe { (*self.codec_context).width }
  }

  pub fn get_height(&self) -> i32 {
    unsafe { (*self.codec_context).height }
  }

  pub fn get_time_base(&self) -> (i32, i32) {
    unsafe {
      (
        (*self.codec_context).time_base.num,
        (*self.codec_context).time_base.den,
      )
    }
  }

  pub fn get_frame_rate(&self) -> (i32, i32) {
    unsafe {
      (
        (*self.codec_context).framerate.num,
        (*self.codec_context).framerate.den,
      )
    }
  }

  pub fn get_aspect_ratio(&self) -> (i32, i32) {
    unsafe {
      (
        (*self.codec_context).sample_aspect_ratio.num,
        (*self.codec_context).sample_aspect_ratio.den,
      )
    }
  }

  pub fn get_pix_fmt_name(&self) -> String {
    unsafe {
      let input_fmt_str = av_get_pix_fmt_name((*self.codec_context).pix_fmt);
      tools::to_string(input_fmt_str)
    }
  }

  pub fn decode(&self, packet: &Packet) -> Result<Frame, String> {
    if packet.get_stream_index() != self.stream_index {
      return Err("bad stream".to_string());
    }
    unsafe {
      check_result!(avcodec_send_packet(self.codec_context, packet.packet));

      let frame = av_frame_alloc();

      check_result!(avcodec_receive_frame(self.codec_context, frame));

      Ok(Frame {
        frame,
        name: Some(self.identifier.clone()),
        index: self.stream_index as usize,
      })
    }
  }
}

impl Drop for VideoDecoder {
  fn drop(&mut self) {
    unsafe {
      if !self.codec_context.is_null() {
        avcodec_close(self.codec_context);
        avcodec_free_context(&mut self.codec_context);
      }
    }
  }
}

unsafe extern "C" fn find_pixel_format(
  ctx: *mut ffmpeg_sys_next::AVCodecContext,
  _: *const ffmpeg_sys_next::AVPixelFormat,
) -> AVPixelFormat {
  get_hw_configs(ctx)[0].1
}

unsafe fn get_hw_configs(
  ctx: *mut ffmpeg_sys_next::AVCodecContext,
) -> Vec<(AVHWDeviceType, AVPixelFormat)> {
  let mut hw_configs = vec![];
  let mut i = 0;
  loop {
    let config = avcodec_get_hw_config((*ctx).codec, i);

    if config.is_null() {
      break;
    }

    if (*config).methods != 0 {
      hw_configs.push(((*config).device_type, (*config).pix_fmt));
    }
    i += 1;
  }
  hw_configs
}
